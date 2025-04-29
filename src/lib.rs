//! # arboriter: Tree Traversal Primitives for Rust
//!
//! The `arboriter` crate provides a tree traversal primitive similar to a for loop
//! but designed specifically for traversing tree-like structures without explicit recursion.
//!
//! Inspired by Tyler Glaiel's blog post discussing the need for such a primitive, this
//! crate implements a `for_tree!` macro that brings the convenience of standard control
//! flow constructs to tree traversal operations.
//!
//! ## Features
//!
//! - **Intuitive Syntax**: Similar to a for loop, familiar to most programmers
//! - **Control Flow**: Support for `break`, `continue`, and a special `prune!` operation
//! - **Flexible**: Works with both in-memory tree structures and imperative tree generation
//! - **Type Safe**: Fully leverages Rust's type system for safety and clarity
//! - **Zero Cost**: Compiles to efficient code equivalent to hand-written recursion
//!
//! ## Quick Example
//!
//! ```rust
//! use arboriter::{for_tree, prune, break_tree, BinaryNode};
//!
//! // Create a simple binary tree
//! let root = BinaryNode::with_children(
//!     1,
//!     Some(Box::new(BinaryNode::new(2))),
//!     Some(Box::new(BinaryNode::new(3)))
//! );
//!
//! // Traverse the tree
//! for_tree!(node in &root; |_| true; |node| {
//!     let mut children: Vec<&BinaryNode<i32>> = Vec::new();
//!     if let Some(left) = &node.left {
//!         children.push(left.as_ref());
//!     }
//!     if let Some(right) = &node.right {
//!         children.push(right.as_ref());
//!     }
//!     children
//! } => {
//!     println!("Value: {}", node.value);
//! });
//! ```
//!
//! ## Usage
//!
//! The `for_tree!` macro syntax is designed to be reminiscent of a standard for loop:
//!
//! ```rust,ignore
//! for_tree!(variable in initial_value; condition; branches => {
//!     // body
//!     // You can use special control flow:
//!     // - break_tree!(); - exits the entire traversal
//!     // - prune!(); - skips traversing children of the current node
//! });
//! ```
//!
//! Where:
//! - `variable` is the name of the variable for the current node
//! - `initial_value` is the starting node
//! - `condition` is a closure that determines if a node should be visited
//! - `branches` is a closure that returns a vector of child nodes
//! - `body` is the code executed for each node
//!
//! ## Advanced Example: String Generation
//!
//! The `for_tree!` macro isn't limited to traversing existing data structures;
//! it can also be used to generate tree-like data:
//!
//! ```rust
//! use arboriter::{for_tree, prune};
//!
//! // Generate all strings of 'a' and 'b' with length <= 2
//! let mut strings = Vec::new();
//!
//! for_tree!(s in String::new(); |s| s.len() <= 2; |s| {
//!     let mut branches: Vec<String> = Vec::new();
//!     branches.push(format!("{}a", s));
//!     branches.push(format!("{}b", s));
//!     branches
//! } => {
//!     strings.push(s.clone());
//!
//!     if s.len() == 2 {
//!         prune!(); // Don't generate longer strings
//!     }
//! });
//!
//! // Strings in depth-first order: ["", "a", "aa", "ab", "b", "ba", "bb"]
//! assert_eq!(strings, vec!["", "a", "aa", "ab", "b", "ba", "bb"]);
//! ```
//!
//! ## Control Flow
//!
//! The `for_tree!` macro supports special control flow operations:
//!
//! - `continue` - Standard Rust continue, skip to the next iteration
//! - `prune!()` - Skip traversing children of the current node
//! - `break_tree!()` - Exit the entire traversal (unwinding the recursion stack)
//!
//! ## Performance
//!
//! The `for_tree!` macro is a zero-cost abstraction - it compiles down to efficient
//! recursive code and introduces no runtime overhead compared to hand-written
//! recursive traversal functions.
//!

