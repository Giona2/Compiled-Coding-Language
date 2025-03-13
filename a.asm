global _start
_start:
  push rbp
  mov rbp, rsp

  sub rsp, 8
  push rax
  mov rax, __float64__(10.0)
  movq xmm0, rax
  mov rax, __float64__(6.0)
  movq xmm1, rax
  divsd xmm0, xmm1
  movq rax, xmm0
  mov QWORD [rbp-8], rax
  pop rax

.exit:
  mov rsp, rbp
  pop rbp
  mov rax, 60
  mov rdi, 0
  syscall