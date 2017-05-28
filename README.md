# xtensa2arm

Experimental, very simple binary translator from xtensa to ARM. Actual binary format parsing and decoding is handled by radare2. Output is supposed to be compiled with GCC. Compiled binaries can then be passed to a decompiler. This way, decompilation of xtensa binaries can be performed.

As of now, it's in a very early stage and doesn't work most of the time.

Example input:
```
.global sdk_rom_i2c_writeReg;
sdk_rom_i2c_writeReg:
    slli a8, a5, 16
    slli a7, a4, 8
    l32r a9, 0x4021c0cc ; a9=0x60000a00
    or a7, a7, a8
    l32r a8, 0x4020a194 ; a8=0x1000000
    or a7, a2, a7
    or a7, a7, a8
    slli a8, a3, 2
    add.n a2, a8, a9
    memw
    s32i a7, a2, 0x300
    memw
    l32i a6, a2, 0x300
    bbci a6, 25, 0x40224a56
0x40224a4d:
    memw
    l32i a9, a2, 0x300
    bbsi a9, 25, 0x40224a4d
0x40224a56:
    ret.n
```

Example output:
```
.global sdk_rom_i2c_writeReg;
sdk_rom_i2c_writeReg:
	lsl r6, r3, #16
	lsl r5, r2, #8
	ldr r7, =0x60000a00
	orr r5, r5, r6
	ldr r6, =0x1000000
	orr r5, r0, r5
	orr r5, r5, r6
	lsl r6, r1, #2
	add r0, r6, r7
	str r5, [r0, #0x300]
	ldr r4, [r0, #0x300]
	tst r4, #0x2000000
	beq loc_40224a56
loc_40224a4d:
	ldr r7, [r0, #0x300]
	tst r7, #0x2000000
	bne loc_40224a4d
loc_40224a56:
	bx lr
```
