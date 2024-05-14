section .text
global our_code_starts_here
extern snek_error
extern snek_print
label_error:
  push rsp
  call snek_error
our_code_starts_here:
 push rbp
 mov rbp, rsp
 sub rsp, 8*103
 mov [rbp - 8], rdi
 mov r11, rsi

;; block to define `f`
 jmp fun_finish_anon_1
fun_start_anon_1:
 push rbp
 mov rbp, rsp
 sub rsp, 8*101
fun_body_anon_1:
 mov rax, 10
 push rax               ;; push arg <5>
 mov rax, [rbp - 8*-2]  ;; load `it`
 call rax               ;; call `it`
 add rsp, 8*1
fun_exit_anon_1:
 mov rsp, rbp
 pop rbp
 ret
fun_finish_anon_1:      ;; save `fn` as local-#1 `f`
 mov rax, fun_start_anon_1
 mov [rbp - 8*2], rax

;; block to define `inc`
jmp fun_finish_anon_2
fun_start_anon_2:
  push rbp
  mov rbp, rsp
  sub rsp, 8*105
fun_body_anon_2:
  mov rax, [rbp - 8*-2]
  mov rcx, [rbp - 8*-3]
  add rax, rcx
  mov rcx, [rbp - 8*-4]
  add rax, rcx
  mov rcx, [rbp - 8*-5]
  add rax, rcx
  mov rcx, [rbp - 8*-6]
  add rax, rcx
fun_exit_anon_2:
  mov rsp, rbp
  pop rbp
  ret
fun_finish_anon_2:
  lea rax, QWORD [rel fun_start_anon_2]
  mov [rbp - 8*3], rax

;; (f add)
mov rax, [rbp - 8*3]    ;; push `inc` as arg
push rax
mov rax, [rbp - 8*2]    ;; load `f` into rax
call rax
add rsp, 8*1

mov rsp, rbp
pop rbp
ret
