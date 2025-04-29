![arboriter logo](./arboriter.png)

# arboriter: Rust Tree Traversal Primitive

This crate provides a `for_tree!` macro that simplifies tree traversal in Rust, similar to how `for` loops simplify linear traversal. It is inspired by [Tyler Glaiel's blog post](https://blog.tylerglaiel.com/p/programming-languages-should-have) discussing the need for a tree traversal primitive in programming languages.

## Motivation

Tree traversal is a common operation, but it often requires writing boilerplate recursive functions. The `for_tree!` macro provides a clean, expressive syntax for traversing tree-like structures without explicit recursion.

## Features

- Clean syntax similar to a `for` loop
- Support for control flow: `break_tree!()`, `prune!()`
- Works with both actual tree data structures and imperative tree generation
- Type-safe and zero-cost abstraction
- Minimal boilerplate

## Usage

### Basic Syntax

```rust
use arboriter::{for_tree, break_tree, prune};

for_tree!(variable = initial_value; condition; branches => {
    // body
    // You can use special control flow:
    // - break_tree!(); - exits the entire traversal
    // - prune!(); - skips traversing children of the current node
});
```

### Example: Traversing a Binary Tree

```rust
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Create a tree
let root = // ...

// Traverse the tree
for_tree!(node = &root; node.is_some(); {
    let mut branches = Vec::new();
    if let Some(left) = &node.left {
        branches.push(left.as_ref());
    }
    if let Some(right) = &node.right {
        branches.push(right.as_ref());
    }
    branches
} => {
    println!("Node value: {}", node.value);

    if node.value == 10 {
        break_tree!(); // Exit the traversal
    }

    if node.value < 0 {
        prune!(); // Don't traverse children of negative nodes
    }
});
```

### Example: Generating Strings

```rust
for_tree!(s = String::new(); s.len() <= 8; {
    vec![
        format!("{}a", s),
        format!("{}b", s),
        format!("{}c", s)
    ]
} => {
    println!("{}", s);

    if s.len() == 8 {
        prune!(); // Don't generate longer strings
    }
});
```

## Syntax Variants

You can use several syntax variants based on your preference:

```rust
// Using => to separate branches from body (recommended)
for_tree!(node = root; condition; branches => { /* body */ });

// Using semicolons instead of =>
for_tree!(node = root; condition; branches; { /* body */ });

// With explicit closures for more control
for_tree!(node in root; |node| condition; |node| branches => { /* body */ });
```

## How It Works

The `for_tree!` macro expands to code that handles the recursive traversal for you. It leverages Rust's ownership system to safely and efficiently traverse trees without any runtime overhead compared to hand-written recursive code.

The traversal follows a depth-first search pattern:
1. The current node is visited
2. Each branch is then fully explored (to its maximum depth) before moving to the next branch
3. Branches are processed in the order they are returned from the branch function

## Performance

The `for_tree!` macro is a zero-cost abstraction. It compiles down to the same efficient code you would write by hand, with no additional runtime overhead.

## License

This crate is licensed under the MIT license.


---
This README was created by Claude 3.7 Sonnet, with <3
