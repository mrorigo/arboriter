//! Example of traversing a file system-like structure
//!
//! This example demonstrates how to use for_tree to traverse a hierarchical
//! file system structure, supporting both files and directories.

use arboriter::for_tree;

/// Represents a file system node (file or directory)
#[derive(Clone)]
struct FsNode {
    name: String,
    is_dir: bool,
    children: Vec<FsNode>,
    size: usize,
}

impl FsNode {
    /// Create a new file
    fn new_file(name: &str, size: usize) -> Self {
        FsNode {
            name: name.to_string(),
            is_dir: false,
            children: Vec::new(),
            size,
        }
    }
    
    /// Create a new directory with the given children
    fn new_dir(name: &str, children: Vec<FsNode>) -> Self {
        let size = children.iter().map(|child| child.size).sum();
        FsNode {
            name: name.to_string(),
            is_dir: true,
            children,
            size,
        }
    }
}

fn main() {
    println!("File System Traversal Example");
    println!("============================");
    
    // Create a sample file system structure
    let fs = create_sample_fs();
    
    println!("\n1. Basic Traversal");
    println!("-----------------");
    traverse_fs(&fs);
    
    println!("\n2. Find Large Files");
    println!("-----------------");
    find_large_files(&fs, 100);
    
    println!("\n3. Calculate Directory Sizes");
    println!("--------------------------");
    print_dir_sizes(&fs);
}

/// Create a sample file system structure for demonstration
fn create_sample_fs() -> FsNode {
    FsNode::new_dir("root", vec![
        FsNode::new_dir("documents", vec![
            FsNode::new_file("report.docx", 50),
            FsNode::new_file("data.xlsx", 120),
            FsNode::new_dir("drafts", vec![
                FsNode::new_file("draft1.txt", 10),
                FsNode::new_file("draft2.txt", 15),
            ]),
        ]),
        FsNode::new_dir("pictures", vec![
            FsNode::new_file("vacation.jpg", 200),
            FsNode::new_file("portrait.png", 150),
        ]),
        FsNode::new_file("config.cfg", 5),
    ])
}

/// Basic traversal of the file system, printing all nodes
fn traverse_fs(root: &FsNode) {
    println!("Traversing file system:");
    
    let mut indent_level = 0;
    
    for_tree!(node in root; |_| true; |node| {
        if node.is_dir {
            node.children.iter().collect()
        } else {
            Vec::new()
        }
    } => {
        let indent = "  ".repeat(indent_level);
        let node_type = if node.is_dir { "DIR" } else { "FILE" };
        println!("{}{}: {} ({} bytes)", indent, node_type, node.name, node.size);
        
        // Increment indent level for children
        indent_level += 1;
        
        // We need to use a special pattern to "pop" the indent level after processing children
        // We use the Drop trait's behavior to decrement after all children are processed
        struct IndentGuard<'a>(&'a mut usize);
        impl<'a> Drop for IndentGuard<'a> {
            fn drop(&mut self) {
                *self.0 -= 1;
            }
        }
        
        let _guard = IndentGuard(&mut indent_level);
    });
}

/// Find all files larger than a given size
fn find_large_files(root: &FsNode, min_size: usize) {
    println!("Finding files larger than {} bytes:", min_size);
    
    let mut large_file_count = 0;
    
    for_tree!(node in root; |_| true; |node| {
        if node.is_dir {
            node.children.iter().collect()
        } else {
            Vec::new()
        }
    } => {
        if !node.is_dir && node.size > min_size {
            println!("Large file found: {} ({} bytes)", node.name, node.size);
            large_file_count += 1;
        }
    });
    
    println!("Total large files found: {}", large_file_count);
}

/// Print all directories with their total sizes
fn print_dir_sizes(root: &FsNode) {
    println!("Directory sizes:");
    
    for_tree!(node in root; |node| node.is_dir; |node| {
        node.children.iter().filter(|child| child.is_dir).collect()
    } => {
        println!("Directory: {}, Total size: {} bytes", node.name, node.size);
    });
}