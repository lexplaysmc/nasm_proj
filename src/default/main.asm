DEFAULT REL

global main

extern printf

section .text
main:
    push    rbp         ; Function prologue
    mov     rbp, rsp

    sub     rsp, 8*4    ; Allocate shadow-space for printf

    mov     rcx, message; &message is printf's first argument
    call    printf      ; Call printf

    add     rsp, 8*4    ; Remove shadow-space

    pop     rbp         ; Function epilogue
    ret                 ; Return

section .data
message:    db "Hello world!", 13, 10, 0 ; 13, 10, 0 is \r\n and null terminator