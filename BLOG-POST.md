# Prompt Engineering the Perfect Vibe Code: Claude's Arboriter Implementation

Ever wondered what happens when you feed a thought-provoking programming concept to an AI and ask it to implement it? I did exactly that with Tyler Glaiel's blog post about tree traversal primitives, and the results from Claude 3.7 Sonnet were nothing short of fascinating.

## The Challenge: From Blog Post to Working Rust Crate

It all started with Tyler Glaiel's insightful post titled ["Programming languages should have a tree traversal primitive"](https://blog.tylerglaiel.com/p/programming-languages-should-have). In it, Tyler articulated a common frustration many developers face: while programming languages have elegant primitives for linear traversal (`for` loops), they lack equivalent elegance when dealing with tree structures.

Tyler proposed a syntax that looked like this:

```cpp
for_tree(Node* N = mytreeroot; N != NULL; N : {N->left, N->right}){
    print(N->value);
}
```

The simplicity and elegance of this concept struck me. Rather than tackling this implementation challenge myself, I wondered: could Claude do this justice? Could it take this concept and craft a proper Rust implementation without getting lost in the complexity of macros, recursion, and borrow checking?

## The Prompt Structure: Guiding Claude's Problem-Solving Process

For this experiment, I didn't just paste the blog post and ask Claude to code something up. Instead, I created a custom prompt structure that guided Claude to approach the problem like an experienced developer would – with clear planning, systematic reasoning, and reflective practice.

The prompt framework I designed required Claude to use specific XML-tagged blocks:

1. **Plan tasks** in a structured format with clear steps
2. **Reason through problems** by explicitly articulating its thought process
3. **Make code changes** in an organized way, focusing on one issue at a time
4. **Summarize completed work** to reflect on progress and identify next steps

This structured approach essentially forced Claude to "show its work" – not just provide code, but explain why certain decisions were made and how problems were approached.

## Inside Claude's Mind: Examples of Planning and Reasoning

What made this collaboration particularly fascinating was seeing the structured thinking process Claude used. Here's what Claude's planning looked like at the start of the project:

```xml
<planning title="Implement tree traversal primitive in Rust">
  <step>Analyze the requirements for the tree traversal primitive</step>
  <step>Design the macro API to be intuitive and similar to the blog post example</step>
  <step>Implement the core recursive traversal function</step>
  <step>Create the macro that provides the user-friendly syntax</step>
  <step>Add support for break, continue, and prune control flow</step>
  <step>Test the implementation with various examples from the blog post</step>
  <step>Document the usage with comments and examples</step>
</planning>
```

This wasn't just a formality—Claude actually followed this plan step by step, addressing each aspect systematically.

When facing specific challenges, Claude's reasoning process was equally structured. For example, when deciding how to handle control flow in the traversal, Claude reasoned:

```xml
<reasoning>
The blog post describes a tree traversal primitive that should:

1. Allow easy traversal of tree-like structures without explicit recursion
2. Support a syntax similar to a for loop: for_tree(init; condition; branch) { body }
3. Support control flow with break, continue, return, and a new "prune" operation
4. Handle both in-memory trees (like Node structures) and imperative trees
   (like generating strings)
5. Be intuitive and less error-prone than writing recursive functions

In Rust, we can implement this using macros. The challenging part will be supporting
the special control flow operations. Standard break/continue won't work naturally with
recursion, so we'll need to create a special enum to represent these operations and
have the macro generate code that handles them properly.

For the "prune" operation, we can return early from the current recursive call without
processing children. For "break", we'll need to propagate a special return value up
through the call stack.
</reasoning>
```

This kind of explicit reasoning led to more thoughtful design decisions than simply trying to code the solution directly.

When Claude encountered errors during implementation, the structured approach really showed its value. Here's how Claude reasoned through a type mismatch error:

```xml
<reasoning>
Looking at the error message:

error[E0308]: mismatched types
  --> src/lib.rs:316:30
   |
316 |         for_tree!(n = 0; n < 10; vec![n + 1] => {
   |                              ^^ expected `&_`, found integer

The issue is in how we're handling references in the macro. When using the simplified
syntax `n = 0`, the macro is expanding in a way that treats `n` as a reference when
checking the condition, but the comparison `n < 10` is trying to compare a reference
with an integer directly.

We need to ensure consistent reference handling throughout the macro. We have
two options:
1. Update the condition function to dereference automatically
2. Modify how the simplified syntax expands to handle this case

I think option 1 is cleaner - we should ensure that when the condition function
receives a reference, it properly dereferences it before making comparisons.
</reasoning>
```

This systematic approach to debugging led to a more robust solution than just trying quick fixes.

## The Development Process: From Concept to Code

When presented with the task, Claude methodically implemented each part of the design plan. For the core functionality, it created a `TreeControl` enum to handle different traversal behaviors:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeControl {
    /// Continue traversal normally
    Continue,
    /// Skip traversing children of the current node
    Prune,
    /// Break out of the entire traversal
    Break,
}
```

And implemented a flexible traversal function that could work with any tree-like structure:

```rust
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
```

After completing major components, Claude would summarize the progress and reflect on the implementation:

```xml
<summary>
I've implemented a tree traversal primitive for Rust using macros, as described in
Tyler Glaiel's blog post. The implementation features:

