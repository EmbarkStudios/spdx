use crate::{
    exception_id,
    lexer::{Lexer, Token},
    license_id, LicenseItem, LicenseReq, ParseError,
};
use smallvec::SmallVec;
use std::fmt;

#[derive(Debug)]
enum ExprNode<'a> {
    Op(Operator),
    Req(LicenseReq<'a>),
}

/// An SPDX license expression that is both syntactically
/// and semantically valid, and can be evaluated
pub struct ValidExpression<'a> {
    expr: SmallVec<[ExprNode<'a>; 3]>,
    // We keep the original string around for display purposes only
    original: &'a str,
}

impl<'a> ValidExpression<'a> {
    pub fn licenses(&self) -> impl Iterator<Item = &LicenseReq<'a>> {
        self.expr.iter().filter_map(|item| match item {
            ExprNode::Req(req) => Some(req),
            _ => None,
        })
    }

    /// Evaluates the expression, using the provided function
    /// to determine if the licensee meets the requirements
    /// for each license term. If enough requirements are
    /// satisfied
    pub fn evaluate<AF: Fn(&LicenseReq<'a>) -> bool>(
        &self,
        allow_func: AF,
    ) -> Result<(), &LicenseReq<'a>> {
        let mut failed = None;
        let mut result_stack = SmallVec::<[bool; 8]>::new();

        // We store the expression as postfix, so just evaluate each license
        // requirement in the order it comes, and then combining the previous
        // results according to each operator as it comes
        for node in self.expr.iter() {
            match node {
                ExprNode::Req(req) => {
                    let allowed = allow_func(req);
                    result_stack.push(allowed);

                    if !allowed {
                        failed = Some(req);
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

        if result_stack.pop() == Some(false) {
            Err(failed.unwrap())
        } else {
            Ok(())
        }
    }
}

impl<'a> fmt::Debug for ValidExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.expr.iter().enumerate() {
            if i > 0 {
                f.write_str(" ")?;
            }

            match node {
                ExprNode::Req(req) => write!(f, "{}", req)?,
                ExprNode::Op(Operator::And) => f.write_str("AND")?,
                ExprNode::Op(Operator::Or) => f.write_str("OR")?,
            }
        }

        Ok(())
    }
}

impl<'a> fmt::Display for ValidExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.original)
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
}

