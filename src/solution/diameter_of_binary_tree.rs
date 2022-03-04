use super::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None {
            return 0;
        }
        let rb = root.as_ref().unwrap().borrow_mut();
        let l = Solution::height(rb.left.clone());
        let r = Solution::height(rb.right.clone());
        (l + r)
            .max(Solution::diameter_of_binary_tree(rb.left.clone()))
            .max(Solution::diameter_of_binary_tree(rb.right.clone()))
    }

    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root == None {
            return 0;
        }
        let rb = root.as_ref().unwrap().borrow_mut();
        match (rb.left.as_ref(), rb.right.as_ref()) {
            (Some(m), None) => 1 + Solution::height(Some(m.clone())),
            (None, Some(n)) => 1 + Solution::height(Some(n.clone())),
            (Some(m), Some(n)) => {
                (1 + Solution::height(Some(m.clone()))).max(1 + Solution::height(Some(n.clone())))
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_case() {
        use std::cell::RefCell;
        use std::rc::Rc;
        println!(
            "{:?}",
            super::Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(
                super::TreeNode::from_vec(vec![1, 2, 3, 4, 5])
            ))))
        );
    }
}
