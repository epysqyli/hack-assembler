# Hack Assembler
This is a simple hack assembler written in Rust.

Hack is the machine code used in the [From Nand To Tetris](https://www.nand2tetris.org/course) course.


# Example

| ASM                                | HACK             |
|------------------------------------|------------------|
| @R0                                | 0000000000000000 |
| D=M                                | 1111110000010000 |
| @ZERO_OUTPUT                       | 0000000000011100 |
| D;JEQ                              | 1110001100000010 |
| @R1                                | 0000000000000001 |
| D=M                                | 1111110000010000 |
| @ZERO_OUTPUT                       | 0000000000011100 |
| D;JEQ                              | 1110001100000010 |
| @R2                                | 0000000000000010 |
| M=0                                | 1110101010001000 |
| @R1                                | 0000000000000001 |
| D=M                                | 1111110000010000 |
| @i                                 | 0000000000010000 |
| M=D                                | 1110001100001000 |
| (LOOP) // add R0 to itself i times |                  |
|   @i                               | 0000000000010000 |
|   D=M                              | 1111110000010000 |
|   @END                             | 0000000000100010 |
|   D;JEQ                            | 1110001100000010 |
|   @R2                              | 0000000000000010 |
|   D=M                              | 1111110000010000 |
|   @R0                              | 0000000000000000 |
|   D=D+M                            | 1111000010010000 |
|   @R2                              | 0000000000000010 |
|   M=D                              | 1110001100001000 |
|   @i                               | 0000000000010000 |
|   M=M-1                            | 1111110010001000 |
|   @LOOP                            | 0000000000001110 |
|   0;JMP                            | 1110101010000111 |
| (ZERO_OUTPUT)                      |                  |
|   @0                               | 0000000000000000 |
|   D=A                              | 1110110000010000 |
|   @R2                              | 0000000000000010 |
|   M=D                              | 1110001100001000 |
|   @END                             | 0000000000100010 |
|   0;JMP                            | 1110101010000111 |
| (END)                              |                  |
|   @END                             | 0000000000100010 |
|   0;JMP                            | 1110101010000111 |
