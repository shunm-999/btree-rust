#[derive(Clone)]
pub(crate) struct BtreeNode {
    keys: Vec<i32>,
    values: Vec<i32>,
    children: Vec<Box<BtreeNode>>,
    is_leaf: bool,
}

pub(crate) enum SearchResult {
    Found(i32, i32),
    Next(BtreeNode),
    NotFound,
}

pub(crate) trait Search {
    fn search(&self, key: i32) -> SearchResult;
}

impl Search for BtreeNode {
    fn search(&self, key: i32) -> SearchResult {
        let target_key = key;
        let mut found_index = None;

        self.keys.iter().enumerate().for_each(|(index, key)| {
            if key == &target_key {
                found_index = Some(index);
            }
        });

        if found_index != None {
            // そのノード内でkeyがヒットしたら、値を返す
            return SearchResult::Found(target_key, self.values[found_index.unwrap()]);
        }
        if found_index == None && self.is_leaf {
            // keyがヒットしないかつ、根ノードの場合はkeyは存在しない
            return SearchResult::NotFound;
        }

        let mut child_index = None;
        self.keys.windows(2).enumerate().for_each(|(index, pair)| {
            let (previous, next) = (&pair[0], &pair[1]);
            if previous < &target_key && &target_key < next {
                child_index = Some(index);
            }
        });

        if child_index != None {
            let child = self.children[child_index.unwrap() as usize].clone();
            SearchResult::Next(*child)
        } else {
            SearchResult::NotFound
        }
    }
}
