use crate::btree::node::{BtreeNode, NodeSplit};
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
        let mut root = self.root.take().unwrap_or(BtreeNode::new(self.max_count));
        root.insert(key, value);

        if root.is_full() {
            let ((key, value), NodeSplit { left, right }) = root.split_node();
            root = BtreeNode::from(
                vec![key],
                vec![value],
                vec![Box::new(left), Box::new(right)],
                self.max_count,
            );
        }

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

    #[test]
    fn test_node_split() {
        let mut tree = Btree::new(3); // max_count = 3 のノード
        
        // ノードがいっぱいになるまで挿入
        tree.insert(10, 100);
        tree.insert(20, 200);
        tree.insert(30, 300);
        
        // 4つ目の要素を挿入して分割を発生させる
        tree.insert(40, 400);
        
        // 分割後も全ての要素が正しく検索できることを確認
        assert_eq!(tree.search(10), Some((10, 100)));
        assert_eq!(tree.search(20), Some((20, 200)));
        assert_eq!(tree.search(30), Some((30, 300)));
        assert_eq!(tree.search(40), Some((40, 400)));
    }

    #[test]
    fn test_multiple_splits() {
        let mut tree = Btree::new(3);
        
        // 複数回の分割を発生させる
        for i in 1..=10 {
            tree.insert(i * 10, i * 100);
        }
        
        // 全ての要素が正しく検索できることを確認
        for i in 1..=10 {
            assert_eq!(tree.search(i * 10), Some((i * 10, i * 100)));
        }
    }
}
