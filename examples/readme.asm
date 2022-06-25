// This file merely demonstrates syntax, the program is nonsense
// and does nothing.

// Comments can be like this,
#  or like this if you want.
;  or like this

// Constants can be declared like this
const SPRITES = 0x200 // comments can be on the same line
const TAU = 6.28318
const can_be_lowercase_or_whatever = 0b1010101
const Ω = 10000

// the builtin register names are
// x0 x1 x2 x3 x4 ... x255
// zero (always immutably zero, x0)
// ra (return address, x1)
// sp (stack pointer, x2)
// gp (global pointer, x3)
// tp (thread pointer, x4)
addi sp, sp, 1
jal ra, anything-goes-with-labels-too

// you can create aliases to refer to registers
reg temp = x5
muli temp, temp, 2

// Opcodes look like this (can be uppercase or lowercase)
li x1, 12
LI x2, Ω
nop
add x1, x1, x2
jal zero, LABEL_DECLARED_LATER

// the memory operations (load/store) have an alternate syntax too
store x1, zero, SPRITES
store x1, zero[SPRITES]
load x2, gp[SPRITES]

// Known ECJR memory addresses are builtin constants prefixed with $
li x1, 64
store x1, zero[$VIDEO_SPRITE_COUNT]

// Jump/branch labels are declared like this 
LABEL_DECLARED_LATER:
anything-goes-with-labels-too:

// Unlike constants, labels can be used before declaration