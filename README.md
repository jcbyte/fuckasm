# fuckasm

An optimized Brainfuck to 64-bit x86-64 NASM compiler written in Rust.

## Usage

`fuckasm <INPUT> [-o OUTPUT]`

- `<INPUT>`: Path to Brainfuck source file.
- `OUTPUT`: Optional. Specify the output assembly file name. Defaults to '<INPUT>.asm'.

## Assembling

After generating the assembly file, you can assemble and run it on Linux:

```
nasm -f elf64 hello.asm -o hello.o
ld hello.o -o hello
./hello
```

## Licence

[Apache License 2.0](LICENSE)