/// Enum representing control flow options within a tree traversal.
///
/// This enum allows controlling how traversal proceeds after visiting a node:
/// 
/// # Variants
///
/// * `Continue` - Continue normal traversal, visiting this node's children
/// * `Prune` - Skip traversing children of the current node, but continue with sibling nodes
/// * `Break` - Stop the entire traversal immediately
///
/// # Usage
///
/// When using the [`for_tree!`] macro, you can use the following control flow operations:
/// * `continue` - (implicit) Normal Rust continue behavior (built-in keyword)
/// * [`prune!`] - Skip children of the current node
/// * [`break_tree!`] - Exit the entire traversal
///
/// When using [`traverse_tree`] directly, return the appropriate variant from your visitor function.
///
/// # Example
///
/// ```
/// use arboriter::{traverse_tree, TreeControl};
///
/// // Generate a sequence of numbers with pruning
/// // We'll manually build up a tree traversal that skips even numbers' children
/// 
/// // This example is a bit contrived just to demonstrate the enum values
/// // In practice, you'd probably use the for_tree! macro instead
/// let mut sequence = Vec::new();
/// 
/// // Start with even numbers 0, 2, 4, 6, 8, 10
/// // Each has a single odd child: +1
/// let root_nodes = vec![0, 2, 4, 6, 8, 10];
/// 
/// for &start in &root_nodes {
///     traverse_tree(
///         start,
///         |_| true, // Visit all nodes
///         |&n| {
///             // Each node branches to n+1, but we'll prune this in the visitor
///             vec![n + 1]
///         },
///         |&n| {
///             sequence.push(n);
///             
///             if n % 2 == 0 {
///                 // Skip children of even numbers
///                 TreeControl::Prune
///             } else {
///                 // Process children of odd numbers
///                 TreeControl::Continue
///             }
///         }
///     );
/// }
/// 
/// // We should see all root nodes (the evens) but none of their children (odds)
/// // because we pruned at even numbers
/// assert_eq!(sequence, vec![0, 2, 4, 6, 8, 10]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeControl {
    /// Continue traversal normally, including this node's children
    Continue,
    /// Skip traversing children of the current node, but continue with siblings
    Prune,
    /// Break out of the entire traversal immediately
    Break,
}

/// Core function that handles depth-first tree traversal of arbitrary tree-like structures.
///
/// This function provides the internal implementation for the [`for_tree!`] macro. It takes
/// a value, traverses it and its branches according to the provided functions, and applies
/// a visitor function to each node in the traversal.
///
/// # Traversal Order
///
/// The traversal order follows classic depth-first search:
/// 1. Visit the current node
/// 2. For each branch (in the order returned by `branch_fn`):
///    - Recursively traverse that branch to its full depth
///    - Only then proceed to the next branch
///
/// # Parameters
///
/// * `initial` - The root value to start traversal from
/// * `condition` - A function that determines whether a node should be visited (returns `true` for visit)
/// * `branch_fn` - A function that returns a vector of branches from a given node
/// * `visit_fn` - A function that is called for each visited node, returning control flow instructions
///
/// # Type Parameters
///
/// * `T` - The type of values in the tree structure (must implement `Clone`)
/// * `C` - The type of the condition function
/// * `B` - The type of the branching function
/// * `F` - The type of the visitor function
///
/// # Control Flow
///
/// The visitor function should return a [`TreeControl`] value to control traversal:
/// * `TreeControl::Continue` - Continue normal traversal
/// * `TreeControl::Prune` - Skip traversing children of the current node
/// * `TreeControl::Break` - Stop the entire traversal
///
/// # Example
///
/// ```
/// use arboriter::{traverse_tree, TreeControl, BinaryNode};
///
/// // Create a simple binary tree
/// let root = BinaryNode::with_children(
///     1,
///     Some(Box::new(BinaryNode::new(2))),
///     Some(Box::new(BinaryNode::new(3)))
/// );
///
/// // Collect values using direct traversal function
/// let mut values = Vec::new();
///
/// traverse_tree(
///     &root,
///     |_| true, // Visit all nodes
///     |node| {
///         // Get child branches
///         let mut children: Vec<&BinaryNode<i32>> = Vec::new();
///         if let Some(left) = &node.left {
///             children.push(left.as_ref());
///         }
///         if let Some(right) = &node.right {
///             children.push(right.as_ref());
///         }
///         children
///     },
///     |node| {
///         // Visit each node
///         values.push(node.value);
///         TreeControl::Continue
///     }
/// );
///
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
pub fn traverse_tree<T, C, B, F>(
    initial: T,
    condition: C,
    branch_fn: B,
    mut visit_fn: F,
) where
    T: Clone,
    C: Fn(&T) -> bool,
    B: Fn(&T) -> Vec<T>,
    F: FnMut(&T) -> TreeControl,
{
    // Define the recursive traversal function
    fn traverse_internal<T, C, B, F>(
        node: &T,
        condition: &C,
        branch_fn: &B,
        visit_fn: &mut F,
    ) -> TreeControl
    where
        T: Clone,
        C: Fn(&T) -> bool,
        B: Fn(&T) -> Vec<T>,
        F: FnMut(&T) -> TreeControl,
    {
        // Visit the current node
        let result = visit_fn(node);

        // Handle control flow
        match result {
            TreeControl::Break => return TreeControl::Break,
            TreeControl::Prune => return TreeControl::Continue,
            TreeControl::Continue => {}
        }

        // Get branches and continue traversal if condition is met
        for child in branch_fn(node) {
            if condition(&child) {
                let child_result = traverse_internal(&child, condition, branch_fn, visit_fn);
                if child_result == TreeControl::Break {
                    return TreeControl::Break;
                }
            }
        }

        TreeControl::Continue
    }

    // Only traverse if the initial node meets the condition
    if condition(&initial) {
        traverse_internal(&initial, &condition, &branch_fn, &mut visit_fn);
    }
}

