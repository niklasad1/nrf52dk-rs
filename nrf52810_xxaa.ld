_sflashdata = LOADADDR(.data);

MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 512K
    RAM  (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}

__stack_size__ = DEFINED(__stack_size__) ? __stack_size__ : 0x1000;

SECTIONS {
    . = 0x00000000;
    .text : {
        /* Put the vector table first */
        KEEP(*(.vectors .vectors.*))
        
        /* IRQ Table, chip specific */
        KEEP(*(.irqs))
        
        /* text and rodata holds program code and constants */
        *(.startup*)
        *(.text*)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    .data : {
        . = ALIGN(4);
        _sdata = .;
        *(.fastrun*)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > FLASH

    .bss : {
        . = ALIGN(4);
        _sbss = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    _estack = ORIGIN(RAM) + LENGTH(RAM); /* stack pointer start */
}
