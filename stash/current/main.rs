use std::fs::File;
use std::io::prelude::*;
use std::{cmp::max, env};

use expr::{Defn, Expr, Prog};
use im::{hashmap, HashMap};

pub mod expr;

type Stack = HashMap<String, i32>;

const FALSE: usize = 3;
const TRUE: usize = 7;

fn test_number(code: usize) -> String {
    format!(
        "mov rcx, rax
             and rcx, {FALSE}
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
            format!("mov rax, [rbp - 8*{}]", x_pos)
        }
        Expr::Let(x, e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let x_pos = sp;
            let x_save = format!("mov [rbp - 8*{}], rax", x_pos);
            let new_env = env.update(x.to_string(), x_pos as i32);
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
                 mov [rbp - 8*{sp}], rax
                 {e2_code}
                 {test_code_2}
                 add rax, [rbp - 8*{sp}]
                "
            )
        }
        Expr::Mult(e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let e2_code = compile_expr(e2, env, sp + 1, count, brk);
            let test_code_1 = test_number(99);
            let test_code_2 = test_number(33);
            let off = 8 * sp;
            format!(
                "{e1_code}
                 {test_code_1}
                 mov [rbp - {off}], rax
                 {e2_code}
                 {test_code_2}
                 sar rax, 1
                 imul rax, [rbp - {off}]
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
                      cmp rax, {FALSE}
                      je label_else_{count}
                      {e_then_code}
                      jmp label_exit_{count}
                    label_else_{count}:
                      {e_else_code}
                    label_exit_{count}:"
            )
        }
        Expr::Input => {
            format!("mov rax, [rbp - 8]")
        }
        Expr::True => {
            format!("mov rax, {TRUE}")
        }
        Expr::False => {
            format!("mov rax, {FALSE}")
        }
        Expr::Eq(e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let e2_code = compile_expr(e2, env, sp + 1, count, brk);
            *count += 1;
            let exit = label("eq_exit".to_string(), count);
            format!(
                "{e1_code}
                 mov [rbp - 8*{sp}], rax
                 {e2_code}
                 cmp rax, [rbp - 8*{sp}]
                 mov rax, {FALSE}
                 jne {exit}
                 mov rax, {TRUE}
               {exit}:
                "
            )
        }
        Expr::Set(x, e) => {
            let x_pos = env.get(x).unwrap();
            let e_code = compile_expr(e, env, sp, count, brk);
            format!(
                "{e_code}
                     mov [rbp - 8*{}], rax",
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
        Expr::Print(e) => {
            let e_code = compile_expr(e, env, sp, count, brk);
            format!(
                "{e_code}
                     mov rdi, rax
                     call snek_print
                    "
            )
        }
        Expr::Call1(f, e1) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            format!(
                "{e1_code}
                 push rax
                 call fun_start_{f}
                 add rsp, 8*1
                "
            )
        }
        Expr::Call2(f, e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let e2_code = compile_expr(e2, env, sp + 1, count, brk);
            format!(
                "{e1_code}
                 mov [rbp - 8*{sp}], rax
                 {e2_code}
                 push rax
                 mov rcx, [rbp - 8*{sp}]
                 push rcx
                 call fun_start_{f}
                 add rsp, 8*2
                "
            )
        }
        Expr::Vec(e1, e2) => {
            let e1_code = compile_expr(e1, env, sp, count, brk);
            let e2_code = compile_expr(e2, env, sp + 1, count, brk);
            format!(
                "{e1_code}
                 mov [rbp - 8*{sp}], rax
                 {e2_code}
                 mov rcx, [rbp - 8*{sp}]
                 mov [r11], rcx
                 mov [r11 + 8], rax
                 mov rax, r11
                 add rax, 1
                 add r11, 16"
            )
        }
        Expr::Get(_, _) => todo!(),
    }
}

fn compile_exit() -> String {
    format!(
        "mov rsp, rbp
             pop rbp
             ret"
    )
}

fn compile_entry(e: &Expr, sp: usize) -> String {
    let vars = expr_vars(e) + sp;
    format!(
        "push rbp
             mov rbp, rsp
             sub rsp, 8*{vars}"
    )
}

fn expr_vars(e: &Expr) -> usize {
    match e {
        Expr::Num(_) | Expr::Var(_) | Expr::Input | Expr::True | Expr::False => 0,
        Expr::Add1(e)
        | Expr::Sub1(e)
        | Expr::Neg(e)
        | Expr::Set(_, e)
        | Expr::Loop(e)
        | Expr::Break(e)
        | Expr::Print(e)
        | Expr::Call1(_, e) => expr_vars(e),
        Expr::Call2(_, e1, e2)
        | Expr::Let(_, e1, e2)
        | Expr::Eq(e1, e2)
        | Expr::Plus(e1, e2)
        | Expr::Mult(e1, e2)
        | Expr::Vec(e1, e2) => max(expr_vars(e1), 1 + expr_vars(e2)),
        Expr::If(e1, e2, e3) => max(expr_vars(e1), max(expr_vars(e2), expr_vars(e3))),
        Expr::Block(es) => es.iter().map(|e| expr_vars(e)).max().unwrap(),
        Expr::Get(_, _) => todo!(),
    }
}

fn init_env(args: &[String]) -> HashMap<String, i32> {
    match &args[..] {
        [] => hashmap! {},
        [x1] => hashmap! { x1.to_string() => -2 },
        [x1, x2] => hashmap! { x1.to_string() => -2, x2.to_string() => -3 },
        _ => panic!("Too many arguments"),
    }
}

fn compile_def_body(args: &[String], sp: usize, body: &Expr, count: &mut i32) -> String {
    let fun_entry = compile_entry(body, sp);
    let body_code = compile_expr(body, &init_env(args), sp, count, "time_to_exit");
    let fun_exit = compile_exit();

    format!(
        "{fun_entry}
         {body_code}
         {fun_exit}"
    )
}

fn compile_def(def: &Defn, count: &mut i32) -> String {
    let (f, args, body) = match def {
        Defn::Fun1(f, x1, e) => (f, vec![x1.to_string()], e),
        Defn::Fun2(f, x1, x2, e) => (f, vec![x1.to_string(), x2.to_string()], e),
    };
    let body = compile_def_body(&args, 1, body, count);
    format!(
        "fun_start_{f}:
            {body}"
    )
}

fn compile_prog(prog: &Prog) -> String {
    let mut count = 0;
    let defs_code = prog
        .defs
        .iter()
        .map(|def| compile_def(def, &mut count))
        .collect::<Vec<String>>()
        .join("\n");
    let e_entry = compile_entry(&prog.expr, 1);
    let e_code = compile_expr(&prog.expr, &hashmap! {}, 2, &mut count, "time_to_exit");
    let e_exit = compile_exit();
    format!(
        "section .text
global our_code_starts_here
extern snek_error
extern snek_print
label_error:
  push rsp
  call snek_error
{defs_code}
our_code_starts_here:
 {e_entry}
 mov [rbp - 8], rdi
 mov r11, rsi
 add r11, 7
 mov rax, 0xfffffffffffffff8
 and r11, rax
 {e_code}
 {e_exit}
time_to_exit:
  ret
"
    )
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let prog = expr::parse(&in_contents);

    let mut out_file = File::create(out_name)?;
    let asm_program = compile_prog(&prog);

    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