/// Skips traversing the children of the current node.
///
/// This macro is used within a [`for_tree!`] block to prevent traversal
/// of the current node's children. The traversal will continue with
/// sibling nodes.
///
/// # Example
///
/// ```
/// use arboriter::{for_tree, prune};
///
/// // Generate numbers in a simple tree structure, starting with 1
/// // Each number branches to [n*2, n*2+1]
/// // We'll prune at even numbers
/// let mut values = Vec::new();
///
/// for_tree!(n in 1; |n| *n < 8; |n| {
///     // Each node branches to [n*2, n*2+1]
///     vec![*n * 2, *n * 2 + 1]
/// } => {
///     values.push(*n);
///     
///     if *n % 2 == 0 {
///         prune!(); // Don't process children of even numbers
///     }
/// });
///
/// // With this traversal and pruning, we should see:
/// // 1 → 2 (prune) → 3 → 6 (prune) → 7
/// assert_eq!(values, vec![1, 2, 3, 6, 7]);
/// ```
#[macro_export]
macro_rules! prune {
    () => {
        return $crate::TreeControl::Prune;
    };
}

/// Breaks out of the entire tree traversal.
///
/// This macro is used within a [`for_tree!`] block to immediately stop
/// the entire traversal, unwinding the traversal stack and returning
/// control to the point after the [`for_tree!`] macro.
///
/// # Example
///
/// ```
/// use arboriter::{for_tree, break_tree};
///
/// // Find a specific value in a tree-like structure
/// let mut found = false;
/// let target = 7;
///
/// for_tree!(n in 0; |n| *n <= 10; |n| vec![*n + 1] => {
///     println!("Checking {}", n);
///     
///     if *n == target {
///         found = true;
///         break_tree!(); // Exit the traversal - we found what we're looking for
///     }
/// });
///
/// assert!(found);
/// ```
#[macro_export]
macro_rules! break_tree {
    () => {
        return $crate::TreeControl::Break;
    };
}

/// A macro for traversing tree-like structures or generating tree-like data.
///
/// # Syntax
///
/// ```rust,ignore
/// // This is just syntax illustration, not meant to be compiled
/// for_tree!(var in initial; condition; branches => {
///     // body
///     // You can use special control flow:
///     // - break_tree!(); - exits the entire traversal
///     // - prune!(); - skips traversing children of the current node
/// });
/// ```
///
/// # Examples
///
/// Traverse a binary tree:
/// ```rust,no_run
/// use arboriter::{for_tree, prune, break_tree, BinaryNode};
///
/// // Create a simple binary tree for demonstration
/// let root = BinaryNode::with_children(
///     10,
///     Some(Box::new(BinaryNode::new(5))),
///     Some(Box::new(BinaryNode::new(15)))
/// );
///
/// for_tree!(node in &root; |_| true; |node| {
///     // Explicitly declare the type of branches
///     let mut branches: Vec<&BinaryNode<i32>> = Vec::new();
///     if let Some(left) = &node.left {
///         branches.push(left.as_ref());
///     }
///     if let Some(right) = &node.right {
///         branches.push(right.as_ref());
///     }
///     branches
/// } => {
///     println!("Node value: {}", node.value);
///
///     if node.value == 10 {
///         break_tree!(); // Exit the traversal
///     }
///
///     if node.value < 0 {
///         prune!(); // Don't traverse children of negative nodes
///     }
/// });
/// ```
///
/// Generate strings of "a", "b", and "c" with length <= 8:
/// ```rust,no_run
/// use arboriter::{for_tree, prune};
///
/// for_tree!(s in String::new(); |s| s.len() <= 8; |s| {
///     // Create branches with explicit type
///     let mut branches: Vec<String> = Vec::new();
///     branches.push(format!("{}a", s));
///     branches.push(format!("{}b", s));
///     branches.push(format!("{}c", s));
///     branches
/// } => {
///     println!("{}", s);
///
///     if s.len() == 8 {
///         prune!(); // Don't generate longer strings
///     }
/// });
/// ```
#[macro_export]
macro_rules! for_tree {
    // Main pattern with => separator
    ($var:ident in $init:expr; $cond:expr; $branch:expr => $body:block) => {
        {
            $crate::traverse_tree(
                $init,
                $cond,
                $branch,
                |$var| {
                    let result = {
                        $body
                        $crate::TreeControl::Continue
                    };
                    result
                }
            );
        }
    };

    // Alternative syntax with semicolons instead of =>
    ($var:ident in $init:expr; $cond:expr; $branch:expr; $body:block) => {
        $crate::for_tree!($var in $init; $cond; $branch => $body);
    };

    // Allows shorter syntax when the closures are simple - uses = like in the blog post
    ($var:ident = $init:expr; $cond:expr; $branch:expr => $body:block) => {
        {
            let initial_value = $init;
            $crate::for_tree!(
                $var in initial_value; 
                |$var| $cond; 
                |$var| $branch;
                $body
            );
        }
    };

    // Very similar to for loop syntax with semicolons
    ($var:ident = $init:expr; $cond:expr; $branch:expr; $body:block) => {
        $crate::for_tree!($var = $init; $cond; $branch => $body);
    };
}

