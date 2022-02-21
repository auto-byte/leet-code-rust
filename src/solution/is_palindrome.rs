use super::{ListNode, Solution};
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let arr: Vec<i32> = (*head.unwrap()).into();
        let len = arr.len();
        for i in 0..len / 2 {
            if arr[i] != arr[len - 1 - i] {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {
        println!("[1] - {}", case(vec![1]));
        println!("[1, 2] - {}", case(vec![1, 2]));
        println!("[1, 2, 2] - {}", case(vec![1, 2, 2]));
        println!("[1, 2, 2, 1] - {}", case(vec![1, 2, 2, 1]));
    }

    fn case(vec: Vec<i32>) -> bool {
        super::Solution::is_palindrome(Some(Box::new(super::ListNode::from(vec))))
    }
}
