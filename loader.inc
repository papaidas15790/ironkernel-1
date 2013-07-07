; flags
PM_32BIT    =   1
GRANULARITY =   1 shl 1
; access
ACCESSED    =   1 shl 0
_RW         =   1 shl 1
EXECUTE     =   1 shl 3
CODE_READ   =   EXECUTE or _RW ; allow read for code
DATA_RW     =   _RW            ; allow write for data
PRESENT     =   1 shl 7

macro gdt_entry base, limit, flags, access {
    dw limit and 0FFFFh
    dw base and 0FFFFh
    db (base shr 16) and 0FFh
    db (access and 0FFh) or (1 shl 4) or PRESENT
    db ((limit shr 16) and 1111b) or ((flags and 11b) shl 6)
    db (base shr 24) and 0FFh
}