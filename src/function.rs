mod parse;

use self::parse::PostfixToken;

pub use parse::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    variables: Vec<char>,
    postfix: Vec<PostfixToken>,
}

impl Function {
    pub fn vars(&self) -> &[char] {
        self.variables.as_slice()
    }

    fn var_index(&self, var: char) -> Option<usize> {
        self.variables.iter().position(|ch| *ch == var)
    }

    pub fn eval(&self, vars: &[bool]) -> Option<bool> {
        let mut stack = Vec::<bool>::with_capacity(16);
        for token in self.postfix.iter() {
            let val = match token {
                PostfixToken::And => stack.pop().unwrap() & stack.pop().unwrap(),
                PostfixToken::Or => stack.pop().unwrap() | stack.pop().unwrap(),
                PostfixToken::Not => !stack.pop().unwrap(),
                PostfixToken::Const(val) => *val,
                PostfixToken::Var(ch) => vars[self.var_index(*ch)?],
            };
            stack.push(val);
        }
        stack.pop()
    }
}

#[cfg(test)]
mod eval_tests {
    use super::Function;

    #[test]
    fn eval_const() {
        assert_eq!(
            Some(false),
            Function::parse("0").unwrap().eval(&[])
        );

        assert_eq!(
            Some(true),
            Function::parse("1").unwrap().eval(&[])
        );
    }

    #[test]
    fn eval_var() {
        assert_eq!(
            Some(true),
            Function::parse("x")
                .unwrap()
                .eval(&[true])
        );

        assert_eq!(
            Some(false),
            Function::parse("x")
                .unwrap()
                .eval(&[false])
        );
    }

    #[test]
    fn eval_and() {
        let func = Function::parse("x & y").unwrap();
        assert_eq!(
            Some(false),
            func.eval(&[false, false])
        );
        assert_eq!(
            Some(false),
            func.eval(&[true, false])
        );
        assert_eq!(
            Some(false),
            func.eval(&[false, true])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, true])
        );
    }

    #[test]
    fn eval_or() {
        let func = Function::parse("x | y").unwrap();
        assert_eq!(
            Some(false),
            func.eval(&[false, false])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, false])
        );
        assert_eq!(
            Some(true),
            func.eval(&[false, true])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, true])
        );
    }

    #[test]
    fn eval_not() {
        let func = Function::parse("!x").unwrap();
        assert_eq!(Some(true), func.eval(&[false]));
        assert_eq!(Some(false), func.eval(&[true]));
    }

    #[test]
    fn eval_chained_and() {
        let func = Function::parse("x & y & z").unwrap();
        assert_eq!(
            Some(false),
            func.eval(&[false, false, false])
        );
        assert_eq!(
            Some(false),
            func.eval(&[false, false, true])
        );
        assert_eq!(
            Some(false),
            func.eval(&[true, false, true])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, true, true])
        );
    }

    #[test]
    fn eval_chained_or() {
        let func = Function::parse("x | y | z").unwrap();
        assert_eq!(
            Some(false),
            func.eval(&[false, false, false])
        );
        assert_eq!(
            Some(true),
            func.eval(&[false, false, true])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, false, true])
        );
    }

    #[test]
    fn eval_chained_not() {
        let func = Function::parse("!!x").unwrap();
        assert_eq!(Some(false), func.eval(&[false]));
        assert_eq!(Some(true), func.eval(&[true]));
    }

    #[test]
    fn eval_idempotent() {
        let func = Function::parse("x|y&y").unwrap();
        dbg!(&func);
        assert_eq!(
            Some(false),
            func.eval(&[false, false])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, false])
        );
        assert_eq!(
            Some(true),
            func.eval(&[false, true])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, true])
        );
    }

    #[test]
    fn eval_complex() {
        let func = Function::parse("!x & (y | z) | !z").unwrap();
        assert_eq!(
            Some(true),
            func.eval(&[false, false, false])
        );
        assert_eq!(
            Some(true),
            func.eval(&[true, false, false])
        );
        assert_eq!(
            Some(false),
            func.eval(&[true, false, true])
        );
    }
}
