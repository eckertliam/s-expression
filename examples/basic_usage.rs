//! Basic usage example for the S-expression parser
//! 
//! This example demonstrates the main features of the parser including
//! error handling, zero-copy parsing, and converting to owned expressions.

use sexpression::{Expression, OwnedExpression, ParseError, read};

fn main() -> Result<(), ParseError> {
    println!("S-Expression Parser Examples\n");
    
    // Example 1: Basic parsing
    println!("1. Basic parsing:");
    let source = "(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))";
    let expr = read(source)?;
    println!("   Input: {}", source);
    println!("   Parsed: {:?}\n", expr);
    
    // Example 2: Different data types
    println!("2. Different data types:");
    let types = vec![
        "42",                    // Number
        "true",                  // Boolean
        "false",                 // Boolean
        "null",                  // Null
        "\"hello world\"",       // String
        "symbol",                // Symbol
        "(1 2 3)",              // List
    ];
    
    for input in types {
        match read(input) {
            Ok(expr) => println!("   {} -> {:?}", input, expr),
            Err(e) => println!("   {} -> Error: {}", input, e),
        }
    }
    println!();
    
    // Example 3: Error handling
    println!("3. Error handling:");
    let errors = vec![
        "(unclosed",
        ")unexpected",
        "",
    ];
    
    for input in errors {
        match read(input) {
            Ok(expr) => println!("   {} -> {:?}", input, expr),
            Err(e) => println!("   {} -> Error: {}", input, e),
        }
    }
    println!();
    
    // Example 4: Converting to owned
    println!("4. Converting to owned:");
    let borrowed = Expression::Symbol("hello");
    let owned: OwnedExpression = borrowed.to_owned();
    println!("   Borrowed: {:?}", borrowed);
    println!("   Owned: {:?}\n", owned);
    
    // Example 5: Complex nested expression
    println!("5. Complex nested expression:");
    let complex = "(define (map f lst) (if (null? lst) '() (cons (f (car lst)) (map f (cdr lst)))))";
    match read(complex) {
        Ok(expr) => {
            println!("   Input: {}", complex);
            println!("   Parsed successfully!");
            println!("   Expression type: {:?}", std::mem::discriminant(&expr));
        }
        Err(e) => println!("   Error: {}", e),
    }
    
    Ok(())
} 