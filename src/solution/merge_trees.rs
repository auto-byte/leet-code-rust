use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (Some(x), None) => Some(x.clone()),
            (None, Some(x)) => Some(x.clone()),
            (Some(x), Some(y)) => match (x.borrow_mut(), y.borrow_mut()) {
                (mut xr, mut yr) => Some(Rc::new(RefCell::new(TreeNode {
                    val: xr.val + yr.val,
                    left: Self::merge_trees(xr.left.take(), yr.left.take()),
                    right: Self::merge_trees(xr.right.take(), yr.right.take()),
                }))),
            },
        }
    }
}
