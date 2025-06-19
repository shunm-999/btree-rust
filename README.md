# btree-rust

A simple B-tree implementation in Rust.

## Features
- Configurable maximum number of elements per node (`max_count`)
- Supports insertion, search, and deletion
- Simple and easy-to-use API

## Usage

### Add as a dependency
Add the following to your `Cargo.toml` (replace the URL with your repository):

```
[dependencies]
btree-rust = { git = "https://github.com/shunm-999/btree-rust" }
```

### Example
```rust
use btree_rust::Btree;

fn main() {
    // Create a B-tree with a maximum of 3 elements per node
    let mut tree = Btree::new(3);

    // Insert elements
    tree.insert(10, 100);
    tree.insert(20, 200);
    tree.insert(30, 300);

    // Search for a key
    if let Some((k, v)) = tree.search(20) {
        println!("key: {}, value: {}", k, v);
    }

    // Delete a key
    tree.delete(20);
    assert_eq!(tree.search(20), None);
}
```

## Public API
- `Btree::new(max_count: usize)` : Create a new B-tree
- `insert(&mut self, key: i32, value: i32)` : Insert a key-value pair
- `search(&self, key: i32) -> Option<(i32, i32)>` : Search for a key
- `delete(&mut self, key: i32)` : Delete a key

## License
MIT 