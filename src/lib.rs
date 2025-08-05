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
//! - **Custom symbol types**: Trait-based system for custom symbol representations in owned expressions
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
//! - [`OwnedExpression`]: Owned version with custom symbol support
//! - [`OwnedSymbol`]: Trait for custom symbol types
//! - [`StringOwnedSymbol`]: Default string-based symbol implementation
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
//! ## Custom Symbol Types
//! 
//! ```rust
//! use sexpression::{OwnedSymbol, OwnedExpression, StringOwnedSymbol};
//! use std::fmt;
//! 
//! #[derive(Debug, Clone, PartialEq)]
//! struct CustomSymbol {
//!     name: String,
//!     namespace: Option<String>,
//! }
//! 
//! impl OwnedSymbol for CustomSymbol {
//!     fn from_str(s: &str) -> Self {
//!         if let Some((ns, name)) = s.split_once("::") {
//!             CustomSymbol {
//!                 name: name.to_string(),
//!                 namespace: Some(ns.to_string()),
//!             }
//!         } else {
//!             CustomSymbol {
//!                 name: s.to_string(),
//!                 namespace: None,
//!             }
//!         }
//!     }
//!     
//!     fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         match &self.namespace {
//!             Some(ns) => write!(f, "{}::{}", ns, self.name),
//!             None => write!(f, "{}", self.name),
//!         }
//!     }
//! }
//! 
//! // Use with custom symbol type
//! let expr = OwnedExpression::<CustomSymbol>::Symbol(CustomSymbol::from_str("std::vector"));
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
//! use sexpression::{Expression, OwnedExpression, StringOwnedSymbol, OwnedSymbol};
//! 
//! let borrowed = Expression::Symbol("hello");
//! let owned: OwnedExpression<StringOwnedSymbol> = borrowed.to_owned();
//! assert_eq!(owned, OwnedExpression::Symbol(StringOwnedSymbol::from_str("hello")));
//! ```

pub mod reader;

// Re-export main types and functions for easy access
pub use crate::reader::{
    Expression,
    OwnedExpression, 
    OwnedSymbol,
    StringOwnedSymbol,
    ParseError,
    read,
    read_unchecked,
};
