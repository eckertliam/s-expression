//! S-Expression Parser Module
//! 
//! This module provides a zero-copy S-expression parser optimized for compiler use cases.
//! The parser uses borrowed string slices to minimize memory allocations while providing
//! fast parsing performance through various optimizations.
//! 
//! # Features
//! 
//! - **Zero-copy parsing**: Uses borrowed string slices to avoid unnecessary allocations
//! - **Fast-path optimizations**: Optimized number parsing and single-character symbols
//! - **Production error handling**: Proper error types instead of panics
//! - **Memory efficient**: Pre-allocated vectors and optimized tokenization
//! 
//! # Example
//! 
//! ```rust
//! use sexpression::{Expression, read};
//! 
//! let result = read("(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))");
//! match result {
//!     Ok(expr) => println!("Parsed: {:?}", expr),
//!     Err(e) => eprintln!("Parse error: {}", e),
//! }
//! ```

// Zero-copy Expression that borrows from source
/// Represents an S-expression as a borrowed data structure.
/// 
/// This enum provides zero-copy access to parsed S-expressions by borrowing
/// string slices from the original source. This is ideal for compiler use cases
/// where you need to parse expressions without taking ownership of the data.
/// 
/// # Examples
/// 
/// ```rust
/// use sexpression::Expression;
/// 
/// // Numbers
/// let num = Expression::Number(42.0);
/// 
/// // Symbols
/// let sym = Expression::Symbol("define");
/// 
/// // Lists
/// let list = Expression::List(vec![
///     Expression::Symbol("+"),
///     Expression::Number(1.0),
///     Expression::Number(2.0)
/// ]);
/// ```
#[derive(PartialEq, Debug)]
pub enum Expression<'a> {
    /// A numeric literal (f64)
    Number(f64),
    /// A boolean literal
    Bool(bool),
    /// A string literal (borrowed from source)
    Str(&'a str),
    /// A symbol/identifier (borrowed from source)
    Symbol(&'a str),
    /// A list of expressions
    List(Vec<Expression<'a>>),
    /// A null value
    Null,
}

/// Owned version of Expression for when you need ownership.
/// 
/// This is useful when you need to store expressions independently of the
/// original source string, or when you need to modify the expressions.
/// 
/// # Examples
/// 
/// ```rust
/// use sexpression::{Expression, OwnedExpression};
/// 
/// let borrowed = Expression::Symbol("hello");
/// let owned: OwnedExpression = borrowed.to_owned();
/// assert_eq!(owned, OwnedExpression::Symbol("hello".to_string()));
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum OwnedExpression {
    /// A numeric literal (f64)
    Number(f64),
    /// A boolean literal
    Bool(bool),
    /// A string literal (owned)
    Str(String),
    /// A symbol/identifier (owned)
    Symbol(String),
    /// A list of expressions
    List(Vec<OwnedExpression>),
    /// A null value
    Null,
}

/// Parse errors that can occur during S-expression parsing.
/// 
/// This enum provides detailed error information for debugging and
/// error handling in production environments.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Unexpected end of input while parsing
    #[error("Unexpected EOF")]
    UnexpectedEOF,
    /// Missing closing parenthesis in a list
    #[error("Missing closing parenthesis")]
    MissingClosingParen,
    /// Unexpected closing parenthesis (no matching opening parenthesis)
    #[error("Unexpected closing parenthesis")]
    UnexpectedClosingParen,
}

impl<'a> Expression<'a> {
    /// Convert a borrowed expression to an owned expression.
    /// 
    /// This method allocates new strings for all string and symbol data,
    /// allowing the resulting expression to outlive the original source.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sexpression::Expression;
    /// 
    /// let borrowed = Expression::Symbol("hello");
    /// let owned = borrowed.to_owned();
    /// assert_eq!(owned, sexpression::OwnedExpression::Symbol("hello".to_string()));
    /// ```
    pub fn to_owned(&self) -> OwnedExpression {
        match self {
            Expression::Number(n) => OwnedExpression::Number(*n),
            Expression::Bool(b) => OwnedExpression::Bool(*b),
            Expression::Str(s) => OwnedExpression::Str(s.to_string()),
            Expression::Symbol(s) => OwnedExpression::Symbol(s.to_string()),
            Expression::List(list) => OwnedExpression::List(
                list.iter().map(|expr| expr.to_owned()).collect()
            ),
            Expression::Null => OwnedExpression::Null,
        }
    }
}

/// Optimized zero-copy tokenizer using string slices.
/// 
/// This function efficiently tokenizes S-expression source code by:
/// - Pre-allocating vectors with realistic capacity estimates
/// - Using efficient string operations instead of character-by-character iteration
/// - Minimizing memory allocations through zero-copy string slices
/// 
/// # Arguments
/// 
/// * `src` - The source string to tokenize
/// 
/// # Returns
/// 
/// A vector of string slices representing the tokens
fn tokenize(src: &str) -> Vec<&str> {
    // More realistic capacity estimate
    let mut tokens = Vec::with_capacity(src.len() / 2);
    let mut current = src;
    
    while !current.is_empty() {
        // Skip leading whitespace efficiently
        current = current.trim_start();
        if current.is_empty() { break; }
        
        // Find next delimiter or whitespace
        let (token, rest) = match current.find(|c: char| c.is_whitespace() || "()'".contains(c)) {
            Some(pos) => {
                let token = &current[..pos];
                let rest = &current[pos..];
                (token, rest)
            }
            None => (current, ""),
        };
        
        if !token.is_empty() {
            tokens.push(token);
        }
        
        // Handle delimiter efficiently
        if !rest.is_empty() {
            let delimiter = &rest[..1];
            if "()'".contains(delimiter) {
                tokens.push(delimiter);
            }
            current = &rest[1..];
        } else {
            break;
        }
    }
    
    tokens
}

/// Optimized zero-copy parser with proper error handling.
/// 
/// This function parses a slice of tokens into an S-expression, using:
/// - Pre-allocated vectors for common list sizes
/// - Proper error handling instead of panics
/// - Recursive descent parsing with zero-copy semantics
/// 
/// # Arguments
/// 
/// * `tokens` - A mutable reference to a slice of tokens to parse
/// 
/// # Returns
/// 
/// A `Result` containing either the parsed expression or a parse error
/// 
/// # Errors
/// 
/// Returns `ParseError` variants for various parsing failures
fn parse<'a>(tokens: &mut &[&'a str]) -> Result<Expression<'a>, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::UnexpectedEOF);
    }
    
    let token = tokens[0];
    *tokens = &tokens[1..]; // Advance slice
    
    match token {
        "(" => {
            // Pre-allocate list vector for common list sizes
            let mut stack = Vec::with_capacity(8);
            while !tokens.is_empty() && tokens[0] != ")" {
                stack.push(parse(tokens)?);
            }
            if tokens.is_empty() {
                return Err(ParseError::MissingClosingParen);
            }
            *tokens = &tokens[1..]; // Skip closing paren
            Ok(Expression::List(stack))
        }
        ")" => Err(ParseError::UnexpectedClosingParen),
        _ => Ok(parse_atom(token)),
    }
}

