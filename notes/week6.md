# Functions as Values

## Part 0: Grammar/Syntax

**Syntax**

```clojure
e ::= ...
    | (defn (f x1... xn) e) ; definition
    | (f e1 ... en)         ; function call
```

**Examples**

Consider the example `f-it.snek`

```clojure
(defn (f it)
  (it 5))

(defn (incr x)
  (+ x 1))

(f incr)
```

**Types**

```rust
pub struct Defn {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Box<Expr>,
}

pub enum Expr {
    ...
    Fun(Defn),
    Call(String, Vec<Expr>),
}
```

## Part 1: Labels

How shall we fill in the `FIXME` for `f-it.s`

```nasm
section .text
global our_code_starts_here
extern snek_error
extern snek_print
label_error:
  push rsp
  call snek_error
; definition of incr
fun_start_incr:
  push rbp
  mov rbp, rsp
  sub rsp, 8*100
fun_body_incr:
  mov rax, [rbp - 8*-2]   ; load x
  add rax, 2              ; add <1>
fun_exit_incr:
  mov rsp, rbp
  pop rbp
  ret
; definition of f
fun_start_f:
  push rbp
  mov rbp, rsp
  sub rsp, 8*100
fun_body_f:
  mov rax, 10
  push rax
  call FIXME1
  add rsp, 8*1
fun_exit_f:
  mov rsp, rbp
  pop rbp
  ret
; definition of main
our_code_starts_here:
  ; setup stack frame
  push rbp
  mov rbp, rsp
  sub rsp, 8*100
  ; body of `main`
  mov [rbp - 8], rdi  ; save `input`
  mov r11, rsi        ; save start of heap in r11
  push FIXME2
  call fun_start_f
  add rsp, 8*1
  ; teardown stack frame
  mov rsp, rbp
  pop rbp
  ret
time_to_exit:
  ret
```

## Part 2: Self

A problem, what happens if we try to compile

```clojure
(defn (sum n)
  (if (= n 0)
      0
      (+ n (sum (+ n -1))))
)

(sum input)
```

How can we fix it?

## Part 3: Arity

Another problem, what should we do with?

```clojure
(let (f (fn (f it) (it 5)))
   (let (add (fn (x y) (+ x y)))
      (f add)
   )
)
```


## Part 3: Environment
