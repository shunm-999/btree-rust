mod node;
pub(crate) mod tree;

pub(crate) trait Search<K, V> {
    fn search(&self, key: &K) -> Option<(K, V)>;
}

pub(crate) trait Insert<K, V> {
    fn insert(&mut self, key: K, value: V);
}

pub(crate) trait Delete<K> {
    fn delete(&mut self, key: &K);
}

pub(crate) trait Merge {
    fn merge(self, other: Self) -> Self;
}

impl<T: Merge> Merge for Box<T> {
    fn merge(self, other: Self) -> Self {
        Box::new((*self).merge(*other))
    }
}

pub(crate) trait BinarySearch<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_lookup_existing_elements() {
        let arr = vec![10, 20, 30, 40, 50];

        // 存在する要素の検索
        assert_eq!(arr.binary_lookup(&10), Ok(0));
        assert_eq!(arr.binary_lookup(&30), Ok(2));
        assert_eq!(arr.binary_lookup(&50), Ok(4));
    }

    #[test]
    fn test_binary_lookup_missing_elements() {
        let arr = vec![10, 20, 30, 40, 50];

        // 存在しない要素の検索
        assert_eq!(arr.binary_lookup(&15), Err(1)); // 中間値
        assert_eq!(arr.binary_lookup(&5), Err(0)); // 最小値より小さい
        assert_eq!(arr.binary_lookup(&60), Err(5)); // 最大値より大きい
    }

    #[test]
    fn test_binary_lookup_empty_array() {
        // 空の配列での検索
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.binary_lookup(&10), Err(0));
    }

    #[test]
    fn test_binary_lookup_single_element() {
        // 1つの要素のみの配列での検索
        let single = vec![10];
        assert_eq!(single.binary_lookup(&10), Ok(0));
        assert_eq!(single.binary_lookup(&5), Err(0));
        assert_eq!(single.binary_lookup(&15), Err(1));
    }

    #[test]
    fn test_binary_lookup_duplicates() {
        // 重複要素のある配列での検索
        let duplicates = vec![10, 10, 20, 20, 30];
        assert_eq!(duplicates.binary_lookup(&10), Ok(0));
        assert_eq!(duplicates.binary_lookup(&20), Ok(2));
        assert_eq!(duplicates.binary_lookup(&30), Ok(4));
    }

    #[test]
    fn test_binary_lookup_negative_numbers() {
        let arr = vec![-50, -30, -10, 10, 30, 50];

        // 負の数の検索
        assert_eq!(arr.binary_lookup(&-50), Ok(0));
        assert_eq!(arr.binary_lookup(&-30), Ok(1));
        assert_eq!(arr.binary_lookup(&-10), Ok(2));
    }

    #[test]
    fn test_binary_lookup_mixed_numbers() {
        let arr = vec![-50, -30, -10, 10, 30, 50];

        // 負の数と正の数の間の値の検索
        assert_eq!(arr.binary_lookup(&-20), Err(2));
        assert_eq!(arr.binary_lookup(&0), Err(3));
        assert_eq!(arr.binary_lookup(&20), Err(4));
    }
}
