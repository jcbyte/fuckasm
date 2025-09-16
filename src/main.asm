[BITS 64]

section .data
    msg: db 0 ; Printing buffer
    d: db 0, 0, 0, 0, 0, 0, 0, 0 ; data 

section .text
    global _start

_start:
    ; Initialise
    mov rbx, 0 ; data index

    ; Code
<GENERATED_CODE_HERE>
     
    ; Exit
    mov rax, 60       ; sys_exit
    mov rdi, 0        ; exit code 0
    syscall

; read 1 byte from stdin
read: 
    mov rax, 0           ; sys_read
    mov rdi, 0           ; stdin
    lea rsi, [d + rbx]   ; pointer to buffer
    mov rdx, 1           ; read 1 byte
    syscall

    ret

; print 1 byte to stdout
print:
    mov rax, 1         ; sys_write
    mov rdi, 1         ; stdout
    lea rsi, [d + rbx] ; pointer to buffer
    mov rdx, 1         ; write 1 byte
    syscall
    
    ret