use crate::btree::{BinarySearch, Delete, Insert, Search};

#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
    max_count: usize,
}

#[derive(Clone)]
pub(crate) struct BtreeNodeEntry {
    keys: Vec<i32>,
    values: Vec<i32>,
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
        self.max_count / 2
    }

    fn push_kv(&mut self, pair: (i32, i32)) {
        let (key, value) = pair;
        self.keys.push(key);
        self.values.push(value);
    }

    fn insert_entry(&mut self, index: usize, pair: (i32, i32)) {
        let (key, value) = pair;
        self.keys.insert(index, key);
        self.values.insert(index, value);
    }

    fn remove_head_entry(&mut self) -> (i32, i32) {
        let key = self.keys.remove(0);
        let value = self.values.remove(0);
        (key, value)
    }

    fn remove_tail_entry(&mut self) -> (i32, i32) {
        let key = self.keys.remove(self.keys.len() - 1);
        let value = self.values.remove(self.values.len() - 1);
        (key, value)
    }

    pub(crate) fn is_full(&self) -> bool {
        self.keys.len() >= self.max_count
    }

    pub(crate) fn is_more_than_min_count(&self) -> bool {
        self.current_count() > self.min_count()
    }

    pub(crate) fn has_key(&self, key: i32) -> bool {
        self.keys.contains(&key)
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

    pub(crate) fn entry(self) -> BtreeNodeEntry {
        BtreeNodeEntry {
            keys: self.keys,
            values: self.values,
        }
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
                } else {
                    // 内部ノードの場合
                    // a: 左側がminCount以上
                }
            }
            Err(i) => match self.get_delete_from_child_operation(key, i) {
                DeleteFromChildOperation::None => {}
                DeleteFromChildOperation::Delete => {
                    self.children[i].delete(key);
                }
                DeleteFromChildOperation::RotateLeft => {
                    self.children[i].delete(key);

                    let head = self.remove_head_entry();
                    self.children[i].push_kv(head);

                    let head = self.children[i + 1].remove_head_entry();
                    self.insert_entry(0, head);
                }
                DeleteFromChildOperation::RotateRight => {
                    self.children[i].delete(key);

                    let tail = self.remove_tail_entry();
                    self.children[i].insert_entry(0, tail);

                    let tail = self.children[i - 1].remove_tail_entry();
                    self.push_kv(tail);
                }
                DeleteFromChildOperation::MergeToLeft => {
                    self.children[i].delete(key);

                    let tail = self.remove_tail_entry();
                    self.children[i - 1].push_kv(tail);

                    if let Some(child) = self.children.pop() {
                        for entry in child.entry() {
                            self.children[i - 1].push_kv(entry);
                        }
                    }
                }
                DeleteFromChildOperation::MergeToRight => {
                    self.children[i].delete(key);

                    let head = self.remove_head_entry();
                    self.children[i + 1].insert_entry(0, head);

                    let child = self.children.remove(0);
                    for entry in child.entry().rev() {
                        self.children[i + 1].insert_entry(0, entry);
                    }
                }
                DeleteFromChildOperation::MergeToSelf => {}
            },
        }
    }
}

enum DeleteFromChildOperation {
    None,
    Delete,
    RotateLeft,
    RotateRight,
    MergeToLeft,
    MergeToRight,
    MergeToSelf,
}

impl BtreeNode {
    fn get_delete_from_child_operation(&self, key: i32, index: usize) -> DeleteFromChildOperation {
        if self.is_leaf() {
            // 葉ノードにkeyが存在しない
            return DeleteFromChildOperation::None;
        }
        if !self.children[index].is_leaf() {
            // 子ノードが内部ノード
            return DeleteFromChildOperation::Delete;
        }

        if !self.children[index].has_key(key) {
            // 子ノードが葉ノードかつ、keyが存在しない
            return DeleteFromChildOperation::None;
        }

        if self.children[index].is_more_than_min_count() {
            // 子ノードが葉ノードかつ、minCountより大きいなら再起的に処理する
            return DeleteFromChildOperation::Delete;
        }
        if index == self.children.len() - 1 && self.children[index - 1].is_more_than_min_count() {
            // 子ノードが一番右かつ、一つ左が十分なノードを持っている
            return DeleteFromChildOperation::RotateRight;
        }
        if index < self.children.len() - 1 && self.children[index + 1].is_more_than_min_count() {
            // 子ノードが一番左かつ、一つ右が十分なノードを持っている
            return DeleteFromChildOperation::RotateLeft;
        }
        if self.current_count() != 1 && index == self.children.len() - 1 {
            // 子ノードが一番右かつ、一つ左が十分なノードを持っていない
            return DeleteFromChildOperation::MergeToLeft;
        }
        if self.current_count() != 1 && index < self.children.len() - 1 {
            // 子ノードが一番左かつ、一つ右が十分なノードを持っていない
            return DeleteFromChildOperation::MergeToRight;
        }
        DeleteFromChildOperation::MergeToSelf
    }
}

impl Iterator for BtreeNodeEntry {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.keys.is_empty() {
            return None;
        }
        Some((self.keys.remove(0), self.values.remove(0)))
    }
}

impl DoubleEndedIterator for BtreeNodeEntry {
    fn next_back(&mut self) -> Option<Self::Item> {
        let key = self.keys.pop()?;
        let value = self.values.pop()?;
        Some((key, value))
    }
}
