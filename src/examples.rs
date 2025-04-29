//! Additional examples of using the for_tree macro

use crate::{BinaryNode, break_tree, for_tree, prune};

/// Example: Finding a value in a binary tree
pub fn find_value_example() {
    let root = create_sample_tree();
    let target = 5;

    println!("Searching for value {} in tree:", target);

    let mut found = false;

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

/// Example: Pruning branches that don't meet a condition
pub fn pruning_example() {
    let root = create_sample_tree();

    println!("Traversing tree, pruning negative values:");

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
        println!("Visiting node with value: {}", node.value);

        if node.value < 0 {
            println!("  Pruning at negative value: {}", node.value);
            prune!();
        }
    });
}

/// Example: Using for_tree like a for loop to generate a sequence
pub fn fibonacci_example() {
    println!("Generating Fibonacci sequence up to 1000:");

    for_tree!(fib in (1, 1); |fib| fib.0 < 1000; |fib| vec![(fib.1, fib.0 + fib.1)] => {
        println!("Fibonacci number: {}", fib.0);
    });
}

/// Example: Emulating a breadth-first search by level tracking
pub fn bfs_emulation_example() {
    let root = create_sample_tree();

    println!("BFS-like traversal by tracking levels:");

    // We use a tuple of (node, level) to track depth
    for_tree!(item in (&root, 0); |_| true; |item| {
        let (node, level) = *item;
        let mut children = Vec::new();

        // We sort children by level to process same-level nodes together
        if let Some(left) = &node.left {
            children.push((left.as_ref(), level + 1));
        }
        if let Some(right) = &node.right {
            children.push((right.as_ref(), level + 1));
        }

        // Sort by level to approximate BFS
        children.sort_by_key(|&(_, l)| l);
        children
    } => {
        let (node, level) = *item;
        println!("Level {}: value {}", level, node.value);
    });
}

/// Example: Using for_tree with a custom recursive data structure
pub struct FileSystemNode {
    pub name: String,
    pub is_directory: bool,
    pub children: Vec<FileSystemNode>,
}

pub fn filesystem_example() {
    // Create a simple file system structure
    let fs_root = FileSystemNode {
        name: "root".to_string(),
        is_directory: true,
        children: vec![
            FileSystemNode {
                name: "documents".to_string(),
                is_directory: true,
                children: vec![
                    FileSystemNode {
                        name: "report.docx".to_string(),
                        is_directory: false,
                        children: vec![],
                    },
                    FileSystemNode {
                        name: "data.xlsx".to_string(),
                        is_directory: false,
                        children: vec![],
                    },
                ],
            },
            FileSystemNode {
                name: "pictures".to_string(),
                is_directory: true,
                children: vec![FileSystemNode {
                    name: "vacation.jpg".to_string(),
                    is_directory: false,
                    children: vec![],
                }],
            },
            FileSystemNode {
                name: "config.cfg".to_string(),
                is_directory: false,
                children: vec![],
            },
        ],
    };

    println!("File system traversal:");

    for_tree!(node in &fs_root; |_| true; |node| {
        // Only directories have children to traverse
        if node.is_directory {
            node.children.iter().collect()
        } else {
            Vec::new()
        }
    } => {
        let indent = "  ".repeat(get_depth(node, &fs_root));
        let node_type = if node.is_directory { "DIR" } else { "FILE" };
        println!("{}{}: {}", indent, node_type, node.name);
    });
}

// Helper function to calculate depth in the filesystem tree
fn get_depth(node: &FileSystemNode, root: &FileSystemNode) -> usize {
    if node.name == root.name {
        0
    } else {
        let mut current = Some(root);
        let mut depth = 0;

        while let Some(n) = current {
            for child in &n.children {
                if child.name == node.name {
                    return depth + 1;
                }
            }

            // This is simplified and not correct for a real implementation
            depth += 1;
            current = n.children.first();
        }

        depth
    }
}

// Helper function to create a sample binary tree
fn create_sample_tree() -> BinaryNode<i32> {
    BinaryNode::with_children(
        1,
        Some(Box::new(BinaryNode::with_children(
            2,
            Some(Box::new(BinaryNode::with_children(
                4,
                Some(Box::new(BinaryNode::new(-8))),
                Some(Box::new(BinaryNode::new(9))),
            ))),
            Some(Box::new(BinaryNode::new(5))),
        ))),
        Some(Box::new(BinaryNode::with_children(
            3,
            Some(Box::new(BinaryNode::new(-7))),
            Some(Box::new(BinaryNode::new(6))),
        ))),
    )
}
