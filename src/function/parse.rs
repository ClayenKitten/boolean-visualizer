use std::collections::HashSet;

use thiserror::Error;

use super::Function;

impl Function {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let mut err = Ok(());
        let mut variables = HashSet::new();
        let infix = s
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(|ch| Ok(match ch {
                '&' => InfixToken::And,
                '|' => InfixToken::Or,
                '!' => InfixToken::Not,
                var @ ('x' | 'y' | 'z') => {
                    variables.insert(var);
                    InfixToken::Variable(var)
                },
                '0' => InfixToken::Const(false),
                '1' => InfixToken::Const(true),
                '(' => InfixToken::LeftBracket,
                ')' => InfixToken::RightBracket,
                ch => return Err(ParseError::IllegalCharacter(ch)),
            }))
            .scan(&mut err, |err, res| match res {
                Ok(o) => Some(o),
                Err(e) => {
                    **err = Err(e);
                    None
                }
            });
        let postfix = into_postfix(infix);
        Ok(
            Function {
                variables,
                postfix,
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("character {0} is not allowed")]
    IllegalCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InfixToken {
    And,
    Or,
    Not,
    Variable(char),
    Const(bool),
    LeftBracket,
    RightBracket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixToken {
    And,
    Or,
    Not,
    Var(char),
    Const(bool),
}

/// Translates infix notation into postfix notation.
fn into_postfix(infix: impl Iterator<Item = InfixToken>) -> Vec<PostfixToken> {
    let mut op_stack = Vec::<OpStackEntry>::new();
    let mut output = Vec::<PostfixToken>::new();
    for token in infix {
        match token {
            InfixToken::Not
                => op_stack.push(OpStackEntry::Not),
            op @ (InfixToken::And | InfixToken::Or) => {
                loop {
                    let top_priority = match op_stack.last() {
                        Some(OpStackEntry::Not) => 2,
                        Some(OpStackEntry::And) => 1,
                        Some(OpStackEntry::Or) => 0,
                        None | Some(OpStackEntry::LeftBracket) => break,
                    };
                    let cur_priority = match op {
                        InfixToken::And => 1,
                        InfixToken::Or => 0,
                        _ => unreachable!(),
                    };
                    if top_priority < cur_priority {
                        break;
                    }
                    output.push(
                        match op_stack.pop().unwrap() {
                            OpStackEntry::And => PostfixToken::And,
                            OpStackEntry::Or => PostfixToken::Or,
                            OpStackEntry::Not => PostfixToken::Not,
                            OpStackEntry::LeftBracket => unreachable!(),
                        }
                    );
                }
                op_stack.push(
                    match op {
                        InfixToken::And => OpStackEntry::And,
                        InfixToken::Or => OpStackEntry::Or,
                        InfixToken::Not => OpStackEntry::Not,
                        _ => unreachable!(),
                    }
                );
            }
            InfixToken::LeftBracket
                => op_stack.push(OpStackEntry::LeftBracket),
            InfixToken::RightBracket => {
                loop {
                    match op_stack.last() {
                        Some(OpStackEntry::LeftBracket) => {
                            op_stack.pop();
                            break;
                        },
                        Some(&token) => {
                            op_stack.pop();
                            let token = match token {
                                OpStackEntry::And => PostfixToken::And,
                                OpStackEntry::Or => PostfixToken::Or,
                                OpStackEntry::Not => PostfixToken::Not,
                                OpStackEntry::LeftBracket => unreachable!(),
                            };
                            output.push(token);
                        }
                        None => {
                            panic!("No right bracket");
                        },
                    }
                }
            },
            InfixToken::Variable(var)
                => output.push(PostfixToken::Var(var)),
            InfixToken::Const(val)
                => output.push(PostfixToken::Const(val)),
        }
    }

    output.reserve_exact(op_stack.len());
    while let Some(token) = op_stack.pop() {
        output.push(match token {
            OpStackEntry::LeftBracket => panic!("No left bracket"),
            OpStackEntry::And => PostfixToken::And,
            OpStackEntry::Or => PostfixToken::Or,
            OpStackEntry::Not => PostfixToken::Not,
        });
    }
    output
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpStackEntry {
    LeftBracket,
    And,
    Or,
    Not,
}

#[cfg(test)]
mod parse_tests {
    use std::collections::HashSet;

    use crate::function::{Function, parse::PostfixToken};

    #[test]
    fn parse_one() {
        let parsed = Function::parse("1");
        let expected = Function {
            variables: HashSet::new(),
            postfix: vec![PostfixToken::Const(true)],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_not() {
        let parsed = Function::parse("!1");
        let expected = Function {
            variables: HashSet::new(),
            postfix: vec![PostfixToken::Const(true), PostfixToken::Not],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_chained_and() {
        let parsed = Function::parse("x & 1 & y");
        let expected = Function {
            variables: HashSet::from(['x', 'y']),
            postfix: vec![
                PostfixToken::Var('x'),
                PostfixToken::Const(true),
                PostfixToken::And,
                PostfixToken::Var('y'),
                PostfixToken::And,
            ],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_chained_or() {
        let parsed = Function::parse("x | y | 0");
        let expected = Function {
            variables: HashSet::from(['x', 'y']),
            postfix: vec![
                PostfixToken::Var('x'),
                PostfixToken::Var('y'),
                PostfixToken::Or,
                PostfixToken::Const(false),
                PostfixToken::Or,
            ],
        };
        assert_eq!(Ok(expected), parsed);        
    }

    #[test]
    fn parse_combined() {
        let parsed = Function::parse("1 & x | y");
        let expected = Function {
            variables: HashSet::from(['x', 'y']),
            postfix: vec![
                PostfixToken::Const(true),
                PostfixToken::Var('x'),
                PostfixToken::And,
                PostfixToken::Var('y'),
                PostfixToken::Or,
            ],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_parenthesis() {
        let parsed = Function::parse("1 & (x | y)");
        let expected = Function {
            variables: HashSet::from(['x', 'y']),
            postfix: vec![
                PostfixToken::Const(true),
                PostfixToken::Var('x'),
                PostfixToken::Var('y'),
                PostfixToken::Or,
                PostfixToken::And,
            ],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_complex() {
        let parsed = Function::parse("!x & (y | z) | !z");
        let expected = Function {
            variables: HashSet::from(['x', 'y', 'z']),
            postfix: vec![
                PostfixToken::Var('x'),
                PostfixToken::Not,
                PostfixToken::Var('y'),
                PostfixToken::Var('z'),
                PostfixToken::Or,
                PostfixToken::And,
                PostfixToken::Var('z'),
                PostfixToken::Not,
                PostfixToken::Or,
            ]
        };
        assert_eq!(Ok(expected), parsed);
    }
}
