use super::Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut num: i32 = column_number;
        let mut vec: Vec<char> = vec![];
        while num > 0 {
            num -= 1;
            vec.push(((num % 26 + 65) as u8) as char);
            num /= 26;
        }
        vec.iter().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case() {
        // assert_eq!(super::Solution::convert_to_title(1), "A");
        // assert_eq!(super::Solution::convert_to_title(2), "B");
        // assert_eq!(super::Solution::convert_to_title(27), "AA");
        assert_eq!(super::Solution::convert_to_title(28), "AB");
    }
}
