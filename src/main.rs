use std::collections::HashMap;
use std::fmt::{self};

type Environment = HashMap<String, Expr>;

#[derive(PartialEq, Eq, Clone)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Boolean(bool),
    LessThan(Box<Expr>, Box<Expr>),
    Variable(String),
}

impl Expr {
    fn is_reducible(&self) -> bool {
        match self {
            Self::Number(_) => false,
            Self::Add(_, _) => true,
            Self::Multiply(_, _) => true,
            Self::Boolean(_) => false,
            Self::LessThan(_, _) => true,
            Self::Variable(_) => true,
        }
    }

    fn reduce(&self, env: &Environment) -> Self {
        match self {
            Self::Number(_) => self.clone(),
            Self::Add(l, r) => {
                if l.is_reducible() {
                    Self::Add(Box::new(l.reduce(env)), r.clone())
                } else if r.is_reducible() {
                    Self::Add(l.clone(), Box::new(r.reduce(env)))
                } else {
                    match (*l.clone(), *r.clone()) {
                        (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
                        _ => panic!("invalid expr"),
                    }
                }
            }
            Self::Multiply(l, r) => {
                if l.is_reducible() {
                    Self::Multiply(Box::new(l.reduce(env)), r.clone())
                } else if r.is_reducible() {
                    Self::Multiply(l.clone(), Box::new(r.reduce(env)))
                } else {
                    match (*l.clone(), *r.clone()) {
                        (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
                        _ => panic!("invalid expr"),
                    }
                }
            }
            Self::Boolean(_) => self.clone(),
            Self::LessThan(l, r) => {
                if l.is_reducible() {
                    Self::LessThan(Box::new(l.reduce(env)), r.clone())
                } else if r.is_reducible() {
                    Self::LessThan(l.clone(), Box::new(r.reduce(env)))
                } else {
                    match (*l.clone(), *r.clone()) {
                        (Self::Number(a), Self::Number(b)) => Self::Boolean(a < b),
                        _ => panic!("invalid expr"),
                    }
                }
            }
            Self::Variable(name) => env[name].clone(),
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

struct Machine {
    expr: Expr,
    env: Environment,
}

impl Machine {
    fn new(expr: Expr, env: Environment) -> Self {
        Self { expr, env }
    }

    fn step(&mut self) {
        self.expr = self.expr.reduce(&self.env);
    }

    fn run(&mut self) {
        while self.expr.is_reducible() {
            println!("{}", self.expr);
            self.step();
        }
        println!("{}", self.expr);
    }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Variable("x".into())),
        Box::new(Expr::Variable("y".into())),
    );
    let mut env = HashMap::new();
    env.insert("x".to_string(), Expr::Number(3));
    env.insert("y".to_string(), Expr::Number(4));
    let mut machine = Machine::new(expr, env);
    machine.run();
}
