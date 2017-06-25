.text
.literal_position
.align 4

.global __entry
.type __entry, @function
__entry:
    movi a1, __stack
    call0 main

all_done:
    nop
    j all_done
