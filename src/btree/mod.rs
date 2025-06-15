mod node;
mod tree;

pub(crate) trait Search {
    fn search(&self, target_key: i32) -> Option<i32>;
}
