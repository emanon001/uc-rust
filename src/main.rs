#[derive(Debug, PartialEq, Eq, Clone)]
enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
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
    println!("{:?}", expr);
}
