section .text
global our_code_starts_here
extern snek_error
extern snek_print
label_error:
  push rsp
  call snek_error
our_code_starts_here:
;; setup "main" stack
  push rbp
  mov rbp, rsp
  sub rsp, 8*100
  mov [rbp - 8], rdi   ;; save input
  mov r11, rsi         ;; init heap

;; block for `f`
  jmp fun_finish_f
fun_start_f:
  push rbp
  mov rbp, rsp
  sub rsp, 8*101
fun_body_f:
  mov rax, 10           ;; push arg 5
  push rax
  mov rax, [rbp - 8*-2] ;; load `it`
  call rax              ;; call `it`
  add rsp, 8*1          ;; pop arg
fun_exit_f:
  mov rsp, rbp
  pop rbp
  ret
fun_finish_f:
  mov rax, fun_start_f  ;; save `f` as local#1 (f) in "main"
  mov [rbp - 8*2], rax

;; block for `fn (z) (+ z 1)`
  jmp fun_finish_anon_1
fun_start_anon_1:
  push rbp
  mov rbp, rsp
  sub rsp, 8*100
fun_body_anon_1:
  mov rax, [rbp - 8*-2] ;; load z
  add rax, 2            ;; add 1
fun_exit_anon_1:
  mov rsp, rbp
  pop rbp
  ret
fun_finish_anon_1:
  mov rax, fun_start_anon_1
  mov [rbp - 8*3], rax  ;; save `fn..` as local#2 (inc) in "main"

;; block for `(f inc)`
mov rax, [rbp - 8*3]    ;; load `inc` into rax
push rax                ;; push as arg
mov rax, [rbp - 8*2]    ;; load caller `f` into rax
call rax
add rsp, 8*1

;; teardown "main" stack
mov rsp, rbp
pop rbp
ret
