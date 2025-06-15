use crate::btree::node::BtreeNode;

#[derive(Clone)]
pub(crate) struct Btree {
    root: Option<BtreeNode>,
    degree: usize,
}

impl Btree {
    fn new(degree: usize) -> Self {
        Btree { root: None, degree }
    }
}
