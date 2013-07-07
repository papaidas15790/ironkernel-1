CC=gcc -m32
LD=ld -melf_i386
RUSTC=rustc
ASM=fasm
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -O --target i386-intel-linux --lib -o $@ -c $<

.asm.o:
	$(ASM) $< $@

main.rs: zero.rs

floppy.img: loader.bin main.bin
	cat $^ > $@

loader.bin: loader.asm
	$(ASM) $< $@

main.bin: linker.ld runtime.o main.o
	$(LD) -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img
