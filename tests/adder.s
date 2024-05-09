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
 mov r11, rsi               ;; save start of heap in r11
 jmp fun_finish_add
         align 16
         fun_start_add:
           push rbp
         mov rbp, rsp
         sub rsp, 8*101
         fun_body_add:
           mov rax, [rbp - 8*-2]
         fun_exit_add:
           mov rsp, rbp
             pop rbp
             ret
         align 16
         fun_finish_add:
         mov rax, fun_start_add
         mov [rbp - 8*2], rax
         mov rax, 20
                 mov [rbp - 8*3], rax
mov rax, 40
                 mov [rbp - 8*4], rax
                 mov rcx, [rbp - 8*4]
             push rcx
mov rcx, [rbp - 8*3]
             push rcx
                 call fun_start_add
                 add rsp, 8*2
 mov rsp, rbp
             pop rbp
             ret
time_to_exit:
  ret