/// Optimized atom parsing with fast paths.
/// 
/// This function parses individual tokens into atomic expressions using:
/// - Fast-path checks for single-character symbols
/// - Optimized number parsing with first-character checks
/// - Bounds-safe string literal handling
/// 
/// # Arguments
/// 
/// * `token` - The token string to parse as an atom
/// 
/// # Returns
/// 
/// The parsed atomic expression
fn parse_atom(token: &str) -> Expression {
    // Fast path: single character symbols
    if token.len() == 1 {
        return Expression::Symbol(token);
    }
    
    // Fast path: check first character for number parsing
    if let Some(first) = token.chars().next() {
        if first.is_ascii_digit() || first == '-' || first == '+' {
            if let Ok(n) = token.parse::<f64>() {
                return Expression::Number(n);
            }
        }
    }
    
    // Check for booleans and null
    match token {
        "true" => return Expression::Bool(true),
        "false" => return Expression::Bool(false),
        "null" => return Expression::Null,
        _ => {}
    }
    
    // Optimized string literal handling with bounds safety
    if token.len() >= 2 && token.starts_with('"') && token.ends_with('"') {
        if let Some(content) = token.get(1..token.len()-1) {
            return Expression::Str(content);
        }
    }
    
    // Default to symbol
    Expression::Symbol(token)
}

/// Main parsing function with error handling.
/// 
/// This is the primary entry point for parsing S-expressions. It provides:
/// - Zero-copy parsing with borrowed string slices
/// - Comprehensive error handling
/// - Optimized performance through various fast paths
/// 
/// # Arguments
/// 
/// * `src` - The source string to parse as an S-expression
/// 
/// # Returns
/// 
/// A `Result` containing either the parsed expression or a parse error
/// 
/// # Examples
/// 
/// ```rust
/// use sexpression::{read, Expression};
/// 
/// // Successful parsing
/// let result = read("(define x 42)");
/// assert!(result.is_ok());
/// 
/// // Error handling
/// let result = read("(unclosed");
/// assert!(result.is_err());
/// ```
pub fn read(src: &str) -> Result<Expression, ParseError> {
    let tokens = tokenize(src);
    let mut token_slice = tokens.as_slice();
    parse(&mut token_slice)
}

/// Convenience function for backward compatibility (panics on error).
/// 
/// This function provides the same interface as the original parser
/// but will panic if parsing fails. Use `read()` for production code
/// that needs proper error handling.
/// 
/// # Arguments
/// 
/// * `src` - The source string to parse as an S-expression
/// 
/// # Returns
/// 
/// The parsed expression
/// 
/// # Panics
/// 
/// Panics if the source cannot be parsed as a valid S-expression
/// 
/// # Examples
/// 
/// ```rust
/// use sexpression::read_unchecked;
/// 
/// let expr = read_unchecked("(hello world)");
/// // Use expr safely knowing it was parsed successfully
/// ```
pub fn read_unchecked(src: &str) -> Expression {
    read(src).expect("Failed to parse S-expression")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_test() {
        assert_eq!(tokenize("this is a test"), vec!["this", "is", "a", "test"]);
        assert_eq!(tokenize("(hello world)"), vec!["(", "hello", "world", ")"]);
    }

    #[test]
    fn read_test() {
        let result = read("(1 symbol \"string\" true null (1.5))").unwrap();
        println!("{:?}", result);
        
        // Test error handling
        assert!(read("(unclosed").is_err());
        assert!(read(")unexpected").is_err());
    }

    #[test]
    fn fast_path_tests() {
        // Test single character symbols
        let result = read("a").unwrap();
        assert!(matches!(result, Expression::Symbol("a")));
        
        // Test number parsing
        let result = read("42").unwrap();
        assert!(matches!(result, Expression::Number(42.0)));
        
        // Test negative numbers
        let result = read("-3.14").unwrap();
        assert!(matches!(result, Expression::Number(-3.14)));
    }
    
    #[test]
    fn performance_test() {
        // Simple performance test without unstable features
        let input = "(define (factorial n) (if (= n 0) 1 (* n (factorial (- n 1)))))";
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = read(input);
        }
        let duration = start.elapsed();
        println!("Parsed 1000 times in {:?}", duration);
    }
}