#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
    is_leaf: bool,
}

pub(crate) trait Search {
    fn search(&self, target_key: i32) -> Option<i32>;
}

impl Search for BtreeNode {
    fn search(&self, target_key: i32) -> Option<i32> {
        let mut found_index = None;

        self.keys.iter().enumerate().for_each(|(index, key)| {
            if key == &target_key {
                found_index = Some(index);
            }
        });

        if found_index != None {
            // そのノード内でkeyがヒットしたら、値を返す
            return Some(self.values[found_index.unwrap()]);
        }
        if found_index == None && self.is_leaf {
            // keyがヒットしないかつ、根ノードの場合はkeyは存在しない
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
