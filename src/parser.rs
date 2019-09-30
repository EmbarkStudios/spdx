use crate::{
    error::{ParseError, Reason},
    lexer::{Lexer, Token},
    LicenseItem, LicenseReq,
};
use smallvec::SmallVec;
use std::fmt;

/// A license requirement inside an SPDX license expression, including
/// the span in the expression where it is located
#[derive(Debug, Clone)]
pub struct ExpressionReq {
    pub req: LicenseReq,
    pub span: std::ops::Range<u32>,
}

#[derive(Debug, Clone)]
enum ExprNode {
    Op(Operator),
    Req(ExpressionReq),
}

/// An SPDX license expression that is both syntactically
/// and semantically valid, and can be evaluated
#[derive(Clone)]
pub struct ValidExpression {
    expr: SmallVec<[ExprNode; 5]>,
    // We keep the original string around for display purposes only
    original: String,
}

impl ValidExpression {
    /// Returns each of the license requirements in the license expression,
    /// but not the operators that join them together
    pub fn requirements(&self) -> impl Iterator<Item = &ExpressionReq> {
        self.expr.iter().filter_map(|item| match item {
            ExprNode::Req(req) => Some(req),
            _ => None,
        })
    }

    /// Evaluates the expression, using the provided function
    /// to determine if the licensee meets the requirements
    /// for each license term. If enough requirements are
    /// satisfied the evaluation will return true.
    pub fn evaluate<AF: FnMut(&LicenseReq) -> bool>(&self, mut allow_func: AF) -> bool {
        let mut result_stack = SmallVec::<[bool; 8]>::new();

        // We store the expression as postfix, so just evaluate each license
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Req(req) => {
                    let allowed = allow_func(&req.req);
                    result_stack.push(allowed);
                }
                ExprNode::Op(Operator::Or) => {
                    let a = result_stack.pop().unwrap();
                    let b = result_stack.pop().unwrap();

                    result_stack.push(a || b);
                }
                ExprNode::Op(Operator::And) => {
                    let a = result_stack.pop().unwrap();
                    let b = result_stack.pop().unwrap();

                    result_stack.push(a && b);
                }
            }
        }

        result_stack.pop().unwrap()
    }

    /// Just as with evaluate, the license expression is evaluated to see if
    /// enough license requirements in the expresssion are met for the evaluation
    /// to succeed, except this method also keeps track of each failed requirement
    /// and returns them, allowing for more detailed error reporting about precisely
    /// what terms in the expression caused the overall failure
    pub fn evaluate_with_failures<AF: FnMut(&LicenseReq) -> bool>(&self, mut allow_func: AF) -> Result<(), Vec<&ExpressionReq>> {
        let mut result_stack = SmallVec::<[bool; 8]>::new();
        let mut failures = Vec::new();

        // We store the expression as postfix, so just evaluate each license
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Req(req) => {
                    let allowed = allow_func(&req.req);
                    result_stack.push(allowed);

                    if !allowed {
                        failures.push(req);
                    }
                }
                ExprNode::Op(Operator::Or) => {
                    let a = result_stack.pop().unwrap();
                    let b = result_stack.pop().unwrap();

                    result_stack.push(a || b);
                }
                ExprNode::Op(Operator::And) => {
                    let a = result_stack.pop().unwrap();
                    let b = result_stack.pop().unwrap();

                    result_stack.push(a && b);
                }
            }
        }

        if let Some(false) = result_stack.pop() {
            Err(failures)
        } else {
            Ok(())
        }
    }
}

impl AsRef<str> for ValidExpression {
    fn as_ref(&self) -> &str {
        &self.original
    }
}

impl fmt::Debug for ValidExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.expr.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }

            match node {
                ExprNode::Req(req) => write!(f, "{}", req.req)?,
                ExprNode::Op(Operator::And) => f.write_str("AND")?,
                ExprNode::Op(Operator::Or) => f.write_str("OR")?,
            }
        }

        Ok(())
    }
}

impl<'a> fmt::Display for ValidExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Operator {
    And,
    Or,
}

impl ValidExpression {
    /// Given a license expression, attempts to parse and validate it as a valid SPDX expression
    ///
    /// The validation can fail for many reasons:
    /// * The expression contains invalid characters
    /// * An unknown/invalid license or exception identifier was found. Only
    /// [SPDX short identifiers](https://spdx.org/ids) are allowed
    /// * The expression contained unbalanced parentheses
    /// * A license or exception immediately follows another license or exception, without
    /// a valid AND, OR, or WITH operator separating them
    /// * An AND, OR, or WITH doesn't have a license or `)` preceding it
    pub fn parse(original: &str) -> Result<Self, ParseError> {
        let lexer = Lexer::new(original);

        // Operator precedence in SPDX 2.1
        // +
        // WITH
        // AND
        // OR
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
        enum Op {
            //Plus,
            //With,
            And,
            Or,
            Open,
        }

        struct OpAndSpan {
            op: Op,
            span: std::ops::Range<usize>,
        }

        let mut op_stack = SmallVec::<[OpAndSpan; 3]>::new();
        let mut expr_queue = SmallVec::<[ExprNode; 5]>::new();

        // Keep track of the last token to simplify validation of the token stream
        let mut last_token: Option<Token<'_>> = None;

        let apply_op = |op: OpAndSpan, q: &mut SmallVec<[ExprNode; 5]>| {
            let op = match op.op {
                Op::And => Operator::And,
                Op::Or => Operator::Or,
                _ => unreachable!(),
            };

            q.push(ExprNode::Op(op));
            Ok(())
        };

        let make_err_for_token = |last_token: Option<Token<'_>>, span: std::ops::Range<usize>| {
            let expected: &[&str] = match last_token {
                None | Some(Token::And) | Some(Token::Or) | Some(Token::OpenParen) => {
                    &["<license>", "("]
                }
                Some(Token::CloseParen) => &["AND", "OR"],
                Some(Token::Exception(_)) => &["AND", "OR", ")"],
                Some(Token::SPDX(_)) => &["AND", "OR", "WITH", ")", "+"],
                Some(Token::LicenseRef { .. }) | Some(Token::Plus) => &["AND", "OR", "WITH", ")"],
                Some(Token::With) => &["<exception>"],
            };

            Err(ParseError {
                original,
                span,
                reason: Reason::Unexpected(&expected),
            })
        };

