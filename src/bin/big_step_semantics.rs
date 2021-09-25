use std::collections::HashMap;
use std::fmt::{self};

type Environment = HashMap<String, Expr>;

#[derive(PartialEq, Eq, Clone)]
enum Expr {
    Number(i64),
    Boolean(bool),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn evalute(&self, env: &Environment) -> Self {
        match self {
            Self::Number(_) => self.clone(),
            Self::Boolean(_) => self.clone(),
            Self::Variable(name) => env[name].clone(),
            Self::Add(l, r) => match (l.evalute(env), r.evalute(env)) {
                (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
                _ => panic!("invalid expr"),
            },
            Self::Multiply(l, r) => match (l.evalute(env), r.evalute(env)) {
                (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
                _ => panic!("invalid expr"),
            },
            Self::LessThan(l, r) => match (l.evalute(env), r.evalute(env)) {
                (Self::Number(a), Self::Number(b)) => Self::Boolean(a < b),
                _ => panic!("invalid expr"),
            },
            _ => panic!("`evalute()` not supported"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Add(l, r) => write!(f, "{} + {}", l, r),
            Self::Multiply(l, r) => write!(f, "{} * {}", l, r),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::LessThan(l, r) => write!(f, "{} < {}", l, r),
            Self::Variable(name) => write!(f, "{}", name),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "<<{}>>", self)
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Stmt {
    DoNothing,
    Assign(String, Expr),
}

impl Stmt {
    fn evalute(&self, env: &Environment) -> Environment {
        match self {
            Self::DoNothing => env.clone(),
            Self::Assign(name, expr) => {
                let mut new_env = env.clone();
                new_env.insert(name.into(), expr.clone());
                new_env
            }
            _ => panic!("`evalute()` not supported"),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::DoNothing => write!(f, "do-nothing"),
            Self::Assign(name, expr) => write!(f, "{} = {}", name, expr),
        }
    }
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "<<{}>>", self)
    }
}

fn main() {
    let env = HashMap::new();
    println!("{}", Expr::Number(1).evalute(&env));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evalute_number() {
        let n = Expr::Number(23);
        let env = HashMap::new();
        assert_eq!(n, n.evalute(&env));
    }

    #[test]
    fn evalute_boolean() {
        let t = Expr::Boolean(true);
        let mut env = HashMap::new();
        assert_eq!(t, t.evalute(&mut env));

        let f = Expr::Boolean(false);
        let env = HashMap::new();
        assert_eq!(f, f.evalute(&env));
    }

    #[test]
    fn evalute_less_than() {
        let expr = Expr::LessThan(
            Expr::Add(Expr::Variable("x".into()).into(), Expr::Number(2).into()).into(),
            Expr::Variable("y".into()).into(),
        );
        let mut env = HashMap::new();
        env.insert("x".into(), Expr::Number(2));
        env.insert("y".into(), Expr::Number(5));
        assert_eq!(Expr::Boolean(true), expr.evalute(&env));
    }

    #[test]
    fn evalute_donothing() {
        let stmt = Stmt::DoNothing;
        let mut env = HashMap::new();
        env.insert("x".into(), Expr::Number(2));
        assert_eq!(env.clone(), stmt.evalute(&env));
    }

    #[test]
    fn evalute_assign() {
        let stmt = Stmt::Assign("x".into(), Expr::Number(1));
        let mut env = HashMap::new();
        env.insert("y".into(), Expr::Number(2));
        let mut expected = env.clone();
        expected.insert("x".into(), Expr::Number(1));
        assert_eq!(expected, stmt.evalute(&env));
    }
}
