
ENTRY(__entry);

PROVIDE(__stack = 0x20100000);
PROVIDE(__test_pass = 0x20100000);

MEMORY
{
    imem (RWX) : ORIGIN = 0x10000000, LENGTH = 0x100000
    dmem (RW)  : ORIGIN = 0x20000000, LENGTH = 0x100000
    tmem (RW)  : ORIGIN = 0x20100000, LENGTH = 0x100
}

SECTIONS
{
   .text : { *(.text) } > imem
   .data : { *(.data) } > dmem
   .bss : { *(.bss) } > dmem
   .test : { *(.test) } > tmem
}
