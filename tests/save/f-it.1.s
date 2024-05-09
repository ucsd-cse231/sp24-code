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
  mov rax, [rbp + 16]
  call rax
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
  mov rax, fun_start_incr
  push rax
  call fun_start_f
  add rsp, 8*1
  ; teardown stack frame
  mov rsp, rbp
  pop rbp
  ret
time_to_exit:
  ret
