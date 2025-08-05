//! Example demonstrating custom symbol types in S-expressions
//! 
//! This example shows how to implement custom symbol types for owned expressions,
//! allowing for rich symbol representations while maintaining zero-copy performance
//! for borrowed expressions.

use sexpression::{OwnedSymbol, OwnedExpression, StringOwnedSymbol, Expression, read};
use std::fmt;

/// A custom symbol type that supports namespaces
#[derive(Debug, Clone, PartialEq)]
struct NamespacedSymbol {
    namespace: Option<String>,
    name: String,
}

impl OwnedSymbol for NamespacedSymbol {
    fn from_str(s: &str) -> Self {
        if let Some((ns, name)) = s.split_once("::") {
            NamespacedSymbol {
                namespace: Some(ns.to_string()),
                name: name.to_string(),
            }
        } else {
            NamespacedSymbol {
                namespace: None,
                name: s.to_string(),
            }
        }
    }
    
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.namespace {
            Some(ns) => write!(f, "{}::{}", ns, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

/// A custom symbol type that tracks symbol categories
#[derive(Debug, Clone, PartialEq)]
struct CategorizedSymbol {
    category: SymbolCategory,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolCategory {
    Function,
    Variable,
    Type,
    Macro,
}

impl OwnedSymbol for CategorizedSymbol {
    fn from_str(s: &str) -> Self {
        // Parse category from prefix
        if let Some((cat, name)) = s.split_once(':') {
            let category = match cat {
                "fn" => SymbolCategory::Function,
                "var" => SymbolCategory::Variable,
                "type" => SymbolCategory::Type,
                "macro" => SymbolCategory::Macro,
                _ => SymbolCategory::Variable, // Default
            };
            CategorizedSymbol {
                category,
                name: name.to_string(),
            }
        } else {
            CategorizedSymbol {
                category: SymbolCategory::Variable,
                name: s.to_string(),
            }
        }
    }
    
    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.category {
            SymbolCategory::Function => "fn:",
            SymbolCategory::Variable => "var:",
            SymbolCategory::Type => "type:",
            SymbolCategory::Macro => "macro:",
        };
        write!(f, "{}{}", prefix, self.name)
    }
}

fn main() {
    println!("=== Custom Symbol Types Example ===\n");
    
    // Example 1: Using default string symbols
    println!("1. Default String Symbols:");
    let default_expr = OwnedExpression::<StringOwnedSymbol>::Symbol(
        StringOwnedSymbol::from_str("hello")
    );
    println!("   {}", default_expr);
    
    // Example 2: Using namespaced symbols
    println!("\n2. Namespaced Symbols:");
    let ns_expr = OwnedExpression::<NamespacedSymbol>::Symbol(
        NamespacedSymbol::from_str("std::vector")
    );
    println!("   {}", ns_expr);
    
    let simple_expr = OwnedExpression::<NamespacedSymbol>::Symbol(
        NamespacedSymbol::from_str("main")
    );
    println!("   {}", simple_expr);
    
    // Example 3: Using categorized symbols
    println!("\n3. Categorized Symbols:");
    let func_expr = OwnedExpression::<CategorizedSymbol>::Symbol(
        CategorizedSymbol::from_str("fn:factorial")
    );
    println!("   {}", func_expr);
    
    let var_expr = OwnedExpression::<CategorizedSymbol>::Symbol(
        CategorizedSymbol::from_str("var:count")
    );
    println!("   {}", var_expr);
    
    let type_expr = OwnedExpression::<CategorizedSymbol>::Symbol(
        CategorizedSymbol::from_str("type:Result")
    );
    println!("   {}", type_expr);
    
    // Example 4: Converting from borrowed expressions
    println!("\n4. Converting from Borrowed Expressions:");
    let borrowed = Expression::Symbol("std::vector");
    let owned_ns: OwnedExpression<NamespacedSymbol> = borrowed.to_owned();
    println!("   Borrowed: {:?}", borrowed);
    println!("   Owned: {}", owned_ns);
    
    // Example 5: Complex expressions with custom symbols
    println!("\n5. Complex Expressions with Custom Symbols:");
    let complex_expr = OwnedExpression::<NamespacedSymbol>::List(vec![
        OwnedExpression::Symbol(NamespacedSymbol::from_str("std::vector")),
        OwnedExpression::Number(1.0),
        OwnedExpression::Number(2.0),
        OwnedExpression::Number(3.0),
    ]);
    println!("   {}", complex_expr);
    
    // Example 6: Parsing and converting
    println!("\n6. Parsing and Converting:");
    match read("(std::vector 1 2 3)") {
        Ok(borrowed) => {
            let owned: OwnedExpression<NamespacedSymbol> = borrowed.to_owned();
            println!("   Parsed: {}", owned);
        }
        Err(e) => println!("   Parse error: {}", e),
    }
    
    println!("\n=== Example Complete ===");
} 