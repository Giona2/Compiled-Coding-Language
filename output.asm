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
  push rax
  mov rax, 8
  mov QWORD [rbp-8], rax
  pop rax
  sub rsp, 8
  push rax
  mov rax, 8
  mov QWORD [rbp-16], rax
  pop rax
.end:
  mov rsp, rbp
  pop rbp
  ret
