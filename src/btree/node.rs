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
        if let Some(i) = self.keys.iter().position(|&k| k == target_key) {
            return Some((self.keys[i], self.values[i]));
        }
        if self.is_leaf {
            // If the key is not found and this is a leaf, the search ends here.
            return None;
        }

        let child_index = self
            .keys
            .iter()
            .position(|&k| target_key <= k)
            .unwrap_or(self.keys.len());

        self.children[child_index].search(target_key)
    }
}
