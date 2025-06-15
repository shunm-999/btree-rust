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

mod tests {
    use super::*;

    #[test]
    fn test_empty_tree_search() {
        let tree = Btree::new(3);
        assert_eq!(tree.search(1), None);
    }

    #[test]
    fn test_insert_and_search() {
        let mut tree = Btree::new(3);

        // 基本的な挿入と検索のテスト
        tree.insert(10, 100);
        assert_eq!(tree.search(10), Some((10, 100)));

        // 存在しないキーの検索
        assert_eq!(tree.search(20), None);

        // 複数の要素の挿入と検索
        tree.insert(5, 50);
        tree.insert(15, 150);

        assert_eq!(tree.search(5), Some((5, 50)));
        assert_eq!(tree.search(15), Some((15, 150)));
    }

    #[test]
    fn test_insert_same_key() {
        let mut tree = Btree::new(3);

        // 同じキーに対する上書きのテスト
        tree.insert(10, 100);
        tree.insert(10, 200);

        assert_eq!(tree.search(10), Some((10, 200)));
    }
}
