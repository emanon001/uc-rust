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
            _ => panic!("`reduce()` not supported"),
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
}

impl Stmt {
    fn is_reducible(&self) -> bool {
        match self {
            Self::DoNothing => false,
            Self::Assign(_, _) => true,
            Self::If { .. } => true,
        }
    }

    fn reduce(&self, env: &Environment) -> (Stmt, Environment) {
        match self {
            Self::Assign(name, expr) => {
                if expr.is_reducible() {
                    (Self::Assign(name.into(), expr.reduce(env)), env.clone())
                } else {
                    let mut new_env = env.clone();
                    new_env.insert(name.into(), expr.clone());
                    (Self::DoNothing, new_env)
                }
            }
            Self::If {
                condition,
                consequence,
                alternative,
            } => {
                if condition.is_reducible() {
                    (
                        Self::If {
                            condition: condition.reduce(env),
                            consequence: consequence.clone(),
                            alternative: alternative.clone(),
                        },
                        env.clone(),
                    )
                } else {
                    match condition {
                        Expr::Boolean(true) => (*consequence.clone(), env.clone()),
                        Expr::Boolean(false) => (*alternative.clone(), env.clone()),
                        _ => panic!("invalid condition"),
                    }
                }
            }
            _ => panic!("`reduce()` not supported"),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::DoNothing => write!(f, "do-nothing"),
            Self::Assign(name, val) => write!(f, "{} = {}", name, val),
            Self::If {
                condition,
                consequence,
                alternative,
            } => write!(
                f,
                "if ({}) {{ {} }} else {{ {} }}",
                condition, consequence, alternative
            ),
        }
    }
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "<<{}>>", self)
    }
}

struct Machine {
    stmt: Stmt,
    env: Environment,
}

impl Machine {
    fn new(stmt: Stmt, env: Environment) -> Self {
        Self { stmt, env }
    }

    fn step(&mut self) {
        let (new_stmt, new_env) = self.stmt.reduce(&self.env);
        self.stmt = new_stmt;
        self.env = new_env;
    }

    fn run(&mut self) {
        while self.stmt.is_reducible() {
            println!("{}, {:?}", self.stmt, self.env);
            self.step();
        }
        println!("{}, {:?}", self.stmt, self.env);
    }
}

fn main() {
    let stmt = Stmt::If {
        condition: Expr::Variable("x".into()),
        consequence: Stmt::Assign("y".into(), Expr::Number(1)).into(),
        alternative: Stmt::Assign("y".into(), Expr::Number(2)).into(),
    };
    let mut env = HashMap::new();
    env.insert("x".to_string(), Expr::Boolean(true));
    let mut machine = Machine::new(stmt, env);
    machine.run();
}
