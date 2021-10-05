use crate::{
    error::{ParseError, Reason},
    expression::{ExprNode, Expression, ExpressionReq, Operator},
    lexer::{Lexer, Token},
    LicenseItem, LicenseReq, ParseMode,
};
use smallvec::SmallVec;

impl Expression {
    /// Given a license expression, attempts to parse and validate it as a valid
    /// SPDX expression. Uses `ParseMode::Strict`.
    ///
    /// The validation can fail for many reasons:
    /// * The expression contains invalid characters
    /// * An unknown/invalid license or exception identifier was found. Only
    /// [SPDX short identifiers](https://spdx.org/ids) are allowed
    /// * The expression contained unbalanced parentheses
    /// * A license or exception immediately follows another license or exception, without
    /// a valid AND, OR, or WITH operator separating them
    /// * An AND, OR, or WITH doesn't have a license or `)` preceding it
    ///
    /// ```
    /// spdx::Expression::parse("MIT OR Apache-2.0 WITH LLVM-exception").unwrap();
    /// ```
    pub fn parse(original: &str) -> Result<Self, ParseError<'_>> {
        Self::parse_mode(original, ParseMode::Strict)
    }

    /// Parses an expression with the specified `ParseMode`. With
    /// `ParseMode::Lax` it permits some non-SPDX syntax, such as imprecise
    /// license names and "/" used instead of "OR" in exprssions.
    ///
    /// ```
    /// spdx::Expression::parse_mode(
    ///     "mit/Apache-2.0 WITH LLVM-exception",
    ///     spdx::ParseMode::Lax
    /// ).unwrap();
    /// ```
    pub fn parse_mode(original: &str, mode: ParseMode) -> Result<Self, ParseError<'_>> {
        let lexer = Lexer::new_mode(original, mode);
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
                Op::Open => unreachable!(),
            };

            q.push(ExprNode::Op(op));
            Ok(())
        };

        let make_err_for_token = |last_token: Option<Token<'_>>, span: std::ops::Range<usize>| {
            let expected: &[&str] = match last_token {
                None | Some(Token::And | Token::Or | Token::OpenParen) => &["<license>", "("],
                Some(Token::CloseParen) => &["AND", "OR"],
                Some(Token::Exception(_)) => &["AND", "OR", ")"],
                Some(Token::Spdx(_)) => &["AND", "OR", "WITH", ")", "+"],
                Some(Token::LicenseRef { .. } | Token::Plus) => &["AND", "OR", "WITH", ")"],
                Some(Token::With) => &["<exception>"],
            };

            Err(ParseError {
                original,
                span,
                reason: Reason::Unexpected(expected),
            })
        };

        // Basic implementation of the https://en.wikipedia.org/wiki/Shunting-yard_algorithm
        'outer: for tok in lexer {
            let lt = tok?;
            match &lt.token {
                Token::Spdx(id) => match last_token {
                    None | Some(Token::And | Token::Or | Token::OpenParen) => {
                        expr_queue.push(ExprNode::Req(ExpressionReq {
                            req: LicenseReq::from(*id),
                            span: lt.span.start as u32..lt.span.end as u32,
                        }));
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::LicenseRef { doc_ref, lic_ref } => match last_token {
                    None | Some(Token::And | Token::Or | Token::OpenParen) => {
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
                    Some(Token::Spdx(_)) => match expr_queue.last_mut().unwrap() {
                        ExprNode::Req(ExpressionReq {
                            req:
                                LicenseReq {
                                    license: LicenseItem::Spdx { or_later, id },
                                    ..
                                },
                            ..
                        }) => {
                            // Handle GNU licenses differently, as they should *NOT* be used with the `+`
                            if mode == ParseMode::Strict && id.is_gnu() {
                                return Err(ParseError {
                                    original,
                                    span: lt.span,
                                    reason: Reason::GnuNoPlus,
                                });
                            }

                            *or_later = true;
                        }
                        _ => unreachable!(),
                    },
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::With => match last_token {
                    Some(Token::Spdx(_) | Token::LicenseRef { .. } | Token::Plus) => {}
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::Or | Token::And => match last_token {
                    Some(
                        Token::Spdx(_)
                        | Token::LicenseRef { .. }
                        | Token::CloseParen
                        | Token::Exception(_)
                        | Token::Plus,
                    ) => {
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
                                            Op::Open => unreachable!(),
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
                    None | Some(Token::And | Token::Or | Token::OpenParen) => {
                        op_stack.push(OpAndSpan {
                            op: Op::Open,
                            span: lt.span,
                        });
                    }
                    _ => return make_err_for_token(last_token, lt.span),
                },
                Token::CloseParen => {
                    match last_token {
                        Some(
                            Token::Spdx(_)
                            | Token::LicenseRef { .. }
                            | Token::Plus
                            | Token::Exception(_)
                            | Token::CloseParen,
                        ) => {
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
            Some(
                Token::Spdx(_)
                | Token::LicenseRef { .. }
                | Token::Exception(_)
                | Token::CloseParen
                | Token::Plus,
            ) => {}
            // We have to have at least one valid license requirement
            None => {
                return Err(ParseError {
                    original,
                    span: 0..original.len(),
                    reason: Reason::Empty,
                });
            }
            Some(_) => return make_err_for_token(last_token, original.len()..original.len()),
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

        // Simplify the expression if possible. There is a limit of 32 unique
        // terms in the simplification algorithm, but if that is hit, someone
        // will need to file a bug, but that would be a pathological case that
        // should never be encountered if there is any hope left in this world
        let simplified = if expr_queue.len() <= 3 {
            expr_queue
        } else {
            println!("BEFORE {:#?}", expr_queue);
            let after = Self::simplify(expr_queue);
            println!("AFTER {:#?}", after);
            after
        };

        Ok(Expression {
            original: original.to_owned(),
            expr: simplified,
        })
    }

    fn simplify(expr: SmallVec<[ExprNode; 5]>) -> SmallVec<[ExprNode; 5]> {
        use quine_mc_cluskey::Bool;

        // We need to keep track of each unique license requirement as a 'term'
        let mut terms = SmallVec::<[&ExpressionReq; 4]>::new();
        let mut bool_stack = Vec::new();

        for node in expr.iter() {
            match node {
                ExprNode::Req(req) => {
                    let term = match terms.iter().position(|r| *r == req) {
                        Some(pos) => pos,
                        None => {
                            terms.push(req);
                            terms.len() - 1
                        }
                    };

                    bool_stack.push(Bool::Term(term as u8));
                }
                ExprNode::Op(Operator::Or) => {
                    let a = bool_stack.pop().unwrap();
                    let b = bool_stack.pop().unwrap();

                    bool_stack.push(Bool::Or(vec![a, b]));
                }
                ExprNode::Op(Operator::And) => {
                    let a = bool_stack.pop().unwrap();
                    let b = bool_stack.pop().unwrap();

                    bool_stack.push(Bool::And(vec![a, b]));
                }
            }
        }

        let root = bool_stack.pop().unwrap();
        println!("BEFORE: {:#?}", root);
        let mut simpled = root.simplify();

        println!("AFTER: {:#?}", simpled);

        let mut simplified = SmallVec::<[ExprNode; 5]>::new();

        fn reform(boo: Bool, terms: &[&ExpressionReq], simplified: &mut SmallVec<[ExprNode; 5]>) {
            match boo {
                Bool::Term(pos) => simplified.push(ExprNode::Req(terms[pos as usize].clone())),
                Bool::And(and) => {
                    for sub in and {
                        reform(sub, terms, simplified);
                    }

                    simplified.push(ExprNode::Op(Operator::And));
                }
                Bool::Or(and) => {
                    for sub in and {
                        reform(sub, terms, simplified);
                    }

                    simplified.push(ExprNode::Op(Operator::Or));
                }
                Bool::True | Bool::False | Bool::Not(_) => unreachable!(),
            }
        }

        while let Some(boo) = simpled.pop() {
            reform(boo, &terms, &mut simplified);
        }

        simplified
    }
}
