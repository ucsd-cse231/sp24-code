use sexp::Atom::*;
use sexp::*;

#[derive(Debug)]
pub struct Prog {
    pub defs: Vec<Defn>,
    pub expr: Box<Expr>,
}

#[derive(Debug)]
pub enum Defn {
    Fun1(String, String, Box<Expr>),
    Fun2(String, String, String, Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Neg(Box<Expr>),
    Var(String),
    Let(String, Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Input,
    True,
    False,
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Print(Box<Expr>),
    Call1(String, Box<Expr>),
    Call2(String, Box<Expr>, Box<Expr>),
    Vec(Box<Expr>, Box<Expr>),
    Get(Box<Expr>, Index),
}

#[derive(Debug)]
pub enum Index {
    Zero,
    One,
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

pub fn mult(e1: Expr, e2: Expr) -> Expr {
    Expr::Mult(Box::new(e1), Box::new(e2))
}

pub fn eq(e1: Expr, e2: Expr) -> Expr {
    Expr::Eq(Box::new(e1), Box::new(e2))
}

pub fn le(e1: Expr, e2: Expr) -> Expr {
    Expr::Le(Box::new(e1), Box::new(e2))
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

fn parse_ident(s: &Sexp) -> String {
    match s {
        Sexp::Atom(S(x)) => x.to_string(),
        _ => panic!("parse error"),
    }
}

pub fn parse_defn(s: &Sexp) -> Defn {
    let Sexp::List(es) = s else {
        panic!("syntax error: expected a list")
    };
    match &es[..] {
        [Sexp::Atom(S(op)), Sexp::List(xs), body] if op == "defn" => {
            //  let [name, params @ ..] = ["cat", "dog", "mouse", "hippo"]
            //     name   <- "cat"
            //     params <- ["dog", "mouse", "hippo"]

            //  let [params @ .., name] = ["cat", "dog", "mouse", "hippo"]
            //     params <- ["cat", "dog", "mouse"]
            //     name   <- "hippo"
            let [name, params @ ..] = &xs[..] else {
                panic!("missing function name");
            };
            let body = Box::new(parse_expr(body));
            let name = parse_ident(name);
            if params.len() == 1 {
                Defn::Fun1(name, parse_ident(&params[0]), body)
            } else if params.len() == 2 {
                Defn::Fun2(name, parse_ident(&params[0]), parse_ident(&params[1]), body)
            } else {
                panic!("syntax error: expected a list of 3 elements")
            }
        }

        _ => panic!("syntax error: expected a list of 4 elements"),
    }
}

fn parse_prog(e: &Sexp) -> Prog {
    let Sexp::List(es) = e else {
        panic!("syntax error: expected a list")
    };

    if let [defs @ .., expr] = &es[..] {
        let defs = defs.iter().map(|e| parse_defn(e)).collect();
        let expr = Box::new(parse_expr(expr));
        Prog { defs, expr }
    } else {
        panic!("syntax error: program must contain a main expression")
    }
}

fn parse_index(s: &Sexp) -> Index {
    match s {
        Sexp::Atom(S(op)) if op == "0" => Index::Zero,
        Sexp::Atom(S(op)) if op == "1" => Index::One,
        _ => panic!("parse error"),
    }
}

fn parse_expr(s: &Sexp) -> Expr {
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
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => mult(parse_expr(e1), parse_expr(e2)),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => eq(parse_expr(e1), parse_expr(e2)),
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => le(parse_expr(e1), parse_expr(e2)),
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
            [Sexp::Atom(S(op)), e1, e2] if op == "vec" => {
                Expr::Vec(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "vec-get" => {
                Expr::Get(Box::new(parse_expr(e1)), parse_index(e2))
            }
            [Sexp::Atom(S(f)), e1] => Expr::Call1(f.to_string(), Box::new(parse_expr(e1))),
            [Sexp::Atom(S(f)), e1, e2] => Expr::Call2(
                f.to_string(),
                Box::new(parse_expr(e1)),
                Box::new(parse_expr(e2)),
            ),

            _ => panic!("parse error (1) {}", s),
        },
        _ => panic!("parse error (2) {}", s),
    }
}

pub fn parse(s: &str) -> Prog {
    let s = format!("({})", s);
    let s = sexp::parse(&s).unwrap_or_else(|_| panic!("invalid s-expr"));
    parse_prog(&s)
}
