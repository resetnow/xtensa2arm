
ENTRY(__entry);

MEMORY
{
    imem (RWX) : ORIGIN = 0x70000000, LENGTH = 0x100000
    dmem (RW)  : ORIGIN = 0x80000000, LENGTH = 0x100000
    io   (RW)  : ORIGIN = 0x60000000, LENGTH = 0x100000
}

SECTIONS
{
   .text : { *(.text) } > imem
   .data : { *(.data) } > dmem
   .bss : { *(.bss) } > dmem
   .mmio : { *(.mmio) } > io
}
