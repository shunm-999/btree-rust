use crate::btree::node::BtreeNode;
use crate::btree::{Delete, Insert, Search};

#[derive(Clone)]
pub struct Btree<V: 'static + Clone> {
    root: Option<BtreeNode<V>>,
    max_count: usize,
}

impl<V: 'static + Clone> Btree<V> {
    fn new(max_count: usize) -> Self {
        Btree {
            root: None,
            max_count,
        }
    }
}

impl<V: 'static + Clone> Search<V> for Btree<V> {
    fn search(&self, target_key: i32) -> Option<(i32, V)> {
        match &self.root {
            None => None,
            Some(root) => root.search(target_key),
        }
    }
}

impl<V: 'static + Clone> Insert<V> for Btree<V> {
    fn insert(&mut self, key: i32, value: V) {
        let mut root = self.root.take().unwrap_or(BtreeNode::new(self.max_count));
        root.insert(key, value);

        if root.is_full() {
            let ((key, value), (left, right)) = root.split_node();
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

impl<V: 'static + Clone> Delete for Btree<V> {
    fn delete(&mut self, key: i32) {
        match &mut self.root {
            None => {}
            Some(root) => {
                root.delete(key);

                if root.is_empty() {
                    self.root = None;
                }
            }
        }
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

    #[test]
    fn test_delete_basic() {
        let mut tree = Btree::new(3);
        tree.insert(10, 100);
        tree.insert(20, 200);
        tree.insert(30, 300);
        assert_eq!(tree.search(20), Some((20, 200)));
        tree.delete(20);
        assert_eq!(tree.search(20), None);
        assert_eq!(tree.search(10), Some((10, 100)));
        assert_eq!(tree.search(30), Some((30, 300)));
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut tree = Btree::new(3);
        tree.insert(10, 100);
        tree.insert(20, 200);
        tree.delete(30); // 存在しないキー
        assert_eq!(tree.search(10), Some((10, 100)));
        assert_eq!(tree.search(20), Some((20, 200)));
    }

    #[test]
    fn test_delete_all_keys() {
        let mut tree = Btree::new(3);
        tree.insert(10, 100);
        tree.insert(20, 200);
        tree.insert(30, 300);
        tree.delete(10);
        tree.delete(20);
        tree.delete(30);
        assert_eq!(tree.search(10), None);
        assert_eq!(tree.search(20), None);
        assert_eq!(tree.search(30), None);
    }

    #[test]
    fn test_delete_with_split_and_merge() {
        let mut tree = Btree::new(3);
        for i in 1..=7 {
            tree.insert(i * 10, i * 100);
        }
        // 40, 50, 60, 70を削除してマージが発生するか確認
        for k in [40, 50, 60, 70] {
            tree.delete(k);
            assert_eq!(tree.search(k), None);
        }
        // 残りのキーが正しく残っているか
        for k in [10, 20, 30] {
            assert_eq!(tree.search(k), Some((k, k * 10)));
        }
    }
}
