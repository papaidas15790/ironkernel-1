format ELF

public abort
public abort as '_GLOBAL_OFFSET_TABLE_'
public abort as '__morestack'
public abort as 'memcmp'
public abort as 'memcpy'
public abort as 'malloc'
public abort as 'free'
public start

extrn main

section '.text' executable

start:
    ; rust functions compare esp against [gs:0x30] as a sort of stack guard thing
    ; as long as we set [gs:0x30] to dword 0, it should be ok
    mov [gs:0x30], dword 0
    ; clear the screen a slightly different colour
    mov edi, 0xb8000
    mov ecx, 80*25*2
    mov al, 1
    rep stosb
    ; jump into rust
    call main
abort:
    jmp $