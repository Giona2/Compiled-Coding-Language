global _start

section .text
_start:
	; Function start.
	; rbp is the pointer to the top of the stack
	; the value in rbp is saved
	push    rbp
	mov     rbp, rsp

	; Declare and assign variable
	sub rsp, 8
	mov QWORD [rbp-8], 2

	; Function end. Return all to their original values
	mov rsp, rbp
	pop rbp

	; Exit the program
	mov rax, 60
	mov rdi, 0
	syscall