1. A clean, intuitive syntax similar to for loops:
   `for_tree!(var = init; condition; branches => { body })`

2. Support for special control flow operations:
   - `break_tree!()` - exits the entire traversal
   - `prune!()` - skips traversing children of the current node

3. Multiple syntax variants for flexibility:
   - Using the 'in' keyword: `for_tree!(node in root; condition; branches)`
   - Using '=' for simpler cases: `for_tree!(node = root; condition; branches)`

4. Works with both in-memory tree structures and imperative tree generation

5. Comprehensive examples and tests showing various use cases

The implementation is a zero-cost abstraction - it compiles down to efficient code
equivalent to hand-written recursive functions while providing a much more
intuitive interface.
</summary>
```

These structured reflections helped guide future improvements and ensure nothing was missed.

## The Result: Arboriter Is Born

After several iterations of implementation and refinement, the result was impressive—a working Rust crate called "arboriter" that fully implemented Tyler's vision. The syntax was clean and intuitive:

```rust
for_tree!(node in &root; |_| true; |node| {
    let mut children: Vec<&BinaryNode<i32>> = Vec::new();
    if let Some(left) = &node.left {
        children.push(left.as_ref());
    }
    if let Some(right) = &node.right {
        children.push(right.as_ref());
    }
    children
} => {
    println!("Value: {}", node.value);

    if node.value == 10 {
        break_tree!(); // Exit the traversal
    }
});
```

But Claude didn't stop at just the implementation. The structured development approach also produced:

1. Comprehensive documentation with clear examples
2. A complete test suite covering core functionality
3. Example programs demonstrating practical applications
4. A properly organized crate structure ready for publication

## Beyond Basic Trees: Creative Applications

What surprised me most was how Claude extended the concept beyond basic tree traversal to showcase the versatility of the primitive:

### Generating All Strings of a Given Alphabet

```rust
for_tree!(s in String::new(); |s| s.len() <= 3; |s| {
    let mut branches: Vec<String> = Vec::new();
    for c in alphabet.chars() {
        branches.push(format!("{}{}", s, c));
    }
    branches
} => {
    println!("Generated: \"{}\"", s);

    if s.len() == 3 {
        prune!(); // Don't generate longer strings
    }
});
```

### Creating Balanced Parentheses

```rust
// Generate all valid balanced parentheses strings with at most n pairs
for_tree!(state in ("".to_string(), 0, 0); |state| state.1 <= n; |state| {
    let (s, open, close) = state;
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
    let (s, open, close) = state;

    // Only print fully balanced strings
    if open == close && open > 0 {
        println!("Generated: {}", s);
    }
});
```

### Traversing File System Structures

```rust
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
```

## The Final Push: Polishing for Publication

Toward the end of the project, Claude demonstrated remarkable attention to detail. When preparing the crate for publication, Claude's reasoning showed careful consideration of the user experience:

```xml
<reasoning>
Before publication, we need to ensure the crate meets high-quality standards:

1. Documentation is crucial - users need to understand how to use this primitive
   without extensive training. Each function and macro should have examples.

2. Error messages need to be clear and helpful - the macro should produce
   understandable errors when used incorrectly.

3. We should have a comprehensive set of examples covering different use cases
   to inspire users.

4. The API should follow Rust conventions and be consistent with how other
   macros in the ecosystem work.

For the crate name, "arboriter" combines "arbor" (tree) with "iter" (iteration),
while also hinting at "arborist" (tree specialist). This name effectively
communicates the purpose while being memorable and unique in the Rust ecosystem.
</reasoning>
```

This kind of holistic thinking about the entire user experience elevated the project from merely functional code to a polished, publishable crate.

## Lessons Learned: The Impact of Structured AI Collaboration

This experiment revealed several valuable insights about working with Claude on complex programming tasks:

### 1. Structure Creates Better Thinking

By requiring Claude to plan, reason, implement, and reflect separately, the quality of its output dramatically improved. The structure didn't constrain creativity—it enhanced it by enforcing methodical problem-solving.

### 2. Explicit Reasoning Leads to Better Solutions

When Claude was required to explain its reasoning before implementing solutions, the resulting code was more robust and correct. This mirrors how human developers often benefit from rubber-duck debugging or writing design documents.

### 3. Iterative Refinement Is Key

The development wasn't a one-shot perfect implementation. It took several rounds of testing and refinement, with Claude learning from errors and improving its approach. This mirrors real-world software development.

### 4. Documentation Becomes Part of the Process

Because documentation was built into the structured approach, the resulting code was well-documented from the start—not as an afterthought. This produced a much more usable final product.

## Conclusion: A New Era of Collaborative Programming

The arboriter project demonstrates the exciting possibilities of human-AI programming collaboration. By providing both a concept (via Tyler's blog post) and a structured development framework, I was able to guide Claude to create a sophisticated, well-implemented Rust crate that I couldn't have built as quickly on my own.

This doesn't mean AI will replace programmers—far from it. Instead, it suggests a future where programmers can operate at a higher level of abstraction, focusing on concepts and design while collaborating with AI to handle implementation details. The key insight is that how we prompt and structure these collaborations dramatically affects the quality of the results.

The code for arboriter is available on GitHub, and I encourage you to check it out. It stands as a testament to what's possible when human guidance meets AI implementation capability, especially when that collaboration is deliberately structured to bring out the best in both parties.
