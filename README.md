# form-repl

A REPL (Read-Eval-Print Loop) for FORM symbolic manipulation system, implemented in Rust.

## Overview

FORM is a computer algebra system specialized for symbolic manipulation, particularly useful for high-energy physics calculations. This project provides an interactive REPL interface for working with FORM expressions.

## Features

- **Interactive REPL**: Read-Eval-Print loop for interactive symbolic computation
- **Symbolic manipulation**: Work with symbols, expressions, and algebraic operations
- **Pattern matching**: Define substitution rules using `id` statements
- **Expression simplification**: Automatic simplification of arithmetic expressions
- **Built-in functions**: Support for mathematical functions (sin, cos, exp, log)
- **Command history**: Navigate through command history using arrow keys

## Installation

Make sure you have Rust installed (https://rustup.rs/), then:

```bash
cargo build --release
```

The binary will be available at `target/release/form-repl`.

## Usage

Run the REPL:

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/form-repl
```

## Examples

### Basic arithmetic

```
FORM> 2 + 3
  5
FORM> (1 + 2) ^ 3
  27
FORM> 5 * 4
  20
```

### Symbolic expressions

```
FORM> Symbols x, y
  Symbols declared
FORM> Expression e = (x + 1) * (x - 1)
  e = ((x + 1) * (x - 1))
```

### Substitution rules

```
FORM> Symbols x, y
  Symbols declared
FORM> Expression e = x + y
  e = (x + y)
FORM> id x = 1
  Rule added: x -> 1
FORM> id y = 2
  Rule added: y -> 2
FORM> .sort
  Sorted and rules applied
FORM> Print e
  e = 3
```

### Working with expressions

```
FORM> Symbols x
  Symbols declared
FORM> Local f = x^2 + 2*x + 1
  f = ((x ^ 2) + ((2 * x) + 1))
FORM> id x = 3
  Rule added: x -> 3
FORM> .sort
  Sorted and rules applied
FORM> Print f
  f = 16
```

## Commands

- `quit` or `exit` - Exit the REPL
- `help` - Show help message
- `clear` - Clear all definitions and rules

## Syntax

- `Symbols x, y, z` - Declare symbols
- `Expression name = expr` - Define a named expression
- `Local name = expr` - Define a local variable (same as Expression)
- `id pattern = replacement` - Add a substitution rule
- `Print name` - Print the value of an expression
- `.sort` - Apply all substitution rules and simplify expressions

## Operators

- `+` - Addition
- `-` - Subtraction
- `*` - Multiplication
- `/` - Division
- `^` - Exponentiation

## Running Tests

```bash
cargo test
```

## License

See LICENSE file for details.

