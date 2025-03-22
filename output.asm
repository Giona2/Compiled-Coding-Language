global _start
_start:
  call main
.exit:
  mov rdi, rax
  mov rax, 60
  syscall

add:
  push rbp
  mov rbp, rsp

  sub rsp, 8
  mov QWORD [rbp-8], rdi
  push rdi
  sub rsp, 8
  mov QWORD [rbp-16], rsi
  push rsi
  mov rdi, QWORD [rbp-8]
  mov rax, rdi
  mov rdi, QWORD [rbp-16]
  add rax, rdi
.end:
  pop rdi
  pop rsi
  mov rsp, rbp
  pop rbp
  ret

main:
  push rbp
  mov rbp, rsp

  sub rsp, 8
  push rax
  mov rdi, 1
  mov rsi, 3
  call add
  mov QWORD [rbp-8], rax
  pop rax
0
.end:
  mov rsp, rbp
  pop rbp
  ret
