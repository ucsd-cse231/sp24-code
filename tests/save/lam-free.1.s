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

;; block to define `five` as local#1
 mov rax, 10
 mov [rbp - 8*2], rax

;; block to define `f`
 jmp fun_finish_anon_1
fun_start_anon_1:
 push rbp
 mov rbp, rsp
 sub rsp, 8*101
fun_body_anon_1:
 ;; RESTORE five
 mov rax, [rbp - 8*-2]  ;; load `self` closure into rax,
 sub rax, 5             ;; remove tag
 mov rax, [rax + 16]    ;; load `five` from closure
 mov [rbp - 8*2], rax   ;; save as local-#1
 mov rax, [rbp - 8*2]   ;; use local-#1 as `five`
 push rax               ;; push arg <5>
 mov rax, [rbp - 8*-3]  ;; load `it`
 ;; CHECK FUNCTION
 ;; CHECK ARITY
 sub rax, 5             ;; remove TAG
 mov rax, [rax]         ;; load actual label of `it` into rax
 call rax               ;; call `it`
 add rsp, 8*1
fun_exit_anon_1:
 mov rsp, rbp
 pop rbp
 ret
fun_finish_anon_1:
 ;; allocate tuple for fun_start_anon_1
 mov rax, fun_start_anon_1
 mov [r11], rax         ;; save label
 mov rax, 1
 mov [r11 + 8], rax     ;; save arity = 1
 mov rax, [rbp - 8*2]
 mov [r11 + 16], rax    ;; save `five` in closure!
 mov rax, r11           ;; save tuple address
 add r11, 32            ;; bump allocation pointer (16-byte aligned)
 add rax, 5             ;; tag rax as "function"
 mov [rbp - 8*3], rax   ;; save `fn` as local-#2 `f`

;; block to define `inc`
jmp fun_finish_anon_2
fun_start_anon_2:
  push rbp
  mov rbp, rsp
  sub rsp, 8*105
fun_body_anon_2:
  mov rax, [rbp - 8*-2]
  add rax, 2
fun_exit_anon_2:
  mov rsp, rbp
  pop rbp
  ret
fun_finish_anon_2:
 ;; allocate tuple for fun_start_anon_2
  mov rax, fun_start_anon_2
  mov [r11], rax         ;; save label
  mov rax, 1
  mov [r11 + 8], rax     ;; save arity = 1
  mov rax, r11           ;; save tuple address
  add r11, 16            ;; bump allocation pointer
  add rax, 5             ;; tag rax as "function"
  mov [rbp - 8*4], rax   ;; save `fn` as local#3 `inc`

;; (f inc)
mov rax, [rbp - 8*4]    ;; push `inc` as arg
push rax
mov rax, [rbp - 8*3]    ;; load `f` tuple into rax
push rax                ;; push `f` as arg (WHY???)
;; CHECK function TAG
;; CHECK arity
sub rax, 5
mov rax, [rax]          ;; load actual label
call rax
add rsp, 8*1

mov rsp, rbp
pop rbp
ret
