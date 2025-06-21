use crate::btree::{BinarySearch, Delete, Insert, Merge, Search};

#[derive(Clone)]
pub(crate) struct BtreeNode<K: 'static + Clone, V: 'static + Clone> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<BtreeNode<K, V>>>,
    max_count: usize,
}

#[derive(Clone)]
pub(crate) struct BtreeNodeEntry<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> BtreeNode<K, V> {
    pub(crate) fn new(max_count: usize) -> Self {
        Self {
            keys: vec![],
            values: vec![],
            children: vec![],
            max_count,
        }
    }

    pub(crate) fn from(
        keys: Vec<K>,
        values: Vec<V>,
        children: Vec<Box<BtreeNode<K, V>>>,
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

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> BtreeNode<K, V> {
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn current_count(&self) -> usize {
        self.keys.len()
    }
    fn min_count(&self) -> usize {
        self.max_count / 2
    }

    fn push_kv(&mut self, pair: (K, V)) {
        let (key, value) = pair;
        self.keys.push(key);
        self.values.push(value);
    }

    fn insert_entry(&mut self, index: usize, pair: (K, V)) {
        let (key, value) = pair;
        self.keys.insert(index, key);
        self.values.insert(index, value);
    }

    fn remove_head_entry(&mut self) -> (K, V) {
        let key = self.keys.remove(0);
        let value = self.values.remove(0);
        (key, value)
    }

    fn remove_tail_entry(&mut self) -> (K, V) {
        let key = self.keys.remove(self.keys.len() - 1);
        let value = self.values.remove(self.values.len() - 1);
        (key, value)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub(crate) fn is_full(&self) -> bool {
        self.keys.len() >= self.max_count
    }

    pub(crate) fn is_more_than_min_count(&self) -> bool {
        self.current_count() > self.min_count()
    }

    pub(crate) fn has_key(&self, key: &K) -> bool {
        self.keys.contains(&key)
    }

    pub(crate) fn split_node(&self) -> ((K, V), (BtreeNode<K, V>, BtreeNode<K, V>)) {
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
            (left, right)
        };
        let key = self.keys[mid_index].clone();
        let value = self.values[mid_index].clone();
        ((key, value), split)
    }

    pub(crate) fn entry(self) -> BtreeNodeEntry<K, V> {
        BtreeNodeEntry {
            keys: self.keys,
            values: self.values,
        }
    }
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> Search<K, V>
    for BtreeNode<K, V>
{
    fn search(&self, key: &K) -> Option<(K, V)> {
        match self.keys.binary_lookup(&key) {
            // key found in this node
            Ok(i) => Some((self.keys[i].clone(), self.values[i].clone())),
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

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> BtreeNode<K, V> {
    fn pop_min(&mut self) -> Option<(K, V)> {
        if !self.is_leaf() {
            return self.children[0].pop_min();
        }
        if self.is_more_than_min_count() {
            let entry = self.remove_head_entry();
            return Some(entry);
        }
        None
    }
    fn pop_max(&mut self) -> Option<(K, V)> {
        if !self.is_leaf() {
            return self.children[0].pop_max();
        }

        if self.is_more_than_min_count() {
            let entry = self.remove_tail_entry();
            return Some(entry);
        }
        None
    }
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> Insert<K, V>
    for BtreeNode<K, V>
{
    fn insert(&mut self, key: K, value: V) {
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
                        let ((key, value), (left, right)) = self.children[i].split_node();

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

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> Delete<K>
    for BtreeNode<K, V>
{
    fn delete(&mut self, key: &K) {
        match self.keys.binary_lookup(&key) {
            Ok(i) => {
                if self.is_leaf() {
                    // 葉ノードの場合はそのまま削除
                    self.keys.remove(i);
                    self.values.remove(i);
                } else {
                    // 内部ノードの場合
                    let operation = self.resolve_delete_from_self_operation();
                    self.apply_delete_from_self_operation(i, operation);
                }
            }
            Err(i) => {
                let operation = self.get_delete_from_child_operation(&key, i);
                self.apply_delete_from_child_operation(key, i, operation);
            }
        }
    }
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> Merge for BtreeNode<K, V> {
    fn merge(self, other: Self) -> Self {
        let keys = [self.keys, other.keys].concat();
        let values = [self.values, other.values].concat();
        let children = [self.children, other.children].concat();
        let max_count = self.max_count;
        Self {
            keys,
            values,
            children,
            max_count,
        }
    }
}

enum DeleteFromSelfOperation<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> {
    Replace((K, V)),
    Merge,
    MergeToSelf,
}
enum DeleteFromChildOperation {
    None,
    Delegate,
    RotateLeft,
    RotateRight,
    MergeToLeft,
    MergeToRight,
    MergeToSelf,
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> BtreeNode<K, V> {
    fn resolve_delete_from_self_operation(&mut self) -> DeleteFromSelfOperation<K, V> {
        if let Some(entry) = self.children[0].pop_max() {
            // 一番左の部分木が余分な要素を持っている
            return DeleteFromSelfOperation::Replace(entry);
        }
        if let Some(entry) = self.children[1].pop_min() {
            // 一番左から、一つ右の部分木が余分な要素を持っている
            return DeleteFromSelfOperation::Replace(entry);
        }
        if self.current_count() != 1 {
            // どちらの部分木も余分な要素をもっていない
            return DeleteFromSelfOperation::Merge;
        }
        // どちらの部分木も余分な要素をもっていないかつ、自身も余分な要素をもっていない
        DeleteFromSelfOperation::MergeToSelf
    }

    fn apply_delete_from_self_operation(
        &mut self,
        index: usize,
        operation: DeleteFromSelfOperation<K, V>,
    ) {
        match operation {
            DeleteFromSelfOperation::Replace((key, value)) => {
                self.keys[index] = key;
                self.values[index] = value;
            }
            DeleteFromSelfOperation::Merge => {
                self.keys.remove(index);
                self.values.remove(index);

                let merged = {
                    let left = self.children.remove(index);
                    let right = self.children.remove(index);
                    left.merge(right)
                };

                self.children.insert(index, merged);
            }
            DeleteFromSelfOperation::MergeToSelf => {
                self.keys.remove(index);
                self.values.remove(index);

                let merged = {
                    let left = self.children.remove(index);
                    let right = self.children.remove(index);
                    left.merge(right)
                };

                self.keys = merged.keys;
                self.values = merged.values;
                self.children = merged.children;
                self.max_count = merged.max_count;
            }
        }
    }
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> BtreeNode<K, V> {
    fn get_delete_from_child_operation(&self, key: &K, index: usize) -> DeleteFromChildOperation {
        if self.is_leaf() {
            // 葉ノードにkeyが存在しない
            return DeleteFromChildOperation::None;
        }
        if !self.children[index].is_leaf() {
            // 子ノードが内部ノード
            return DeleteFromChildOperation::Delegate;
        }

        if !self.children[index].has_key(key) {
            // 子ノードが葉ノードかつ、keyが存在しない
            return DeleteFromChildOperation::None;
        }

        if self.children[index].is_more_than_min_count() {
            // 子ノードが葉ノードかつ、minCountより大きいなら再起的に処理する
            return DeleteFromChildOperation::Delegate;
        }
        if index == self.children.len() - 1 && self.children[index - 1].is_more_than_min_count() {
            // 子ノードが一番右かつ、一つ左が十分な要素を持っている
            return DeleteFromChildOperation::RotateRight;
        }
        if index < self.children.len() - 1 && self.children[index + 1].is_more_than_min_count() {
            // 子ノードが一番右以外かつ、一つ右が十分な要素を持っている
            return DeleteFromChildOperation::RotateLeft;
        }
        if self.current_count() != 1 && index == self.children.len() - 1 {
            // 子ノードが一番右かつ、一つ左が十分な要素を持っていない
            return DeleteFromChildOperation::MergeToLeft;
        }
        if self.current_count() != 1 && index < self.children.len() - 1 {
            // 子ノードが一番右以外かつ、一つ右が十分な要素を持っていない
            return DeleteFromChildOperation::MergeToRight;
        }
        // 自身の要素が１
        DeleteFromChildOperation::MergeToSelf
    }

    fn apply_delete_from_child_operation(
        &mut self,
        key: &K,
        index: usize,
        operation: DeleteFromChildOperation,
    ) {
        match operation {
            DeleteFromChildOperation::None => {}
            DeleteFromChildOperation::Delegate => {
                self.children[index].delete(key);
            }
            DeleteFromChildOperation::RotateLeft => {
                self.children[index].delete(key);

                let head = self.remove_head_entry();
                self.children[index].push_kv(head);

                let head = self.children[index + 1].remove_head_entry();
                self.insert_entry(0, head);
            }
            DeleteFromChildOperation::RotateRight => {
                self.children[index].delete(key);

                let tail = self.remove_tail_entry();
                self.children[index].insert_entry(0, tail);

                let tail = self.children[index - 1].remove_tail_entry();
                self.push_kv(tail);
            }
            DeleteFromChildOperation::MergeToLeft => {
                self.children[index].delete(key);

                let tail = self.remove_tail_entry();
                self.children[index - 1].push_kv(tail);

                if let Some(child) = self.children.pop() {
                    for entry in child.entry() {
                        self.children[index - 1].push_kv(entry);
                    }
                }
            }
            DeleteFromChildOperation::MergeToRight => {
                self.children[index].delete(key);

                let head = self.remove_head_entry();
                self.children[index + 1].insert_entry(0, head);

                let child = self.children.remove(0);
                for entry in child.entry().rev() {
                    self.children[index + 1].insert_entry(0, entry);
                }
            }
            DeleteFromChildOperation::MergeToSelf => {
                let (new_keys, new_values) = {
                    let mut left = self.children.remove(0);
                    let mut right = self.children.remove(0);

                    if left.has_key(&key) {
                        left.delete(key);
                    }
                    if right.has_key(&key) {
                        right.delete(key);
                    }
                    let new_keys = {
                        let mut new_keys = left.keys;
                        new_keys.extend(self.keys.clone());
                        new_keys.extend(right.keys);
                        new_keys
                    };

                    let new_values = {
                        let mut new_values = left.values;
                        new_values.extend(self.values.clone());
                        new_values.extend(right.values);
                        new_values
                    };
                    (new_keys, new_values)
                };

                self.keys = new_keys;
                self.values = new_values;
            }
        }
    }
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> Iterator
    for BtreeNodeEntry<K, V>
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.keys.is_empty() {
            return None;
        }
        Some((self.keys.remove(0), self.values.remove(0)))
    }
}

impl<K: 'static + Clone + PartialEq + PartialOrd, V: 'static + Clone> DoubleEndedIterator
    for BtreeNodeEntry<K, V>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let key = self.keys.pop()?;
        let value = self.values.pop()?;
        Some((key, value))
    }
}