        // Basic implementation of the https://en.wikipedia.org/wiki/Shunting-yard_algorithm
        'outer: for tok in lexer {
            let lt = tok?;
            match &lt.token {
                Token::SPDX(id) => match last_token {
                    None | Some(Token::And) | Some(Token::Or) | Some(Token::OpenParen) => {
                        expr_queue.push(ExprNode::Req(ExpressionReq {
                            req: LicenseReq {
                                license: LicenseItem::SPDX {
                                    id: *id,
                                    or_later: false,
                                },
                                exception: None,
                            },
                            span: lt.span.start as u32..lt.span.end as u32,
                        }));
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::LicenseRef { doc_ref, lic_ref } => match last_token {
                    None | Some(Token::And) | Some(Token::Or) | Some(Token::OpenParen) => {
                        expr_queue.push(ExprNode::Req(ExpressionReq {
                            req: LicenseReq {
                                license: LicenseItem::Other {
                                    doc_ref: doc_ref.map(String::from),
                                    lic_ref: String::from(*lic_ref),
                                },
                                exception: None,
                            },
                            span: lt.span.start as u32..lt.span.end as u32,
                        }));
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::Plus => match last_token {
                    Some(Token::SPDX(_)) => match expr_queue.last_mut().unwrap() {
                        ExprNode::Req(ExpressionReq {
                            req:
                                LicenseReq {
                                    license: LicenseItem::SPDX { or_later, .. },
                                    ..
                                },
                            ..
                        }) => {
                            *or_later = true;
                        }
                        _ => unreachable!(),
                    },
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::With => match last_token {
                    Some(Token::SPDX(_)) | Some(Token::LicenseRef { .. }) | Some(Token::Plus) => {}
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::Or | Token::And => match last_token {
                    Some(Token::SPDX(_))
                    | Some(Token::LicenseRef { .. })
                    | Some(Token::CloseParen)
                    | Some(Token::Exception(_))
                    | Some(Token::Plus) => {
                        let new_op = match lt.token {
                            Token::Or => Op::Or,
                            Token::And => Op::And,
                            _ => unreachable!(),
                        };

                        while let Some(op) = op_stack.last() {
                            match &op.op {
                                Op::Open => break,
                                top => {
                                    if *top < new_op {
                                        let top = op_stack.pop().unwrap();

                                        match top.op {
                                            Op::And | Op::Or => apply_op(top, &mut expr_queue)?,
                                            _ => unreachable!(),
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }

                        op_stack.push(OpAndSpan {
                            op: new_op,
                            span: lt.span,
                        });
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::OpenParen => match last_token {
                    None | Some(Token::And) | Some(Token::Or) | Some(Token::OpenParen) => {
                        op_stack.push(OpAndSpan {
                            op: Op::Open,
                            span: lt.span,
                        });
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::CloseParen => {
                    match last_token {
                        Some(Token::SPDX(_))
                        | Some(Token::LicenseRef { .. })
                        | Some(Token::Plus)
                        | Some(Token::Exception(_))
                        | Some(Token::CloseParen) => {
                            while let Some(top) = op_stack.pop() {
                                match top.op {
                                    Op::And | Op::Or => apply_op(top, &mut expr_queue)?,
                                    Op::Open => {
                                        // This is the only place we go back to the top of the outer loop,
                                        // so make sure we correctly record this token
                                        last_token = Some(Token::CloseParen);
                                        continue 'outer;
                                    }
                                }
                            }

                            // We didn't have an opening parentheses if we get here
                            return Err(ParseError {
                                original,
                                span: lt.span,
                                reason: Reason::UnopenedParens,
                            });
                        }
                        _ => return make_err_for_token(last_token, lt.span),
                    }
                }
                Token::Exception(exc) => match last_token {
                    Some(Token::With) => match expr_queue.last_mut() {
                        Some(ExprNode::Req(lic)) => {
                            lic.req.exception = Some(*exc);
                        }
                        _ => unreachable!(),
                    },
                    _ => return make_err_for_token(last_token, lt.span),
                },
            }

            last_token = Some(lt.token);
        }

        // Validate that the terminating token is valid
        match last_token {
            Some(Token::SPDX(_))
            | Some(Token::LicenseRef { .. })
            | Some(Token::Exception(_))
            | Some(Token::CloseParen)
            | Some(Token::Plus) => {}
            // We have to have at least one valid license requirement
            None => {
                return Err(ParseError {
                    original,
                    span: 0..original.len(),
                    reason: Reason::Empty,
                });
            }
            _ => return make_err_for_token(last_token, original.len()..original.len()),
        }

        while let Some(top) = op_stack.pop() {
            match top.op {
                Op::And | Op::Or => apply_op(top, &mut expr_queue)?,
                Op::Open => {
                    return Err(ParseError {
                        original,
                        span: top.span,
                        reason: Reason::UnclosedParens,
                    });
                }
            }
        }

        // TODO: Investigate using https://github.com/oli-obk/quine-mc_cluskey to simplify
        // expressions, but not really critical. Just cool.

        Ok(ValidExpression {
            original: original.to_owned(),
            expr: expr_queue,
        })
    }
}
