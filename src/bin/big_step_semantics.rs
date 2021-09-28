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
    If {
        condition: Expr,
        consequence: Box<Stmt>,
        alternative: Box<Stmt>,
    },
    Sequence {
        first: Box<Stmt>,
        second: Box<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl Stmt {
    fn evalute(&self, mut env: Environment) -> Environment {
        match self {
            Self::DoNothing => env,
            Self::Assign(name, expr) => {
                env.insert(name.into(), expr.evalute(&env));
                env
            }
            Self::If {
                condition,
                consequence,
                alternative,
            } => match condition.evalute(&env) {
                Expr::Boolean(true) => consequence.evalute(env),
                Expr::Boolean(false) => alternative.evalute(env),
                _ => panic!("invalid condition"),
            },
            Self::Sequence { first, second } => second.evalute(first.evalute(env)),
            Self::While { condition, body } => match condition.evalute(&env) {
                Expr::Boolean(true) => self.evalute(body.evalute(env)),
                Expr::Boolean(false) => env,
                _ => panic!("invalid condition"),
            },
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::DoNothing => write!(f, "do-nothing"),
            Self::Assign(name, expr) => write!(f, "{} = {}", name, expr),
            Self::If {
                condition,
                consequence,
                alternative,
            } => write!(
                f,
                "if ({}) {{ {} }} else {{ {} }}",
                condition, consequence, alternative
            ),
            Self::Sequence { first, second } => write!(f, "{}; {}", first, second),
            Self::While { condition, body } => write!(f, "while ({}) {{ {} }}", condition, body),
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
    fn evalute_add() {
        let expr = Expr::Add(Expr::Number(1).into(), Expr::Number(2).into());
        let env = HashMap::new();
        assert_eq!(Expr::Number(3), expr.evalute(&env));
    }

    #[test]
    fn evalute_multiply() {
        let expr = Expr::Multiply(Expr::Number(2).into(), Expr::Number(3).into());
        let env = HashMap::new();
        assert_eq!(Expr::Number(6), expr.evalute(&env));
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
        assert_eq!(env.clone(), stmt.evalute(env));
    }

    #[test]
    fn evalute_assign() {
        let stmt = Stmt::Assign("x".into(), Expr::Number(1));
        let mut env = HashMap::new();
        env.insert("y".into(), Expr::Number(2));
        let mut expected = env.clone();
        expected.insert("x".into(), Expr::Number(1));
        assert_eq!(expected, stmt.evalute(env));
    }

    #[test]
    fn evalute_if() {
        let stmt = Stmt::If {
            condition: Expr::LessThan(Expr::Variable("x".into()).into(), Expr::Number(3).into()),
            consequence: Stmt::Assign(
                "y".into(),
                Expr::Multiply(Expr::Variable("x".into()).into(), Expr::Number(2).into()).into(),
            )
            .into(),
            alternative: Stmt::DoNothing.into(),
        };
        let mut env = HashMap::new();
        env.insert("x".into(), Expr::Number(2));
        let mut expected = env.clone();
        expected.insert("y".into(), Expr::Number(4));
        assert_eq!(expected, stmt.evalute(env));
    }

    #[test]
    fn evalute_sequence() {
        let stmt = Stmt::Sequence {
            first: Stmt::Assign("x".into(), Expr::Number(2).into()).into(),
            second: Stmt::Assign(
                "y".into(),
                Expr::Multiply(Expr::Variable("x".into()).into(), Expr::Number(2).into()).into(),
            )
            .into(),
        };
        let env = HashMap::new();
        let mut expected = env.clone();
        expected.insert("x".into(), Expr::Number(2));
        expected.insert("y".into(), Expr::Number(4));
        assert_eq!(expected, stmt.evalute(env));
    }

    #[test]
    fn evalute_while() {
        let stmt = Stmt::While {
            condition: Expr::LessThan(Expr::Variable("x".into()).into(), Expr::Number(5).into())
                .into(),
            body: Stmt::Assign(
                "x".into(),
                Expr::Multiply(Expr::Variable("x".into()).into(), Expr::Number(3).into()).into(),
            )
            .into(),
        };
        let mut env = HashMap::new();
        env.insert("x".into(), Expr::Number(1));

        let mut expected = env.clone();
        expected.insert("x".into(), Expr::Number(9));
        assert_eq!(expected, stmt.evalute(env));
    }
}
