use super::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let mut lo = 0i64;
        let mut hi = x + 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let pow = mid * mid;
            if pow < x {
                lo = mid + 1;
            } else if pow == x {
                return mid as i32;
            } else {
                hi = mid;
            }
        }
        lo as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {
        assert_eq!(super::Solution::my_sqrt(4), 2);
    }
}
