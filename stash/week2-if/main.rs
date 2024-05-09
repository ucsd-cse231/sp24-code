use std::env;
use std::fs::File;
use std::io::prelude::*;

use expr::Expr;
use im::HashMap;

pub mod expr;

type Stack = HashMap<String, usize>;

fn compile_expr(e: &Expr, env: &Stack, sp: usize, count: &mut i32) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n),
        Expr::Add1(subexpr) => compile_expr(subexpr, env, sp, count) + "\nadd rax, 1",
        Expr::Sub1(subexpr) => compile_expr(subexpr, env, sp, count) + "\nsub rax, 1",
        Expr::Neg(subexpr) => compile_expr(subexpr, env, sp, count) + "\nneg rax",
        Expr::Var(x) => {
            let x_pos = env.get(x).unwrap();
            format!("mov rax, [rsp - 8*{}]", x_pos)
        }
        Expr::Let(x, e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count);
            let x_pos = sp;
            let x_save = format!("mov [rsp - 8*{}], rax", x_pos);
            let new_env = env.update(x.to_string(), x_pos);
            let e2_code = compile_expr(e2, &new_env, sp + 1, count);
            format!("{e1_code:}\n{x_save:}\n{e2_code:}")
        }
        Expr::Plus(e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count);
            let e2_code = compile_expr(e2, env, sp + 1, count);
            format!(
                "{e1_code}
                 mov [rsp - 8*{sp}], rax
                 {e2_code}
                 add rax, [rsp - 8*{sp}]
                "
            )
        }
        Expr::If(e_cond, e_then, e_else) => {
            *count += 1;
            let e_cond_code = compile_expr(e_cond, env, sp, count);
            let e_then_code = compile_expr(e_then, env, sp, count);
            let e_else_code = compile_expr(e_else, env, sp, count);
            format!(
                "{e_cond_code}
                      cmp rax, 0
                      je label_else_{count}
                      {e_then_code}
                      jmp label_exit_{count}
                    label_else_{count}:
                      {e_else_code}
                    label_exit_{count}:"
            )
        }
        Expr::True => format!("mov rax, 1"),
        Expr::False => format!("mov rax, 0"),
        Expr::Eq(_e1, _e2) => todo!(),
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
    let mut count = 0;
    let result = compile_expr(&expr, &HashMap::new(), 1, &mut count);

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

/*

                        {}, 0
(let (a 10)
                        {a:1}, 1
    (let (b 20)
                        {a:1, b:2}, 2
        (let (a 30)
                        {a:3, b:2}, 3
            (let (b 40)
                        {a:3, b:4}
                a))))

]


                        {}
(let (a 10)
                        {a:[1]}
    (let (b 20)
                        {a:[1], b:[2]}
        (let (a 30)
                        {a:[1,3], b:[2]}
            (let (b 40)
                        {a:[1,3], b:[2,4]}
                a))))




                        {}
(let (a 10)
                        {a:1}
    (let (b 20)
                        {a:1, b:2}
        (let (a 30)
                        {a:1, b:2}
            (let (b 40)
                        {a:[1,3], b:[2,4]}
                a))))



*/
