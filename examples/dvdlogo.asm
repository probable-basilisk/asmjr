// Shows three spinning dvd logos
// (make sure to use dvdlogo.png as the videorom!)

// the first row of memory 0x0000 - 0x0100 has special meanings,
// but other than that we can use anything else 0x0100 - 0xffff freely
const SPRITES = 0x200
const LOGOW = 128
const HALFW = 64
const LOGOH = 74
const HALFH = 37
const TAU = 6.28318

// reconfigure for three cores
li x1, 3
crcfg zero, x1

// have only core zero do setup
crid x1
bne zero, x1, NOT_CORE_ZERO

// enable video and three sprites, point sprite buffer to 
// the area of memory we 'claimed' above
li x1, 1
store x1, zero, VIDEO_ENABLE
li x1, 3
store x1, zero, VIDEO_SPRITE_COUNT
li x1, SPRITES
store x1, zero, VIDEO_SPRITE_BUFFER_ADDR

NOT_CORE_ZERO:

// give some more convenient names to registers
reg cursprite = x15
reg theta = x14
reg dtheta = x13
reg xpos = x12
reg ypos = x11

// each sprite has twelve parameters
// cursprite = SPRITES + 12*coreid
crid cursprite
muli cursprite, cursprite, 12
addi cursprite, cursprite, SPRITES

// read sprite out of vrom (ycoord >= 256)
li x1, 256
store x1, cursprite, 1

// cursprite[2], [6] = LOGOW
li x1, LOGOW
store x1, cursprite, 2
store x1, cursprite, 6

// cursprite[3], [7] = LOGOH
li x1, LOGOH
store x1, cursprite, 3
store x1, cursprite, 7

// fg color = coreid * 80 + 60
crid x1
muli x1, x1, 80
addi x1, x1, 60
store x1, cursprite, 8

// dtheta = (coreid + 1) * 0.0123
crid dtheta
addi dtheta, dtheta, 1
muli dtheta, dtheta, 0.0123

FRAMELOOP:
add theta, theta, dtheta
cos xpos, theta
muli xpos, xpos, 40
addi xpos, xpos, 128
subi xpos, xpos, HALFW
sin ypos, theta
muli ypos, ypos, 40
addi ypos, ypos, 96
subi ypos, ypos, HALFH

store xpos, cursprite, 4
store ypos, cursprite, 5

// kill with condition 1 means 'yield until next frame'
kill zero, 1
jal zero, FRAMELOOP
