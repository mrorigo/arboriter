//! Example of traversing a binary tree
//!
//! This example demonstrates how to traverse a binary tree using the for_tree macro.
//! It shows basic traversal, finding a specific node, and pruning branches.

use arboriter::{for_tree, prune, break_tree, BinaryNode};

fn main() {
    println!("Binary Tree Example");
    println!("==================");

    // Create a sample binary tree
    let root = create_sample_tree();
    
    println!("\n1. Basic Traversal");
    println!("-----------------");
    basic_traversal(&root);
    
    println!("\n2. Finding a Value");
    println!("-----------------");
    find_value(&root, 5);
    find_value(&root, 42); // Value not in tree
    
    println!("\n3. Pruning Example");
    println!("-----------------");
    pruning_example(&root);
}

/// Create a sample binary tree for demonstration
fn create_sample_tree() -> BinaryNode<i32> {
    // Create a tree that looks like:
    //       1
    //     /   \
    //    2     3
    //   / \   / \
    //  4   5 6   7
    //     /
    //    8
    BinaryNode::with_children(
        1,
        Some(Box::new(BinaryNode::with_children(
            2,
            Some(Box::new(BinaryNode::new(4))),
            Some(Box::new(BinaryNode::with_children(
                5,
                Some(Box::new(BinaryNode::new(8))),
                None,
            ))),
        ))),
        Some(Box::new(BinaryNode::with_children(
            3,
            Some(Box::new(BinaryNode::new(6))),
            Some(Box::new(BinaryNode::new(7))),
        ))),
    )
}

/// Basic traversal of a binary tree, printing all values
fn basic_traversal(root: &BinaryNode<i32>) {
    println!("Traversing binary tree in depth-first order...");
    
    for_tree!(node in root; |_| true; |node| {
        let mut children: Vec<&BinaryNode<i32>> = Vec::new();
        if let Some(left) = &node.left {
            children.push(left.as_ref());
        }
        if let Some(right) = &node.right {
            children.push(right.as_ref());
        }
        children
    } => {
        println!("Node value: {}", node.value);
    });
}

/// Find a specific value in the tree
fn find_value(root: &BinaryNode<i32>, target: i32) {
    println!("Searching for value {} in tree...", target);
    
    let mut found = false;
    
    for_tree!(node in root; |_| true; |node| {
        let mut children: Vec<&BinaryNode<i32>> = Vec::new();
        if let Some(left) = &node.left {
            children.push(left.as_ref());
        }
        if let Some(right) = &node.right {
            children.push(right.as_ref());
        }
        children
    } => {
        println!("Checking node with value: {}", node.value);
        
        if node.value == target {
            println!("Found target value: {}", target);
            found = true;
            break_tree!();
        }
    });
    
    if !found {
        println!("Value {} not found in tree", target);
    }
}

/// Demonstrate pruning by skipping subtrees with certain conditions
fn pruning_example(root: &BinaryNode<i32>) {
    println!("Traversing tree, pruning at nodes with even values:");
    
    for_tree!(node in root; |_| true; |node| {
        let mut children: Vec<&BinaryNode<i32>> = Vec::new();
        if let Some(left) = &node.left {
            children.push(left.as_ref());
        }
        if let Some(right) = &node.right {
            children.push(right.as_ref());
        }
        children
    } => {
        println!("Visiting node with value: {}", node.value);
        
        if node.value % 2 == 0 {
            println!("  Pruning at even value: {}", node.value);
            prune!();
        }
    });
}