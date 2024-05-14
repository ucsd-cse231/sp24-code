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
 ;; CHECK rax is a function! HOW?
 ;; CHECK ARITY?
 sub rax, 5             ;; remove TAG
 mov rcx, [rax+8]       ;; load EXPECTED "arity" into rcx
 cmp rcx, 1             ;; compare ACTUAL arity
 mov rdi, 404           ;; error code for arity-error
 jne label_error        ;; exit if mismatch
 mov rax, [rax]         ;; load actual label of `it` into rax
 call rax               ;; call `it`
 add rsp, 8*1
fun_exit_anon_1:
 mov rsp, rbp
 pop rbp
 ret
fun_finish_anon_1:      ;; save `fn` as local-#1 `f`
 ;; allocate tuple for fun_start_anon_1
 mov rax, fun_start_anon_1
 mov [r11], rax         ;; save label
 mov rax, 1
 mov [r11 + 8], rax     ;; save arity = 1
 mov rax, r11           ;; save tuple address
 add r11, 16            ;; bump allocation pointer
 add rax, 5             ;; tag rax as "function"
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
 ;; allocate tuple for fun_start_anon_2
  lea rax, QWORD [rel fun_start_anon_2]
  mov [r11], rax         ;; save label
  mov rax, 5
  mov [r11 + 8], rax     ;; save arity = 5
  mov rax, r11           ;; save tuple address
  add r11, 16            ;; bump allocation pointer
  add rax, 5             ;; tag rax as "function"
  mov [rbp - 8*3], rax

;; (f inc)
mov rax, [rbp - 8*3]    ;; push `inc` as arg
push rax
mov rax, [rbp - 8*2]    ;; load `f` tuple into rax
;; CHECK function TAG
sub rax, 5
;; CHECK arity
mov rax, [rax]          ;; load actual label
call rax
add rsp, 8*1

mov rsp, rbp
pop rbp
ret
