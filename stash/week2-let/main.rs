use std::env;
use std::fs::File;
use std::io::prelude::*;

use expr::Expr;
use im::HashMap;

pub mod expr;

pub fn eval(e: &Expr, env: &HashMap<String, i32>) -> i32 {
    match e {
        Expr::Num(n) => *n,
        Expr::Add1(e1) => eval(e1, env) + 1,
        Expr::Sub1(e1) => eval(e1, env) - 1,
        Expr::Neg(e1) => -eval(e1, env),
        Expr::Var(x) => *env.get(x).unwrap(),
        Expr::Let(x, e1, e2) => {
            let v1 = eval(e1, env);
            let new_env = env.update(x.to_string(), v1);
            eval(e2, &new_env)
        }
    }
}

type Stack = HashMap<String, usize>;

fn compile_expr(e: &Expr, env: &Stack) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr, env) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr, env) + "\nsub rax, 1",
        Expr::Neg(subexpr) => compile_expr(subexpr, env) + "\nneg rax",
        Expr::Var(x) => {
            let x_pos = env.get(x).unwrap();
            format!("mov rax, [rsp - 8*{}]", x_pos)
        }
        Expr::Let(x, e1, e2) => {
            let e1_code = compile_expr(e1, env);
            let x_pos = env.len() + 1;
            let x_save = format!("mov [rsp - 8*{}], rax", x_pos);
            let new_env = env.update(x.to_string(), x_pos);
            let e2_code = compile_expr(e2, &new_env);
            format!("{e1_code:}\n{x_save:}\n{e2_code:}")

            // compute e1 into RAX
            // SAVE RAX at the position for x
            // compute e2
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let expr = expr::parse_expr(&sexp::parse(&in_contents).unwrap());
    let result = compile_expr(&expr, &HashMap::new());

    let asm_program = format!(
        "section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eval() {
        let e = Expr::Add1(Box::new(Expr::Num(1)));
        assert_eq!(eval(&e, &HashMap::new()), 2);
    }

    // let (x 10) (add1 x)
    #[test]
    fn test_compile_expr() {
        let e = Expr::Let(
            "x".to_string(),
            Box::new(Expr::Num(10)),
            Box::new(Expr::Add1(Box::new(Expr::Var("x".to_string())))),
        );
        assert_eq!(eval(&e, &HashMap::new()), 11);
    }
}
