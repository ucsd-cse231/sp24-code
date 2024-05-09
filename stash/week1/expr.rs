use sexp::Atom::*;
use sexp::*;

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Neg(Box<Expr>),
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

pub fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => num(i32::try_from(*n).unwrap()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => add1(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "sub1" => sub1(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "negate" => negate(parse_expr(e)),
            _ => panic!("parse error (1) {}", s),
        },
        _ => panic!("parse error (2) {}", s),
    }
}
