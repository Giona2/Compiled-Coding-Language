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
  mov rax, 9
  mov QWORD [rbp-8], rax
  pop rax
  mov rax, QWORD [rbp-8]
  imul rax, 3
.end:
  mov rsp, rbp
  pop rbp
  ret
