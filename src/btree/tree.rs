use crate::btree::Search;
use crate::btree::node::BtreeNode;

#[derive(Clone)]
pub(crate) struct Btree {
    root: Option<BtreeNode>,
    degree: usize,
}

impl Btree {
    fn new(degree: usize) -> Self {
        Btree { root: None, degree }
    }
}

impl Search for Btree {
    fn search(&self, target_key: i32) -> Option<i32> {
        match &self.root {
            None => None,
            Some(root) => root.search(target_key),
        }
    }
}
