//! Basic usage example for the S-expression parser
//! 
//! This example demonstrates both the original zero-copy API and the new
//! custom symbol functionality for owned expressions.

use sexpression::{Expression, OwnedExpression, StringOwnedSymbol, OwnedSymbol, read};
use std::fmt;

/// A simple custom symbol type for demonstration
#[derive(Debug, Clone, PartialEq)]
struct SimpleSymbol {
    name: String,
}

impl OwnedSymbol for SimpleSymbol {
    fn from_str(s: &str) -> Self {
        SimpleSymbol {
            name: s.to_string(),
        }
    }
    
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    println!("=== Basic Usage Example ===\n");
    
    // Example 1: Original zero-copy API
    println!("1. Zero-copy parsing (original API):");
    let source = "(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))";
    match read(source) {
        Ok(expr) => {
            println!("   Parsed: {:?}", expr);
            println!("   Display: {}", expr);
        }
        Err(e) => eprintln!("   Parse error: {}", e),
    }
    
    // Example 2: Converting to owned with default symbols
    println!("\n2. Converting to owned (default symbols):");
    let borrowed = Expression::Symbol("hello");
    let owned: OwnedExpression<StringOwnedSymbol> = borrowed.to_owned();
    println!("   Borrowed: {:?}", borrowed);
    println!("   Owned: {}", owned);
    
    // Example 3: Custom symbol types
    println!("\n3. Custom symbol types:");
    let custom_expr = OwnedExpression::<SimpleSymbol>::Symbol(
        SimpleSymbol::from_str("custom_symbol")
    );
    println!("   Custom symbol: {}", custom_expr);
    
    // Example 4: Complex expression with custom symbols
    println!("\n4. Complex expression with custom symbols:");
    let complex = OwnedExpression::<SimpleSymbol>::List(vec![
        OwnedExpression::Symbol(SimpleSymbol::from_str("define")),
        OwnedExpression::Symbol(SimpleSymbol::from_str("x")),
        OwnedExpression::Number(42.0),
    ]);
    println!("   Complex: {}", complex);
    
    // Example 5: Parsing and converting with custom symbols
    println!("\n5. Parsing and converting with custom symbols:");
    match read("(define x 42)") {
        Ok(borrowed) => {
            let owned: OwnedExpression<SimpleSymbol> = borrowed.to_owned();
            println!("   Parsed and converted: {}", owned);
        }
        Err(e) => println!("   Parse error: {}", e),
    }
    
    println!("\n=== Example Complete ===");
} 