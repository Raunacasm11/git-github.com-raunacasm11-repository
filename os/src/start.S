.section .text._start
.global _start
_start:
    # Set up stack pointer
    la sp, boot_stack_top
    
    # Call Rust main function
    call rust_main
    
    # In case rust_main returns, loop forever
    j .

.section .bss.stack
.align 4
.global boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16  # 64KB stack
.global boot_stack_top
boot_stack_top:
