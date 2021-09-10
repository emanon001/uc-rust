use std::fmt::{self};

#[derive(PartialEq, Eq, Clone)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Boolean(bool),
}

impl Expr {
    fn is_reducible(&self) -> bool {
        match self {
            Self::Number(_) => false,
            Self::Add(_, _) => true,
            Self::Multiply(_, _) => true,
            Self::Boolean(_) => false,
        }
    }

    fn reduce(&self) -> Self {
        match self {
            Self::Number(_) => self.clone(),
            Self::Add(l, r) => {
                if l.is_reducible() {
                    Self::Add(Box::new(l.reduce()), r.clone())
                } else if r.is_reducible() {
                    Self::Add(l.clone(), Box::new(r.reduce()))
                } else {
                    match (*l.clone(), *r.clone()) {
                        (Self::Number(a), Self::Number(b)) => Self::Number(a + b),
                        _ => panic!("invalid expr"),
                    }
                }
            }
            Self::Multiply(l, r) => {
                if l.is_reducible() {
                    Self::Add(Box::new(l.reduce()), r.clone())
                } else if r.is_reducible() {
                    Self::Add(l.clone(), Box::new(r.reduce()))
                } else {
                    match (*l.clone(), *r.clone()) {
                        (Self::Number(a), Self::Number(b)) => Self::Number(a * b),
                        _ => panic!("invalid expr"),
                    }
                }
            }
            Self::Boolean(_) => self.clone(),
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
}

impl Machine {
    fn new(expr: Expr) -> Self {
        Self { expr }
    }

    fn step(&mut self) {
        self.expr = self.expr.reduce();
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
        Box::new(Expr::Multiply(
            Box::new(Expr::Number(1)),
            Box::new(Expr::Number(2)),
        )),
        Box::new(Expr::Multiply(
            Box::new(Expr::Number(3)),
            Box::new(Expr::Number(4)),
        )),
    );
    let mut machine = Machine::new(expr);
    machine.run();
}
