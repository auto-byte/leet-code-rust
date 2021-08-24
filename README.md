# leet-code-rust
LeetCode in Rust

## 15. 三数之和
```Rust
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut store = vec![];
        let len = nums.len();

        if len < 3 {
            return store;
        }

        let mut s_nums = nums.clone();
        s_nums.sort();

        for i in 0..len {
            if s_nums[i] > 0 {
                return store;
            }
            if i > 0 && s_nums[i] == s_nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = len - 1;

            while j < k {
                let sum = s_nums[i] + s_nums[j] + s_nums[k];
                if sum == 0 {
                    store.push(vec![s_nums[i], s_nums[j], s_nums[k]]);
                    while j < k && s_nums[j] == s_nums[j + 1] {
                        j += 1;
                    }
                    while j < k && s_nums[k] == s_nums[k - 1] {
                        k -= 1;
                    }
                    j += 1;
                    k -= 1;
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        return store;
    }
}
```


## 20. 有效的括号
```Rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = std::collections::LinkedList::<char>::new();
        let chars: Vec<char> = s.chars().collect();
        for n in chars.iter() {
            if *n == '(' || *n == '[' || *n == '{' {
                stack.push_back(*n);
            } else {
                if let Some(v) = stack.pop_back() {
                    match *n {
                        ']' => if v != '[' { return false; },
                        '}' => if v != '{' { return false; },
                        ')' => if v != '(' { return false; },
                        _ => {}
                    }
                } else {

                    return false;
                }
            }
        }
        return stack.is_empty();
    }
}
```

## 53. 最大子序和
```Rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut max = nums[0];
        for item in nums.iter() {
            pre = Self::max(pre + item, *item);
            max = Self::max(pre, max);
        }
        return max;
    }

    pub fn max(a: i32, b: i32) -> i32 {
        return if a >= b { a } else { b };
    }
}
```

## 416. 分割等和子集
```Rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum & 1 == 1 {
            return false;
        }

        let target = sum >> 2;
        let col_len = (target + 1) as usize;
        let mut dp: Vec<Vec<bool>> = vec![vec![false; col_len]; nums.len()];

        if nums[0] <= target {
            dp[0][nums[0] as usize] = true;
        }

        for i in 1..nums.len() {
            let num = nums[i] as usize;
            for j in 0..col_len {
                dp[i][j] = dp[i - 1][j];
                if num == j {
                    dp[i][j] = true;
                    continue;
                }
                if num < j {
                    dp[i][j] = dp[i - 1][j] | dp[i - 1][j - num];
                }
            }
        }

        return dp[nums.len() as usize - 1][target as usize];
    }
}
```
