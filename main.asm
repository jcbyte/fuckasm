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
    inc rbx                 ; >
    dec rbx                 ; <
    inc byte [d + rbx]      ; +
    dec byte [d + rbx]      ; -
    call read               ; ,
    call print              ; .
    loop_start_0:           ; [
      cmp byte [d + rbx], 0
      je loop_end_0
      ; loop body
      jmp loop_start_0
    loop_end_0:             ; ]
     
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