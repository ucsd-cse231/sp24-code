use std::env;
use std::fs::File;
use std::io::prelude::*;

use expr::Expr;
use im::HashMap;

pub mod expr;

type Stack = HashMap<String, usize>;

fn test_number(code: usize) -> String {
    format!(
        "mov rcx, rax
             and rcx, 1
             cmp rcx, 0
             mov rdi, {code}
             jne label_error"
    )
}

fn label(prefix: String, count: &i32) -> String {
    format!("{prefix}_{count}")
}

fn compile_expr(e: &Expr, env: &Stack, sp: usize, count: &mut i32, brk: &str) -> String {
    match e {
        Expr::Num(n) => format!("mov rax, {}", *n << 1),
        Expr::Add1(subexpr) => compile_expr(subexpr, env, sp, count, brk) + "\nadd rax, 2",
        Expr::Sub1(subexpr) => compile_expr(subexpr, env, sp, count, brk) + "\nsub rax, 2",
        Expr::Neg(subexpr) => compile_expr(subexpr, env, sp, count, brk) + "\nneg rax",
        Expr::Var(x) => {
            let x_pos = env.get(x).unwrap();
            format!("mov rax, [rsp - 8*{}]", x_pos)
        }
        Expr::Let(x, e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let x_pos = sp;
            let x_save = format!("mov [rsp - 8*{}], rax", x_pos);
            let new_env = env.update(x.to_string(), x_pos);
            let e2_code = compile_expr(e2, &new_env, sp + 1, count, brk);
            format!("{e1_code:}\n{x_save:}\n{e2_code:}")
        }
        Expr::Plus(e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let e2_code = compile_expr(e2, env, sp + 1, count, brk);
            let test_code_1 = test_number(99);
            let test_code_2 = test_number(33);

            format!(
                "{e1_code}
                 {test_code_1}
                 mov [rsp - 8*{sp}], rax
                 {e2_code}
                 {test_code_2}
                 add rax, [rsp - 8*{sp}]
                "
            )
        }
        Expr::If(e_cond, e_then, e_else) => {
            *count += 1;
            let e_cond_code = compile_expr(e_cond, env, sp, count, brk);
            let e_then_code = compile_expr(e_then, env, sp, count, brk);
            let e_else_code = compile_expr(e_else, env, sp, count, brk);
            format!(
                "{e_cond_code}
                      cmp rax, 1
                      je label_else_{count}
                      {e_then_code}
                      jmp label_exit_{count}
                    label_else_{count}:
                      {e_else_code}
                    label_exit_{count}:"
            )
        }
        Expr::Input => {
            format!("mov rax, [rsp - 8]")
        }
        Expr::True => {
            format!("mov rax, 3")
        }
        Expr::False => {
            format!("mov rax, 1")
        }
        Expr::Eq(e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let e2_code = compile_expr(e2, env, sp + 1, count, brk);
            *count += 1;
            let exit = label("eq_exit".to_string(), count);
            format!(
                "{e1_code}
                 mov [rsp - 8*{sp}], rax
                 {e2_code}
                 cmp rax, [rsp - 8*{sp}]
                 mov rax, 1
                 jne {exit}
                 mov rax, 3
               {exit}:
                "
            )
        }
        Expr::Set(x, e) => {
            let x_pos = env.get(x).unwrap();
            let e_code = compile_expr(e, env, sp, count, brk);
            format!(
                "{e_code}
                     mov [rsp - 8*{}], rax",
                x_pos
            )
        }
        Expr::Block(es) => {
            let e_codes: Vec<String> = es
                .iter()
                .map(|e| compile_expr(e, env, sp, count, brk))
                .collect();
            e_codes.join("\n")
        }
        Expr::Loop(e) => {
            *count += 1;
            let loop_start = label("loop_start".to_string(), count);
            let loop_exit = label("loop_exit".to_string(), count);
            let e_code = compile_expr(e, env, sp, count, &loop_exit);
            format!(
                "{loop_start}:
                        {e_code}
                        jmp {loop_start}
                     {loop_exit}:"
            )
        }
        Expr::Break(e) => {
            let e_code = compile_expr(e, env, sp, count, brk);
            format!(
                "{e_code}
                     jmp {brk}"
            )
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
    let mut count = 0;
    let time_to_exit = "time_to_exit";
    let result = compile_expr(&expr, &HashMap::new(), 2, &mut count, &time_to_exit);

    let asm_program = format!(
        "section .text
global our_code_starts_here
extern snek_error
label_error:
  push rsp
  call snek_error
our_code_starts_here:
  mov [rsp - 8], rdi
  {}
{time_to_exit}:
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
