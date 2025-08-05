# S-Expression Parser

A high-performance, zero-copy S-expression parser optimized for compiler use cases. Features fast parsing with minimal memory allocations through borrowed string slices and various performance optimizations.

## Features

- **Zero-copy parsing**: Uses borrowed string slices to avoid unnecessary allocations
- **Fast-path optimizations**: Optimized number parsing and single-character symbols
- **Production error handling**: Proper error types instead of panics
- **Memory efficient**: Pre-allocated vectors and optimized tokenization

## Quick Start

```rust
use sexpression::{Expression, read, ParseError};

fn main() -> Result<(), ParseError> {
    let source = "(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))";
    let expr = read(source)?;
    println!("Parsed: {:?}", expr);
    Ok(())
}
```

## Documentation

Run `cargo doc --open` to view the full documentation.
