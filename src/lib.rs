//! S-Expression Parser Library
//! 
//! A high-performance, zero-copy S-expression parser optimized for compiler use cases.
//! This library provides fast parsing with minimal memory allocations through borrowed
//! string slices and various performance optimizations.
//! 
//! # Features
//! 
//! - **Zero-copy parsing**: Uses borrowed string slices to avoid unnecessary allocations
//! - **Fast-path optimizations**: Optimized number parsing and single-character symbols
//! - **Production error handling**: Proper error types instead of panics
//! - **Memory efficient**: Pre-allocated vectors and optimized tokenization
//! - **Compiler-friendly**: Designed for use in language compilers and interpreters
//! 
//! # Quick Start
//! 
//! ```rust
//! use sexpression::{Expression, read, ParseError};
//! 
//! fn main() -> Result<(), ParseError> {
//!     let source = "(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))";
//!     let expr = read(source)?;
//!     println!("Parsed: {:?}", expr);
//!     Ok(())
//! }
//! ```
//! 
//! # API Overview
//! 
//! ## Core Types
//! 
//! - [`Expression`]: Zero-copy S-expression representation
//! - [`OwnedExpression`]: Owned version for independent storage
//! - [`ParseError`]: Comprehensive error types
//! 
//! ## Main Functions
//! 
//! - [`read`]: Primary parsing function with error handling
//! - [`read_unchecked`]: Convenience function that panics on error
//! 
//! # Performance
//! 
//! The parser is optimized for:
//! 
//! - **Memory efficiency**: Zero-copy parsing with borrowed slices
//! - **Speed**: Fast-path checks and pre-allocated vectors
//! - **Compiler workloads**: Designed for parsing large amounts of code
//! 
//! # Examples
//! 
//! ## Basic Parsing
//! 
//! ```rust
//! use sexpression::{read, Expression};
//! 
//! let result = read("(define x 42)");
//! match result {
//!     Ok(expr) => println!("Success: {:?}", expr),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//! 
//! ## Error Handling
//! 
//! ```rust
//! use sexpression::{read, ParseError};
//! 
//! let result = read("(unclosed");
//! assert!(matches!(result, Err(ParseError::MissingClosingParen)));
//! ```
//! 
//! ## Converting to Owned
//! 
//! ```rust
//! use sexpression::{Expression, OwnedExpression};
//! 
//! let borrowed = Expression::Symbol("hello");
//! let owned = borrowed.to_owned();
//! assert_eq!(owned, OwnedExpression::Symbol("hello".to_string()));
//! ```

pub mod reader;

// Re-export main types and functions for easy access
pub use crate::reader::{
    Expression,
    OwnedExpression, 
    ParseError,
    read,
    read_unchecked,
};
