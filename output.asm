global _start
_start:
  call main
.exit:
  mov rdi, rax
  mov rax, 60
  syscall

main:
  push rbp
  mov rbp, rsp

  sub rsp, 8
  mov rdi, 3
  mov rax, rdi
  mov QWORD [rbp-8], rax

  mov rdi, QWORD [rbp-8]  ; Perform the equation using the rdi and rax registers
  mov rax, rdi
  mov rdi, 1
  add rax, rdi
  mov rdi, rax            ; Put the final calculation into the rdi register
  mov rax, rdi            ; Put the result into the rax register
  mov QWORD [rbp-8], rax  ; Put what was calculated from the rax register into the accociated slot

  mov rdi, QWORD [rbp-8]
  mov rax, rdi
.end:
  mov rsp, rbp
  pop rbp
  ret
