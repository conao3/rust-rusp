# rusp

A Lisp interpreter written in Rust.

## Overview

rusp is a minimal Lisp implementation featuring an interactive REPL with command history, lexically scoped environments, and support for both integers and floating-point arithmetic.

## Features

- Interactive REPL with persistent history
- First-class functions and lambda expressions
- Lexical scoping with nested environments
- Integer and floating-point number support
- Strings, symbols, and keywords
- Cons cells and proper list handling

## Installation

```bash
git clone https://github.com/conao3/rust-rusp.git
cd rust-rusp
cargo build --release
```

## Usage

Start the REPL:

```bash
cargo run
```

### Examples

```lisp
rusp> (+ 1 2 3)
6

rusp> (setq x 10)
10

rusp> (* x 2)
20

rusp> ((lambda (n) (* n n)) 5)
25

rusp> (if (< 1 2) "yes" "no")
"yes"
```

## Built-in Functions

| Function | Description |
|----------|-------------|
| `+`, `-`, `*`, `/` | Arithmetic operations |
| `=`, `/=`, `<`, `<=`, `>`, `>=` | Numeric comparisons |
| `if` | Conditional expression |
| `set`, `setq` | Variable assignment |
| `quote` | Return expression unevaluated |
| `lambda` | Create anonymous function |
| `apply` | Apply function to arguments |

## Requirements

- Rust (nightly channel required for `is_some_with` feature)

## License

Apache License 2.0
