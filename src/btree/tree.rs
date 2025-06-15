use crate::btree::node::BtreeNode;

pub(crate) struct Btree {
    root: Option<BtreeNode>,
    degree: usize,
}

impl Btree {
    fn new(degree: usize) -> Self {
        Btree { root: None, degree }
    }
}
