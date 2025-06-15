mod node;
mod tree;

pub(crate) trait Search {
    fn search(&self, key: i32) -> Option<(i32, i32)>;
}

pub(crate) trait Insert {
    fn insert(&mut self, key: i32, value: i32);
}
