mod parse;

use std::collections::{HashSet, HashMap};

use self::parse::PostfixToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    variables: HashSet<char>,
    postfix: Vec<PostfixToken>,
}

impl Function {
    pub fn vars(&self) -> &HashSet<char> {
        &self.variables
    }

    pub fn eval(&self, vars: HashMap<char, bool>) -> Option<bool> {
        let mut stack = Vec::<bool>::with_capacity(16);
        for token in self.postfix.iter() {
            let val = match token {
                PostfixToken::And => stack.pop().unwrap() & stack.pop().unwrap(),
                PostfixToken::Or => stack.pop().unwrap() | stack.pop().unwrap(),
                PostfixToken::Not => !stack.pop().unwrap(),
                PostfixToken::Const(val) => *val,
                PostfixToken::Var(ch) => *vars.get(ch)?,
            };
            stack.push(val);
        }
        stack.pop()
    }
}

#[cfg(test)]
mod eval_tests {
    use std::collections::HashMap;

    use super::Function;

    #[test]
    fn eval_const() {
        assert_eq!(
            Some(false),
            Function::parse("0")
                .unwrap()
                .eval(HashMap::new())
        );

        assert_eq!(
            Some(true),
            Function::parse("1")
                .unwrap()
                .eval(HashMap::new())
        );
    }

    #[test]
    fn eval_var() {
        assert_eq!(
            Some(true),
            Function::parse("x")
                .unwrap()
                .eval(HashMap::from([('x', true)]))
        );

        assert_eq!(
            Some(false),
            Function::parse("x")
                .unwrap()
                .eval(HashMap::from([('x', false)]))
        );
    }

    #[test]
    fn eval_and() {
        let func = Function::parse("x & y").unwrap();
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', false)])));
        assert_eq!(Some(false), func.eval(HashMap::from([('x', true), ('y', false)])));
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', true)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', true)])));
    }

    #[test]
    fn eval_or() {
        let func = Function::parse("x | y").unwrap();
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', false), ('y', true)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', true)])));
    }

    #[test]
    fn eval_not() {
        let func = Function::parse("!x").unwrap();
        assert_eq!(Some(true), func.eval(HashMap::from([('x', false)])));
        assert_eq!(Some(false), func.eval(HashMap::from([('x', true)])));
    }

    #[test]
    fn eval_chained_and() {
        let func = Function::parse("x & y & z").unwrap();
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', false), ('z', false)])));
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', false), ('z', true)])));
        assert_eq!(Some(false), func.eval(HashMap::from([('x', true), ('y', false), ('z', true)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', true), ('z', true)])));
    }

    #[test]
    fn eval_chained_or() {
        let func = Function::parse("x | y | z").unwrap();
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', false), ('z', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', false), ('y', false), ('z', true)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', false), ('z', true)])));
    }

    #[test]
    fn eval_chained_not() {
        let func = Function::parse("!!x").unwrap();
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true)])));
    }

    #[test]
    fn eval_idempotent() {
        let func = Function::parse("x|y&y").unwrap();
        dbg!(&func);
        assert_eq!(Some(false), func.eval(HashMap::from([('x', false), ('y', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', false), ('y', true)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', true)])));
    }

    #[test]
    fn eval_complex() {
        let func = Function::parse("!x & (y | z) | !z").unwrap();
        assert_eq!(Some(true), func.eval(HashMap::from([('x', false), ('y', false), ('z', false)])));
        assert_eq!(Some(true), func.eval(HashMap::from([('x', true), ('y', false), ('z', false)])));
        assert_eq!(Some(false), func.eval(HashMap::from([('x', true), ('y', false), ('z', true)])));
    }
}
