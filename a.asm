global _start
_start:
  push rbp
  mov rbp, rsp

  sub rsp, 8
  push rax
  mov rax, 10
  mov QWORD [rbp-8], rax
  pop rax
  sub rsp, 8
  push rax
  mov rax, 25
  mov QWORD [rbp-16], rax
  pop rax
  sub rsp, 8
  push rax
  mov rax, QWORD [rbp-8]
  mul QWORD [rbp-16]
  mov QWORD [rbp-24], rax
  pop rax

.exit:
  mov rsp, rbp
  pop rbp
  mov rax, 60
  mov rdi, 0
  syscall