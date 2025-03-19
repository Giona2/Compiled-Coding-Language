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
.end:
  mov rsp, rbp
  pop rbp
  ret

add:
  push rbp
  mov rbp, rsp
.end:
  mov rsp, rbp
  pop rbp
  ret
