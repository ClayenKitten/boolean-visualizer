use std::fmt::Display;

use thiserror::Error;

use super::Function;

impl Function {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        use ParseErrorKind::*;

        let mut variables = Vec::with_capacity(3);
        let mut infix = Vec::with_capacity(s.len());

        let mut bracket_number = 0isize;
        let mut previous = TokenSeqType::Operator;
        for (pos, ch) in s.char_indices() {
            if ch.is_whitespace() {
                continue;
            }
            let token = match ch {
                '&' => InfixToken::And,
                '|' => InfixToken::Or,
                '!' => InfixToken::Not,
                var @ ('a'..='z') => {
                    if !variables.contains(&var) {
                        variables.push(var);
                    }
                    InfixToken::Variable(var)
                }
                '0' => InfixToken::Const(false),
                '1' => InfixToken::Const(true),
                '(' => {
                    bracket_number += 1;
                    InfixToken::LeftBracket
                }
                ')' => {
                    bracket_number -= 1;
                    if bracket_number < 0 {
                        return Err(UnmatchedParenthesis.at(pos));
                    }
                    InfixToken::RightBracket
                }
                ch => return Err(IllegalCharacter(ch).at(pos)),
            };
            previous.matches(token).map_err(|e| e.at(pos))?;
            previous = token.into();
            infix.push(token);
        }
        if bracket_number > 0 {
            return Err(UnclosedParenthesis.at(s.len()));
        }
        if previous == TokenSeqType::Operator {
            return Err(UnexpectedEOF.at(s.len()));
        }
        let postfix = Self::into_postfix(infix.into_iter());

        variables.sort_unstable();
        Ok(Function { variables, postfix })
    }

    /// Translates infix notation into postfix notation.
    fn into_postfix(infix: impl Iterator<Item = InfixToken>) -> Vec<PostfixToken> {
        let mut op_stack = Vec::<OpStackEntry>::new();
        let mut output = Vec::<PostfixToken>::new();
        for token in infix {
            match token {
                InfixToken::Not => op_stack.push(OpStackEntry::Not),
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
                        output.push(match op_stack.pop().unwrap() {
                            OpStackEntry::And => PostfixToken::And,
                            OpStackEntry::Or => PostfixToken::Or,
                            OpStackEntry::Not => PostfixToken::Not,
                            OpStackEntry::LeftBracket => unreachable!(),
                        });
                    }
                    op_stack.push(match op {
                        InfixToken::And => OpStackEntry::And,
                        InfixToken::Or => OpStackEntry::Or,
                        InfixToken::Not => OpStackEntry::Not,
                        _ => unreachable!(),
                    });
                }
                InfixToken::LeftBracket => op_stack.push(OpStackEntry::LeftBracket),
                InfixToken::RightBracket => loop {
                    match op_stack.last() {
                        Some(OpStackEntry::LeftBracket) => {
                            op_stack.pop();
                            break;
                        }
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
                        }
                    }
                },
                InfixToken::Variable(var) => output.push(PostfixToken::Var(var)),
                InfixToken::Const(val) => output.push(PostfixToken::Const(val)),
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub struct ParseError {
    pos: usize,
    kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ParseErrorKind {
    #[error("character `{0}` is not allowed")]
    IllegalCharacter(char),
    #[error("more brackets closed than opened")]
    UnmatchedParenthesis,
    #[error("more brackets opened than closed")]
    UnclosedParenthesis,
    #[error("expected one of: '&', '|', or ')'; got '{0}'")]
    ExpectedOperator(char),
    #[error("expected one of: variable, constant, '!', or '('; got '{0}'")]
    ExpectedOperand(char),
    #[error("expected one of: variable, constant, '!', or '('; got EOF")]
    UnexpectedEOF,
}

impl ParseErrorKind {
    pub fn at(self, pos: usize) -> ParseError {
        ParseError { pos, kind: self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenSeqType {
    /// A token that should be followed by '&', '|', or ')'.
    Operand,
    /// A token that should be followed by operand, '!', or '('.
    Operator,
}

impl TokenSeqType {
    pub fn matches(&self, next: InfixToken) -> Result<(), ParseErrorKind> {
        use {InfixToken::*, TokenSeqType::*};

        match self {
            Operand => match next {
                And | Or | RightBracket => Ok(()),
                _ => Err(ParseErrorKind::ExpectedOperator(next.into())),
            },
            Operator => match next {
                Variable(_) | Const(_) | Not | LeftBracket => Ok(()),
                _ => Err(ParseErrorKind::ExpectedOperand(next.into())),
            },
        }
    }
}

impl From<InfixToken> for TokenSeqType {
    fn from(value: InfixToken) -> Self {
        use {InfixToken::*, TokenSeqType::*};

        match value {
            And => Operator,
            Or => Operator,
            Not => Operator,
            Variable(_) => Operand,
            Const(_) => Operand,
            LeftBracket => Operator,
            RightBracket => Operand,
        }
    }
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

impl Into<char> for InfixToken {
    fn into(self) -> char {
        match self {
            InfixToken::And => '&',
            InfixToken::Or => '|',
            InfixToken::Not => '!',
            InfixToken::Variable(ch) => ch,
            InfixToken::Const(true) => '1',
            InfixToken::Const(false) => '0',
            InfixToken::LeftBracket => '(',
            InfixToken::RightBracket => ')',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostfixToken {
    And,
    Or,
    Not,
    Var(char),
    Const(bool),
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
    use crate::function::{parse::PostfixToken, Function};

    #[test]
    fn parse_one() {
        let parsed = Function::parse("1");
        let expected = Function {
            variables: Vec::new(),
            postfix: vec![PostfixToken::Const(true)],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_not() {
        let parsed = Function::parse("!1");
        let expected = Function {
            variables: Vec::new(),
            postfix: vec![PostfixToken::Const(true), PostfixToken::Not],
        };
        assert_eq!(Ok(expected), parsed);
    }

    #[test]
    fn parse_chained_and() {
        let parsed = Function::parse("x & 1 & y");
        let expected = Function {
            variables: vec!['x', 'y'],
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
            variables: vec!['x', 'y'],
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
            variables: vec!['x', 'y'],
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
            variables: vec!['x', 'y'],
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
            variables: vec!['x', 'y', 'z'],
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
            ],
        };
        assert_eq!(Ok(expected), parsed);
    }
}
