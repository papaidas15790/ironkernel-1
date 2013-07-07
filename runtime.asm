format ELF

public _GLOBAL_OFFSET_TABLE_
public __morestack
public abort
public memcmp
public memcpy
public malloc
public free
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
    jmp $

_GLOBAL_OFFSET_TABLE_:

__morestack:

abort:
    jmp $

memcmp:
    jmp $

memcpy:
    jmp $

malloc:
    jmp $

free:
    jmp $
