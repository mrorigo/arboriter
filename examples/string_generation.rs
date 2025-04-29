//! Example of generating strings using for_tree
//!
//! This example demonstrates how the for_tree macro can be used for
//! combinatorial generation, not just traversing existing structures.

use arboriter::{for_tree, prune};

fn main() {
    println!("String Generation Examples");
    println!("========================");
    
    println!("\n1. Generate Strings of 'a' and 'b'");
    println!("-------------------------------");
    generate_strings("ab");
    
    println!("\n2. Generate Balanced Parentheses");
    println!("------------------------------");
    generate_balanced_parens(3);  // Generate all balanced parens with at most 3 pairs
    
    println!("\n3. Generate Binary Numbers");
    println!("------------------------");
    generate_binary_numbers(4);  // Generate binary numbers up to 4 bits
}

/// Generate all strings using characters from the given alphabet up to length 3
fn generate_strings(alphabet: &str) {
    println!("Generating strings using alphabet '{}' with length <= 3:", alphabet);
    
    let mut strings = Vec::new();
    
    for_tree!(s in String::new(); |s| s.len() <= 3; |s| {
        let mut branches: Vec<String> = Vec::new();
        for c in alphabet.chars() {
            branches.push(format!("{}{}", s, c));
        }
        branches
    } => {
        println!("Generated: \"{}\"", s);
        strings.push(s.clone());
        
        if s.len() == 3 {
            prune!(); // Don't generate longer strings
        }
    });
    
    println!("Total strings generated: {}", strings.len());
}

/// Generate all valid balanced parentheses strings with at most n pairs
fn generate_balanced_parens(n: usize) {
    println!("Generating balanced parentheses with at most {} pairs:", n);
    
    // State consists of (current_string, open_count, close_count)
    // where open_count and close_count track how many of each type we've used
    for_tree!(state in ("".to_string(), 0, 0); |state| state.1 <= n; |state| {
        let (s, open, close) = state.clone(); // Clone to get owned values
        let mut branches = Vec::new();
        
        // We can add an open paren if we haven't used all n
        if open < n {
            branches.push((format!("{}(", s), open + 1, close));
        }
        
        // We can add a close paren if there are unclosed open parens
        if close < open {
            branches.push((format!("{})", s), open, close + 1));
        }
        
        branches
    } => {
        let (s, open, close) = state.clone(); // Clone to get owned values
        
        // Only print fully balanced strings
        if open == close && open > 0 {
            println!("Generated: {}", s);
        }
    });
}

/// Generate all binary numbers up to a certain length
fn generate_binary_numbers(max_bits: usize) {
    println!("Generating binary numbers up to {} bits:", max_bits);
    
    for_tree!(num in "".to_string(); |num| num.len() <= max_bits; |num| {
        let mut branches = Vec::new();
        branches.push(format!("{}0", num));
        branches.push(format!("{}1", num));
        branches
    } => {
        // Skip the empty string
        if !num.is_empty() {
            // Convert to integer for display
            if let Ok(value) = u32::from_str_radix(&num, 2) {
                println!("Binary: {}, Decimal: {}", num, value);
            }
        }
        
        if num.len() == max_bits {
            prune!(); // Don't generate longer numbers
        }
    });
}