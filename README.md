# tinybrain

A `brainfuck` interpreter written in Rust.

## Overview

Tinybrain is a small interpreter to execute `brainfuck` code. To learn more about `brainfuck`, please visit [https://en.wikipedia.org/wiki/Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)

## Usage

```rust
use tinybrain;

fn main() {
    let result = tinybrain.process("
    ++++++++[>++++[>++>+++>+++>+<<<<-]
    >+>+>->>+[<]<-]>>.>---.+++++++..++
    +.>>.<-.<.+++.------.--------.>>+.>
    ++.
    ");

    println!("{}", result);
}
```
