use super::Solution;

impl Solution {
    // 测试未通过，执行超时
    fn appeal_sum(s: String) -> i64 {
        let len = s.len();
        if len <= 1 {
            return s.len() as i64;
        }

        let mut count: Vec<i64> = vec![1; len];
        count[len - 1] = len as i64;
        let mut alp: Vec<Vec<char>> = s.chars().map(|f| vec![f]).collect();
        let c = s.chars().collect::<Vec<_>>();
        for v in 1..len {
            let mut start: usize = 0;
            let offset = v;

            while start < len - offset {
                let end = start + offset;
                if !alp[start].contains(&c[end]) {
                    alp[start].push(c[end]);
                    count[start] = count[start] + 1;
                }
                start += 1;
            }

            let last = len - offset - 1;
            for i in 0..last {
                count[last] += count[i];
            }
        }

        count.iter().sum()
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
