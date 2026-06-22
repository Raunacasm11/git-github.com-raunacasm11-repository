OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Kernel load address for RISC-V QEMU */
. = 0x80000000;

/* Code section + Trampoline page alignment */
.text : {
    *(.text.entry)
    . = ALIGN(4K);
    strampoline = .;
    *(.text.trampoline);
    . = ALIGN(4K);
    *(.text .text.*)
}

/* Read-only data */
.rodata ALIGN(4K) : {
    *(.rodata .rodata.*)
}

/* Initialized data */
.data ALIGN(4K) : {
    *(.data .data.*)
}

/* BSS Section: Define sbss, ebss (fix missing symbols) */
.bss ALIGN(4K) : {
    sbss = .;
    *(.bss .bss.*)
    *(COMMON)
    ebss = .;
}

/* Kernel end symbol (for physical frame allocator) */
ekernel = .;