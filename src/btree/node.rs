use crate::btree::{Insert, Search};

#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
    is_leaf: bool,
}

impl BtreeNode {
    fn count(&self) -> usize {
        self.keys.len()
    }
}

impl Search for BtreeNode {
    fn search(&self, key: i32) -> Option<(i32, i32)> {
        match self.keys.binary_search(&key) {
            // key found in this node
            Ok(i) => Some((self.keys[i], self.values[i])),
            // key not found, recurse into appropriate child node
            Err(i) => {
                if self.is_leaf {
                    None // key not found, and no children to search
                } else {
                    self.children[i].search(key)
                }
            }
        }
    }
}

impl Insert for BtreeNode {
    fn insert(&mut self, key: i32, value: i32) {
        match self.keys.binary_search(&key) {
            Ok(i) => {
                self.keys[i] = key;
                self.values[i] = value;
            }
            Err(i) => {
                if self.is_leaf {
                    self.keys.insert(i, key);
                    self.values.insert(i, value);
                } else {
                    self.children[i].insert(key, value);
                }
            }
        }
    }
}