// Examples

/// Tree node example for binary trees
pub struct BinaryNode<T> {
    pub value: T,
    pub left: Option<Box<BinaryNode<T>>>,
    pub right: Option<Box<BinaryNode<T>>>,
}

impl<T> BinaryNode<T> {
    /// Creates a new `BinaryNode` with the given value and no children.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to store in this node
    ///
    /// # Returns
    ///
    /// A new `BinaryNode` with the specified value and `None` for both children.
    ///
    /// # Example
    ///
    /// ```
    /// use arboriter::BinaryNode;
    ///
    /// let node = BinaryNode::new(42);
    /// assert_eq!(node.value, 42);
    /// assert!(node.left.is_none());
    /// assert!(node.right.is_none());
    /// ```
    pub fn new(value: T) -> Self {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Creates a new `BinaryNode` with the given value and child nodes.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to store in this node
    /// * `left` - The left child of this node, if any
    /// * `right` - The right child of this node, if any
    ///
    /// # Returns
    ///
    /// A new `BinaryNode` with the specified value and children.
    ///
    /// # Example
    ///
    /// ```
    /// use arboriter::BinaryNode;
    ///
    /// let node = BinaryNode::with_children(
    ///     1,
    ///     Some(Box::new(BinaryNode::new(2))),
    ///     Some(Box::new(BinaryNode::new(3)))
    /// );
    ///
    /// assert_eq!(node.value, 1);
    /// assert_eq!(node.left.as_ref().unwrap().value, 2);
    /// assert_eq!(node.right.as_ref().unwrap().value, 3);
    /// ```
    pub fn with_children(
        value: T,
        left: Option<Box<BinaryNode<T>>>,
        right: Option<Box<BinaryNode<T>>>,
    ) -> Self {
        BinaryNode { value, left, right }
    }
}

/// Demonstrates traversing a binary tree with the for_tree macro.
///
/// This function shows a common pattern for traversing a binary tree using
/// the [`for_tree!`] macro. It prints the value of each node in the tree
/// in depth-first order.
///
/// # Parameters
///
/// * `root` - The root node of the binary tree to traverse
///
/// # Type Parameters
///
/// * `T` - The type of value stored in each node, must implement Debug
///
/// # Example
///
/// ```
/// use arboriter::{BinaryNode, binary_tree_example};
///
/// // Create a simple binary tree
/// let root = BinaryNode::with_children(
///     1,
///     Some(Box::new(BinaryNode::new(2))),
///     Some(Box::new(BinaryNode::new(3)))
/// );
///
/// // This will print:
/// // Traversing binary tree:
/// // Visiting node with value: 1
/// // Visiting node with value: 2
/// // Visiting node with value: 3
/// binary_tree_example(&root);
/// ```
pub fn binary_tree_example<T: std::fmt::Debug>(root: &BinaryNode<T>) {
    println!("Traversing binary tree:");

    for_tree!(node in root; |_| true; |node| {
        let mut children: Vec<&BinaryNode<T>> = Vec::new();
        if let Some(left) = &node.left {
            children.push(left.as_ref());
        }
        if let Some(right) = &node.right {
            children.push(right.as_ref());
        }
        children
    } => {
        println!("Visiting node with value: {:?}", node.value);
    });
}

/// Demonstrates using for_tree for string generation.
///
/// This function shows how [`for_tree!`] can be used for tasks other than
/// traversing existing data structures. It generates all possible strings
/// of "a", "b", and "c" with length <= 3, illustrating how tree traversal
/// can be used for combinatorial generation.
///
/// The example also demonstrates the use of [`prune!`] to limit the depth
/// of the traversal.
///
/// # Example
///
/// ```no_run
/// use arboriter::generate_strings_example;
///
/// // This will print all strings of a, b, c with length <= 3:
/// // ""
/// // "a"
/// // "aa"
/// // "aaa"
/// // "aab"
/// // "aac"
/// // "ab"
/// // ...etc.
/// generate_strings_example();
/// ```
pub fn generate_strings_example() {
    println!("Generating strings of a, b, c with length <= 3:");

    for_tree!(s in String::new(); |s| s.len() <= 3; |s| {
        let mut branches: Vec<String> = Vec::new();
        branches.push(format!("{}a", s));
        branches.push(format!("{}b", s));
        branches.push(format!("{}c", s));
        branches
    } => {
        println!("Generated string: {}", s);

        if s.len() == 3 {
            prune!(); // Don't generate longer strings
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree() {
        // Create a simple binary tree
        let root = BinaryNode::with_children(
            1,
            Some(Box::new(BinaryNode::with_children(
                2,
                Some(Box::new(BinaryNode::new(4))),
                Some(Box::new(BinaryNode::new(5))),
            ))),
            Some(Box::new(BinaryNode::with_children(
                3,
                None,
                Some(Box::new(BinaryNode::new(6))),
            ))),
        );

        // Collect values using for_tree
        let mut values = Vec::new();

        for_tree!(node in &root; |_| true; |node| {
            let mut children = Vec::new();
            if let Some(left) = &node.left {
                children.push(left.as_ref());
            }
            if let Some(right) = &node.right {
                children.push(right.as_ref());
            }
            children
        } => {
            values.push(node.value);
        });

        // Verify depth-first traversal order
        assert_eq!(values, vec![1, 2, 4, 5, 3, 6]);
    }

    #[test]
    fn test_string_generation() {
        // Generate all strings of length <= 2
        let mut strings = Vec::new();

        for_tree!(s in String::new(); |s| s.len() <= 2; |s| {
            let mut branches = Vec::new();
            branches.push(format!("{}a", s));
            branches.push(format!("{}b", s));
            branches
        } => {
            strings.push(s.clone());

            if s.len() == 2 {
                prune!();
            }
        });

        // Check that we got all possible strings
        // The order is determined by the depth-first traversal:
        // "" -> "a" -> "aa" -> "ab" -> "b" -> "ba" -> "bb"
        let expected = vec!["", "a", "aa", "ab", "b", "ba", "bb"];

        assert_eq!(strings, expected);
    }

    #[test]
    fn test_break() {
        // Test breaking out of traversal
        let mut count = 0;

        for_tree!(n in 0; |n| *n < 10; |n| vec![*n + 1] => {
            count += 1;

            if *n >= 5 {
                break_tree!();
            }
        });

        // Should only visit 0, 1, 2, 3, 4, 5
        assert_eq!(count, 6);
    }

    #[test]
    fn test_prune() {
        // Create a binary tree with pruning
        let root = BinaryNode::with_children(
            1,
            Some(Box::new(BinaryNode::with_children(
                2, // We'll prune this branch
                Some(Box::new(BinaryNode::new(4))),
                Some(Box::new(BinaryNode::new(5))),
            ))),
            Some(Box::new(BinaryNode::with_children(
                3,
                None,
                Some(Box::new(BinaryNode::new(6))),
            ))),
        );

        let mut values = Vec::new();

        for_tree!(node in &root; |_| true; |node| {
            let mut children = Vec::new();
            if let Some(left) = &node.left {
                children.push(left.as_ref());
            }
            if let Some(right) = &node.right {
                children.push(right.as_ref());
            }
            children
        } => {
            values.push(node.value);

            if node.value == 2 {
                prune!();  // Don't visit children of node with value 2
            }
        });

        // Should only visit 1, 2, 3, 6 (4 and 5 are pruned)
        assert_eq!(values, vec![1, 2, 3, 6]);
    }
}
