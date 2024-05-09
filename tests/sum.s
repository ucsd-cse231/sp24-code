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
         sub rsp, 8*102
 mov [rbp - 8], rdi
 mov r11, rsi               ;; save start of heap in r11
 jmp fun_finish_sum
         align 16
         fun_start_sum:
           push rbp
         mov rbp, rsp
         sub rsp, 8*103
         fun_body_sum:
           mov rax, [rbp - 8*-2]
                 mov [rbp - 8*1], rax
                 mov rax, 0
                 cmp rax, [rbp - 8*1]
                 mov rax, 3
                 jne eq_exit_2
                 mov rax, 7
               eq_exit_2:

                      cmp rax, 3
                      je label_else_2
                      mov rax, 0
                      jmp label_exit_2
                    label_else_2:
                      mov rax, [rbp - 8*-2]
                 mov rcx, rax
             and rcx, 1
             cmp rcx, 0
             mov rdi, 99
             jne label_error
                 mov [rbp - 8*1], rax
                 mov rax, [rbp - 8*-2]
                 mov rcx, rax
             and rcx, 1
             cmp rcx, 0
             mov rdi, 99
             jne label_error
                 mov [rbp - 8*2], rax
                 mov rax, -2
                 mov rcx, rax
             and rcx, 1
             cmp rcx, 0
             mov rdi, 33
             jne label_error
                 add rax, [rbp - 8*2]

                 mov [rbp - 8*2], rax
                 mov rcx, [rbp - 8*2]
             push rcx
                 call fun_start_sum
                 add rsp, 8*1
                 mov rcx, rax
             and rcx, 1
             cmp rcx, 0
             mov rdi, 33
             jne label_error
                 add rax, [rbp - 8*1]

                    label_exit_2:
         fun_exit_sum:
           mov rsp, rbp
             pop rbp
             ret
         align 16
         fun_finish_sum:
         mov rax, fun_start_sum
mov [rbp - 8*2], rax
mov rax, 2
                 mov [rbp - 8*3], rax
                 mov rcx, [rbp - 8*3]
             push rcx
                 call fun_start_sum
                 add rsp, 8*1
 mov rsp, rbp
             pop rbp
             ret
time_to_exit:
  ret