impl<'a> ValidExpression<'a> {
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
    pub fn parse(expr: &'a str) -> Result<Self, ParseError> {
        let lexer = Lexer::new(expr);

        // Operator precedence as of SPDX 2.1
        // +
        // WITH
        // AND
        // OR
        #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
        enum Op {
            With,
            And,
            Or,
            Open(usize),
        }

        let mut op_stack = SmallVec::<[Op; 3]>::new();
        let mut expr_queue = SmallVec::<[ExprNode; 3]>::new();

        'outer: for tok in lexer {
            let lex_tok = tok?;
            let cur_str = &expr[lex_tok.start..lex_tok.end];

            let license_item = match lex_tok.token {
                Token::License(lic) => LicenseItem::SPDX(
                    license_id(lic).ok_or_else(|| ParseError::UnknownLicenseId(lic))?,
                ),
                Token::LicenseRef { doc, lic } => LicenseItem::Other {
                    document_ref: doc,
                    license_ref: lic,
                },
                Token::Plus => {
                    // Anything other than an open parens at the top of the op stack means
                    // the plus is following another operator, which isn't allowed
                    match op_stack.last() {
                        Some(Op::Open(_)) | None => {}
                        _ => return Err(ParseError::UnexpectedToken(cur_str, &["<license>"])),
                    }

                    match expr_queue.last_mut() {
                        Some(ExprNode::Req(lic)) => {
                            // This isn't C++, the + is only an unary suffix
                            if lic.or_later {
                                return Err(ParseError::UnexpectedToken(
                                    cur_str,
                                    &["AND", "OR", "WITH"],
                                ));
                            } else {
                                lic.or_later = true;
                            }
                        }
                        _ => return Err(ParseError::UnexpectedToken(cur_str, &["<license>"])),
                    }

                    continue;
                }
                op_tok @ Token::Or | op_tok @ Token::And => {
                    // while ((there is a function at the top of the operator stack)
                    //     or (there is an operator at the top of the operator stack with greater precedence)
                    //     or (the operator at the top of the operator stack has equal precedence and is left associative))
                    //     and (the operator at the top of the operator stack is not a left parenthesis):
                    //     pop operators from the operator stack onto the output queue.
                    // push it onto the operator stack.

                    let new_op = match op_tok {
                        Token::Or => Op::Or,
                        Token::And => Op::And,
                        _ => unreachable!(),
                    };

                    while let Some(op) = op_stack.last() {
                        match op {
                            Op::Open(_) => break,
                            other => {
                                if *other < new_op {
                                    match op_stack.pop().unwrap() {
                                        Op::And => expr_queue.push(ExprNode::Op(Operator::And)),
                                        Op::Or => expr_queue.push(ExprNode::Op(Operator::Or)),
                                        _other => {
                                            return Err(ParseError::UnexpectedToken(
                                                cur_str,
                                                &["<license>"],
                                            ))
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    // match op_stack.last() {
                    //     Some(Op::Or) => {
                    //         if new_op == Op::And {
                    //             //println!("TOP IS {:?} @ INDEX {}", top, op_stack.len() - 1);
                    //             op_stack.insert(op_stack.len() - 2, new_op);
                    //             continue;
                    //         }
                    //     }
                    //     _ => {}
                    // }

                    op_stack.push(new_op);
                    continue;
                }
                Token::With => {
                    // WITH must directly follow a license
                    // match op_stack.last() {
                    //     Some(Op::Open(_)) | None => {},
                    //     _ => return Err(ParseError::UnexpectedToken(cur_str, &["<license>"])),
                    // }

                    match expr_queue.last() {
                        Some(ExprNode::Req(lr)) => {
                            if lr.exception.is_some() {
                                return Err(ParseError::UnexpectedToken(cur_str, &["AND", "OR"]));
                            }
                        }
                        _ => return Err(ParseError::UnexpectedToken(cur_str, &["<license>"])),
                    }

                    op_stack.push(Op::With);
                    continue;
                }
                Token::OpenParen => {
                    op_stack.push(Op::Open(lex_tok.start));
                    continue;
                }
                Token::CloseParen => {
                    while let Some(op) = op_stack.pop() {
                        match op {
                            Op::Or => expr_queue.push(ExprNode::Op(Operator::Or)),
                            Op::And => expr_queue.push(ExprNode::Op(Operator::And)),
                            Op::Open(_) => continue 'outer,
                            Op::With => {
                                return Err(ParseError::UnexpectedToken(cur_str, &["<exception>"]));
                            }
                        }
                    }

                    // We didn't have an opening parentheses if we get here
                    return Err(ParseError::UnbalancedParen(lex_tok.start));
                }
                Token::Exception(exc) => {
                    // Exceptions can only follow a WITH, and a WITH can only
                    // follow a license, so we just pop the WITH op off and
                    // directly modify the preceding license
                    match op_stack.last() {
                        Some(Op::With) => {
                            op_stack.pop();

                            let exc_id = exception_id(exc)
                                .ok_or_else(|| ParseError::UnknownExceptionId(exc))?;

                            match expr_queue.last_mut() {
                                Some(ExprNode::Req(lr)) => {
                                    lr.exception = Some(exc_id);
                                }
                                _ => {
                                    return Err(ParseError::UnexpectedToken(
                                        cur_str,
                                        &["<license>"],
                                    ))
                                }
                            }
                        }
                        _ => return Err(ParseError::UnexpectedToken(cur_str, &["<license>"])),
                    }

                    continue;
                }
            };

            // Ensure we have a valid preceding token
            if let Some(Op::With) = op_stack.last() {
                return Err(ParseError::UnexpectedToken(cur_str, &["<exception>"]));
            }

            expr_queue.push(ExprNode::Req(LicenseReq {
                license: license_item,
                exception: None,
                or_later: false,
            }));
        }

        while let Some(op) = op_stack.pop() {
            match op {
                Op::And => expr_queue.push(ExprNode::Op(Operator::And)),
                Op::Or => expr_queue.push(ExprNode::Op(Operator::Or)),
                Op::With => return Err(ParseError::UnexpectedToken("WITH", &["<exception>"])),
                Op::Open(ind) => return Err(ParseError::UnbalancedParen(ind)),
            }
        }

        // We have to have at least one valid license requirement
        if !expr_queue.iter().any(|i| match i {
            ExprNode::Req(_) => true,
            _ => false,
        }) {
            return Err(ParseError::Empty);
        }

        // let trailing_parens = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 AND (OpenSSL OR MIT)";
        // Or => [Apache with LLVM, And => [Apache-2.0, Or => [OpenSSL, MIT]]]

        // let leading_parens = "((Apache-2.0 WITH LLVM-exception) OR Apache-2.0) AND OpenSSL OR MIT";
        // Or => [And => [Or => [Apache with LLVM, Apache-2.0], OpenSSL], MIT]

        // let trailing_parens = "Apache-2.0 WITH LLVM-exception OR (Apache-2.0 AND OpenSSL OR MIT)";
        // Or => [Apache with LLVM, And => [Apache-2.0, Or => [OpenSSL, MIT]]]

        Ok(ValidExpression {
            expr: expr_queue,
            original: expr,
        })
    }
}
