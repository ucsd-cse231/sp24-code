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
    Input,
    True,
    False,
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Print(Box<Expr>),
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

pub fn eq(e1: Expr, e2: Expr) -> Expr {
    Expr::Eq(Box::new(e1), Box::new(e2))
}

pub fn ite(e1: Expr, e2: Expr, e3: Expr) -> Expr {
    Expr::If(Box::new(e1), Box::new(e2), Box::new(e3))
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

        Sexp::Atom(S(s)) if s == "input" => Expr::Input,
        Sexp::Atom(S(s)) if s == "true" => Expr::True,
        Sexp::Atom(S(s)) if s == "false" => Expr::False,

        Sexp::Atom(S(s)) => Expr::Var(s.clone()),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => add1(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "sub1" => sub1(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "negate" => negate(parse_expr(e)),
            [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "print" => Expr::Print(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => plus(parse_expr(e1), parse_expr(e2)),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => eq(parse_expr(e1), parse_expr(e2)),
            [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => {
                ite(parse_expr(e1), parse_expr(e2), parse_expr(e3))
            }
            [Sexp::Atom(S(op)), bind, e2] if op == "let" => {
                let (x, e1) = parse_bind(bind);
                let e2 = parse_expr(e2);
                Expr::Let(x, Box::new(e1), Box::new(e2))
            }
            [Sexp::Atom(S(op)), Sexp::Atom(S(x)), e] if op == "set!" => {
                Expr::Set(x.to_string(), Box::new(parse_expr(e)))
            }
            [Sexp::Atom(S(op)), exprs @ ..] if op == "block" => {
                Expr::Block(exprs.into_iter().map(parse_expr).collect())
            }
            _ => panic!("parse error (1) {}", s),
        },
        _ => panic!("parse error (2) {}", s),
    }
}
