use crate::btree::{BinarySearch, Delete, Insert, Search};

#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
    max_count: usize,
}

pub(crate) struct NodeSplit {
    pub(crate) left: BtreeNode,
    pub(crate) right: BtreeNode,
}

impl BtreeNode {
    pub(crate) fn new(max_count: usize) -> Self {
        Self {
            keys: vec![],
            values: vec![],
            children: vec![],
            max_count,
        }
    }

    pub(crate) fn from(
        keys: Vec<i32>,
        values: Vec<i32>,
        children: Vec<Box<BtreeNode>>,
        max_count: usize,
    ) -> Self {
        Self {
            keys,
            values,
            children,
            max_count,
        }
    }
}

impl BtreeNode {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    fn current_count(&self) -> usize {
        self.keys.len()
    }
    fn min_count(&self) -> usize {
        (self.max_count - 1) / 2
    }

    fn remove_head(&mut self) -> (i32, i32) {
        let key = self.keys.remove(0);
        let value = self.values.remove(0);
        (key, value)
    }

    fn remove_tail(&mut self) -> (i32, i32) {
        let key = self.keys.remove(self.keys.len() - 1);
        let value = self.values.remove(self.values.len() - 1);
        (key, value)
    }

    pub(crate) fn is_full(&self) -> bool {
        self.keys.len() >= self.max_count
    }

    pub(crate) fn is_less_than_min_count(&self) -> bool {
        self.current_count() < self.min_count()
    }

    pub(crate) fn split_node(&self) -> ((i32, i32), NodeSplit) {
        let mid_index = self.keys.len() / 2;
        let split = {
            let left = BtreeNode::from(
                self.keys[..mid_index].to_vec(),
                self.values[..mid_index].to_vec(),
                if self.children.is_empty() {
                    vec![]
                } else {
                    self.children[..=mid_index].to_vec()
                },
                self.max_count,
            );
            let right = BtreeNode::from(
                self.keys[mid_index + 1..].to_vec(),
                self.values[mid_index + 1..].to_vec(),
                if self.children.is_empty() {
                    vec![]
                } else {
                    self.children[mid_index + 1..].to_vec()
                },
                self.max_count,
            );
            NodeSplit { left, right }
        };
        let key = self.keys[mid_index];
        let value = self.values[mid_index];
        ((key, value), split)
    }
}

impl Search for BtreeNode {
    fn search(&self, key: i32) -> Option<(i32, i32)> {
        match self.keys.binary_lookup(&key) {
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
        match self.keys.binary_lookup(&key) {
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

                    if self.children[i].is_full() {
                        let ((key, value), NodeSplit { left, right }) =
                            self.children[i].split_node();

                        self.keys.insert(i, key);
                        self.values.insert(i, value);

                        self.children[i] = Box::new(left);
                        self.children.insert(i + 1, Box::new(right));
                    }
                }
            }
        }
    }
}

impl Delete for BtreeNode {
    fn delete(&mut self, key: i32) {
        match self.keys.binary_lookup(&key) {
            Ok(i) => {
                if self.is_leaf() {
                    // 葉ノードの場合はそのまま削除
                    self.keys.remove(i);
                    self.values.remove(i);
                    return;
                }
                // 内部ノードの場合
                // a: 左側がminCount以上
            }
            Err(i) => {
                if self.is_leaf() {
                    // keyが存在しない
                    return;
                }
                self.children[i].delete(key);
            }
        }
    }
}
