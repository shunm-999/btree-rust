use crate::btree::node::BtreeNode;
use crate::btree::{Insert, Search};

#[derive(Clone)]
pub(crate) struct Btree {
    root: Option<BtreeNode>,
    max_count: usize,
}

impl Btree {
    fn new(max_count: usize) -> Self {
        Btree {
            root: None,
            max_count,
        }
    }
}

impl Search for Btree {
    fn search(&self, target_key: i32) -> Option<(i32, i32)> {
        match &self.root {
            None => None,
            Some(root) => root.search(target_key),
        }
    }
}

impl Insert for Btree {
    fn insert(&mut self, key: i32, value: i32) {
        let mut root = self.root.take().unwrap_or(BtreeNode::new());
        root.insert(key, value);
        self.root = Some(root);
    }
}
