mod node;
mod tree;

pub(crate) trait Search {
    fn search(&self, key: i32) -> Option<(i32, i32)>;
}

pub(crate) trait Insert {
    fn insert(&mut self, key: i32, value: i32);
}

pub(crate) trait Delete {
    fn delete(&mut self, key: i32);
}

pub(crate) trait BinarySearch<T: 'static> {
    fn binary_lookup(&self, key: &T) -> Result<usize, usize>;
}

impl<T: 'static + PartialEq + PartialOrd> BinarySearch<T> for Vec<T> {
    fn binary_lookup(&self, key: &T) -> Result<usize, usize> {
        for (index, node) in self.iter().enumerate() {
            if key == node {
                return Ok(index);
            }
            if key < node {
                return Err(index);
            }
        }
        Err(self.len())
    }
}
