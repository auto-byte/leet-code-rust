#![allow(dead_code)]
mod is_palindrome;
mod is_symmetric;
mod merge_trees;
mod find_disappeared_numbers;
mod diameter_of_binary_tree;

use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    fn from_vec(arr: Vec<i32>) -> Self {
        return Self {
            val: arr[0],
            left: None,
            right: None,
        };
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn set_val(&mut self, val: i32) {
        self.val = val;
    }

    fn set_next(&mut self, val: i32) {
        self.next = Some(Box::new(ListNode::new(val)));
    }

    fn get_next(&mut self) -> &mut Option<Box<ListNode>> {
        &mut self.next
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(arr: Vec<i32>) -> Self {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut head;

        for i in 0..arr.len() - 1 {
            let va = curr.as_mut().unwrap();
            va.set_val(arr[i]);
            va.set_next(arr[i + 1]);
            curr = va.get_next();
        }

        return *head.unwrap();
    }
}

impl Into<Vec<i32>> for ListNode {
    fn into(self) -> Vec<i32> {
        let mut arr = Vec::<i32>::new();
        let mut node = &self;

        arr.push(self.val);

        loop {
            match node.next {
                Some(ref x) => {
                    arr.push(x.val);
                    node = x.as_ref();
                }
                None => break,
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_list_node() {
        let node = super::ListNode::from(vec![1, 2, 3]);
        println!("{:?}", node);
        let arr: Vec<i32> = node.into();
        println!("{:?}", arr);
    }
}
