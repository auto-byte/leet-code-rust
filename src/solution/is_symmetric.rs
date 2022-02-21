#![allow(dead_code)]
use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.as_ref().unwrap().borrow_mut();
        return Self::diff(node.left.clone(), node.right.clone());
    }

    fn diff(left: Node, right: Node) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }

        let left = left.as_ref().unwrap().borrow_mut();
        let right = right.as_ref().unwrap().borrow_mut();
        if left.val != right.val {
            return false;
        }

        return Self::diff(left.left.clone(), right.right.clone())
            && Self::diff(left.right.clone(), right.left.clone());
    }
}

async fn sa() -> () {

}

#[cfg(test)]
mod test {
    #[test]
    fn test_case() {
        let mut a = 2;
        let b = &mut a;
        *b = 12;
        assert_eq!(*b, 2); 
    }
}
