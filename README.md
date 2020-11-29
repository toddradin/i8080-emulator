# i8080
An emulator of the Intel 8080 processor written in Rust.

# i8080-tests
To run tests against this emulator, execute 
```
cargo run --release
```
from the i8080-tests directory. The test roms are taken from [here](https://altairclone.com/downloads/cpu_tests/) and the following roms pass:
- TST8080.COM
- CPUTEST.COM
- 8080PRE.COM
- 8080EXM.COM

```
i8080-emulator/i8080-tests$ cargo run --release
======================
EXECUTING TEST: test-roms/TST8080.COM
MICROCOSM ASSOCIATES 8080/8085 CPU DIAGNOSTIC
 VERSION 1.0  (C) 1980

 CPU IS OPERATIONAL

======================
EXECUTING TEST: test-roms/CPUTEST.COM
DIAGNOSTICS II V1.2 - CPU TEST
COPYRIGHT (C) 1981 - SUPERSOFT ASSOCIATES

ABCDEFGHIJKLMNOPQRSTUVWXYZ
CPU IS 8080/8085
BEGIN TIMING TEST
END TIMING TEST
CPU TESTS OK


======================
EXECUTING TEST: test-roms/8080PRE.COM
8080 Preliminary tests complete

======================
EXECUTING TEST: test-roms/8080EXM.COM
8080 instruction exerciser

dad <b,d,h,sp>................  PASS! crc is:14474ba6
aluop nn......................  PASS! crc is:9e922f9e
aluop <b,c,d,e,h,l,m,a>.......  PASS! crc is:cf762c86
<daa,cma,stc,cmc>.............  PASS! crc is:bb3f030c
<inr,dcr> a...................  PASS! crc is:adb6460e
<inr,dcr> b...................  PASS! crc is:83ed1345
<inx,dcx> b...................  PASS! crc is:f79287cd
<inr,dcr> c...................  PASS! crc is:e5f6721b
<inr,dcr> d...................  PASS! crc is:15b5579a
<inx,dcx> d...................  PASS! crc is:7f4e2501
<inr,dcr> e...................  PASS! crc is:cf2ab396
<inr,dcr> h...................  PASS! crc is:12b2952c
<inx,dcx> h...................  PASS! crc is:9f2b23c0
<inr,dcr> l...................  PASS! crc is:ff57d356
<inr,dcr> m...................  PASS! crc is:92e963bd
<inx,dcx> sp..................  PASS! crc is:d5702fab
lhld nnnn.....................  PASS! crc is:a9c3d5cb
shld nnnn.....................  PASS! crc is:e8864f26
lxi <b,d,h,sp>,nnnn...........  PASS! crc is:fcf46e12
ldax <b,d>....................  PASS! crc is:2b821d5f
mvi <b,c,d,e,h,l,m,a>,nn......  PASS! crc is:eaa72044
mov <bcdehla>,<bcdehla>.......  PASS! crc is:10b58cee
sta nnnn / lda nnnn...........  PASS! crc is:ed57af72
<rlc,rrc,ral,rar>.............  PASS! crc is:e0d89235
stax <b,d>....................  PASS! crc is:2b0471e9
Tests complete

```

# space-invaders
A Space Invaders emulator, written in Rust and uses [SDL2](http://libsdl.org/download-2.0.php) for display rendering and [SDL2_mixer](https://www.libsdl.org/projects/SDL_mixer/) for sound. These must be downloaded and installed on your machine.

## Build
Place the following rom files into the ```space-invaders/roms/``` directory:
- invaders.e
- invaders.f
- invaders.g
- invaders.h

Place the following sound files into the ```space-invaders/sounds/``` directory:
- ufo.wav
- shoot.wav
- player_death.wav
- invader_death.wav
- invader1.wav
- invader2.wav
- invader3.wav
- invader4.wav

Then run: 
```
cargo run --release
```
from the space-invaders directory to begin emulation of Space Invaders.

## Controls
Key | Action
--- | ---
0 | Insert a coin
1 | Start a game in one-player mode
2 | Start a game in two-player mode
A | Move player one left
D | Move player one right
W | Player one shoot
J | Move player two left
L | Move player two right
I | Player two shoot
ESC | Exit the game


# Resources
- [Intel 8080 Assembly Language Programming Manual](https://altairclone.com/downloads/manuals/8080%20Programmers%20Manual.pdf)
- [Intel 8080 Datasheet](http://kazojc.com/elementy_czynne/IC/INTEL-8080A.pdf)
- [Intel 8080 Opcodes](https://pastraiser.com/cpu/i8080/i8080_opcodes.html)
- [Emulator 101](http://www.emulator101.com)
