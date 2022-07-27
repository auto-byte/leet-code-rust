use super::Solution;

impl Solution {
    // 测试未通过，执行超时
    fn appeal_sum(s: String) -> i64 {
        let len = s.len();
        let mut pos: Vec<i64> = vec![-1; 26];
        let chars = s.chars().collect::<Vec<_>>();
        let mut sum: i64 = 0;

        for i in 0..len {
            let a = (chars[i] as i64) - ('a' as i64);
            sum += ((i as i64) - pos[a as usize]) * ((len - i) as i64);
            pos[a as usize] = i as i64;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {
        assert_eq!(super::Solution::appeal_sum(String::from("abbca")), 28);
        assert_eq!(super::Solution::appeal_sum(String::from("code")), 20);
    }
}
