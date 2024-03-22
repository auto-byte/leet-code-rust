#![allow(dead_code)]
use super::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;
            let mut hours = 0;
            for pile in &piles {
                hours += (pile + mid - 1) / mid;
            }

            if hours > h {
                left = mid + 1;
            } else {
                right = mid;
            }
        }


        left
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_case() {
        println!("{:?}", super::Solution::min_eating_speed(vec![1, 2, 2], 3));
    }
}
