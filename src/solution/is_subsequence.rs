use super::Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s == t {
            return true;
        }

        let (mut ps, mut pt) = (0, 0);
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();

        while ps < s.len() && pt < t.len() {
            ps += if s[ps] == t[pt] { 1 } else { 0 };
            pt += 1;
        }

        ps == s.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {
        println!(
            "abc in acbdsc: {}",
            super::Solution::is_subsequence(String::from("abc"), String::from("acbdsc"))
        );
        println!(
            "abc in acbds: {}",
            super::Solution::is_subsequence(String::from("abc"), String::from("acbds"))
        );
        println!(
            "abc in ac: {}",
            super::Solution::is_subsequence(String::from("abc"), String::from("ac"))
        );
        println!(
            "abc in abc: {}",
            super::Solution::is_subsequence(String::from("abc"), String::from("abc"))
        );
    }
}
