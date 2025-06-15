use crate::btree::{Insert, Search};

#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
}

struct Split {
    left: BtreeNode,
    right: BtreeNode,
}

impl BtreeNode {
    pub(crate) fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
            children: vec![],
        }
    }

    pub(crate) fn from(keys: Vec<i32>, values: Vec<i32>) -> Self {
        Self {
            keys,
            values,
            children: vec![],
        }
    }
}

impl BtreeNode {
    pub(crate) fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub(crate) fn is_full(&self, max_count: usize) -> bool {
        self.keys.len() >= max_count
    }

    pub(crate) fn split_node(&self) -> ((i32, i32), Split) {
        let mid_index = self.keys.len() / 2;
        let split = {
            let left = BtreeNode::from(
                self.keys[..mid_index].to_vec(),
                self.values[..mid_index].to_vec(),
            );
            let right = BtreeNode::from(
                self.keys[mid_index + 1..].to_vec(),
                self.values[mid_index + 1..].to_vec(),
            );
            Split { left, right }
        };
        let key = self.keys[mid_index];
        let value = self.values[mid_index];
        ((key, value), split)
    }
}

impl Search for BtreeNode {
    fn search(&self, key: i32) -> Option<(i32, i32)> {
        match self.keys.binary_search(&key) {
            // key found in this node
            Ok(i) => Some((self.keys[i], self.values[i])),
            // key not found, recurse into appropriate child node
            Err(i) => {
                if self.is_leaf() {
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
                if self.is_leaf() {
                    self.keys.insert(i, key);
                    self.values.insert(i, value);
                } else {
                    self.children[i].insert(key, value);
                }
            }
        }
    }
}
