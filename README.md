# brainfuck-lamina

Brainfuck Compiler Powered by Lamina

## About Brainfuck

Brainfuck is an esoteric, minimalist programming language created by Urban Müller in 1993. It's designed to be extremely simple while still being Turing-complete, meaning it can theoretically perform any computation that any other programming language can.

### Language Specification

Brainfuck operates on an array of memory cells (typically 30,000 cells) and a data pointer that moves along this array. Each memory cell holds a single byte (0-255), with values wrapping around on overflow/underflow.

#### Commands

Brainfuck uses only 8 commands, each represented by a single character:

- `>` - Increment the data pointer (move right)
- `<` - Decrement the data pointer (move left)
- `+` - Increment the byte at the data pointer
- `-` - Decrement the byte at the data pointer
- `.` - Output the byte at the data pointer as an ASCII character
- `,` - Input an ASCII character and store it at the data pointer
- `[` - If the byte at the data pointer is zero, jump forward to the matching `]`
- `]` - If the byte at the data pointer is nonzero, jump back to the matching `[`

All other characters are ignored and can be used as comments.

#### Memory Model

- **Tape**: Linear array of bytes (usually 30,000 cells)
- **Data Pointer**: Points to current cell, starts at position 0
- **Cell Values**: 8-bit bytes (0-255), wrap around on overflow/underflow
- **Control Flow**: Loop constructs using `[` and `]` for conditional execution

#### File Extensions

- `.b`
- `.bf`

### Compiler Backend

This project uses [Lamina](https://docs.rs/lamina/latest/lamina/) as the compiler backend. Lamina provides the compilation infrastructure and optimization capabilities needed to efficiently translate Brainfuck programs into executable code.

For more information about Lamina, see:
- [Lamina Crate Documentation](https://docs.rs/crate/lamina/latest)
- [Lamina API Reference](https://docs.rs/lamina/latest/lamina/index.html)

### Examples

#### Hello World

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

#### Adding Two Values

```brainfuck
,>++++++[<-------->-],[<+>-]<.
```

This program reads two characters, adds their ASCII values, and outputs the result.
