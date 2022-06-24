# asmjr
Tiny assembler + cart builder for ECJR

## Example usage
To build the example program `dvdlogo.asm` into a cartridge, with `dvdlogo.png` as the video rom, run (if you are on Linux I assume you understand how to run a binary with `./`):
```
asmjr.exe -i dvdlogo.png dvdlogo.asm dvdlogo.cart
```

Help:
```
asmjr.exe --help

asmjr 0.1.0

USAGE:
    asmjr.exe [OPTIONS] <SOURCE> [OUTPUT]

ARGS:
    <SOURCE>    Assembly source file
    <OUTPUT>    Output ECJR cartridge file

OPTIONS:
        --author <AUTHOR>        Author to embed into metadata
    -h, --help                   Print help information
    -i, --imagerom <IMAGEROM>    Load image (red channel only) into rom
    -l, --listing                Dump out ops to terminal
    -m, --message <MESSAGE>      Simple message to embed in metadata
    -r, --rawrom <RAWROM>        Load raw bytes into rom
        --readme <README>        Readme file to embed in metadata
    -u, --uncompressed           Leave cart body uncompressed
    -V, --version                Print version information
```

## Assembly Language
The included assembler is extremely minimal. This snippet covers basically all the syntax:
```
// Comments can be like this,
#  or like this if you want.
#  Comments must be on their own lines, the parser is line-sensitive

// Constants can be declared like this
const SPRITES = 0x200
const TAU = 6.28318
const can_be_lowercase_or_whatever = 0b1010101
const Ω = 10000

// The standard ECJR memory addresses are builtin constants
li x1, 64
store x1, zero, VIDEO_SPRITE_COUNT

// Opcodes look like this (can be uppercase or lowercase)
li x1, 12
LI x2, Ω
nop
add x1, x1, x2
jal zero, LABEL_DECLARED_LATER

// the builtin register names are
// zero x1 x2 x3 x4 ... x255
// but you can create aliases
reg temp = x5
muli temp, temp, 2

// Labels like this 
LABEL_DECLARED_LATER:
anything-goes-with-labels-too:

// files must end with a newline

```

The Eclipse/Snakefield ISA is beyond the scope of this document, see the ECJR emulator documentation.

## Building
This builds in the normal way with cargo (e.g., `cargo build --release`) *however* you will need to have
the Google protocol buffer compiler (`protoc`) on your path because prost needs it (see: https://grpc.io/docs/protoc-installation/).
