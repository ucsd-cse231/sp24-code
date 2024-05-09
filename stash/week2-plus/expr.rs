use sexp::Atom::*;
use sexp::*;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Neg(Box<Expr>),
    Var(String),
    Let(String, Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
}

pub fn num(n: i32) -> Expr {
    Expr::Num(n)
}

pub fn add1(e: Expr) -> Expr {
    Expr::Add1(Box::new(e))
}

pub fn sub1(e: Expr) -> Expr {
    Expr::Sub1(Box::new(e))
}

pub fn negate(e: Expr) -> Expr {
    Expr::Neg(Box::new(e))
}

pub fn expr0() -> Expr {
    add1(sub1(num(5)))
}

pub fn expr1() -> Expr {
    negate(add1(num(5)))
}

pub fn plus(e1: Expr, e2: Expr) -> Expr {
    Expr::Plus(Box::new(e1), Box::new(e2))
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(x)), e] => (x.to_string(), parse_expr(e)),
            _ => panic!("parse error"),
        },
        _ => panic!("parse error"),
    }
}

pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => num(i32::try_from(*n).unwrap()),
        Sexp::Atom(S(s)) => Expr::Var(s.clone()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => add1(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "sub1" => sub1(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "negate" => negate(parse_expr(e)),
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => plus(parse_expr(e1), parse_expr(e2)),
            [Sexp::Atom(S(op)), bind, e2] if op == "let" => {
                let (x, e1) = parse_bind(bind);
                let e2 = parse_expr(e2);
                Expr::Let(x, Box::new(e1), Box::new(e2))
            }
            _ => panic!("parse error (1) {}", s),
        },
        _ => panic!("parse error (2) {}", s),
    }
}
