#![allow(dead_code)]
use super::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut count = 2;

        for i in 2..nums.len() {
            if nums[i] != nums[count - 2] {
                nums[count] = nums[i];
                count += 1;
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_case() {
        println!(
            "{:?}",
            super::Solution::remove_duplicates(&mut vec![1, 2, 2, 2, 4, 5])
        );

        println!(
            "{:?}",
            super::Solution::remove_duplicates(&mut vec![1])
        );
    }
}
