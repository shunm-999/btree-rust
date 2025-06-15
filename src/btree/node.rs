use crate::btree::Search;

#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
    is_leaf: bool,
}

impl Search for BtreeNode {
    fn search(&self, target_key: i32) -> Option<(i32, i32)> {
        match self.keys.binary_search(&target_key) {
            // key found in this node
            Ok(i) => Some((self.keys[i], self.values[i])),
            // key not found, recurse into appropriate child node
            Err(i) => {
                if self.is_leaf {
                    None  // key not found, and no children to search
                } else {
                    self.children[i].search(target_key)
                }
            }
        }
    }
}
