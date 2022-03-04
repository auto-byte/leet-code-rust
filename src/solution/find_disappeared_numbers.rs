#![allow(dead_code)]
use super::Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut arr: Vec<i32> = vec![0; nums.len()];
        for i in 0..nums.len() {
            arr[i] = (i as i32) + 1;
        }

        for i in 0..nums.len() {
            arr[nums[i] as usize - 1] = 0;
        }

        let mut res = vec![];
        for i in arr.iter() {
            if *i > 0 {
                res.push(*i);
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_case() {
        println!(
            "{:?}",
            super::Solution::find_disappeared_numbers(vec![1, 2, 2])
        );
    }
}
