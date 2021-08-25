# leet-code-rust
LeetCode in Rust

## 1. 两数之和
```Rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }

        let mut cache = std::collections::HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            let sub = target - *x;
            if let Some(v) = cache.get(&sub) {
                return vec![*v, i as i32];
            }
            cache.insert(*x, i as i32);
        }

        return vec![];
    }
}
```

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

## 18. 四数之和
```Rust
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
       let mut res = vec![];
        let len = nums.len();
        let mut s_nums = nums.clone();
        s_nums.sort();

        if len < 4 {
            return res;
        }

        for i in 0..(len - 3) {
            if i > 0 && s_nums[i] == s_nums[i - 1] {
                continue;
            }

            if target >= 0 && (target - s_nums[i] < 0) {
                return res;
            }

            for j in (i + 1)..(len - 2) {
                if j > (i + 1) && s_nums[j] == s_nums[j - 1] {
                    continue;
                }
                if target >= 0 && (target - (s_nums[i] + s_nums[j]) < 0) {
                    break;
                }
                let mut k = j + 1;
                let mut l = len - 1;
                while k < l {
                    let sum = target - (s_nums[i] + s_nums[j] + s_nums[k] + s_nums[l]);
                    if sum == 0 {
                        res.push(vec![s_nums[i], s_nums[j], s_nums[k], s_nums[l]]);
                        while k < l && s_nums[k] == s_nums[k + 1] {
                            k += 1;
                        }
                        while k < l && s_nums[l] == s_nums[l - 1] {
                            l -= 1;
                        }
                        k += 1;
                        l -= 1;
                    } else if sum > 0 {
                        k += 1;
                    } else {
                        l -= 1;
                    }
                }
            }
        }

        return res;
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
## 70. 爬楼梯
```Rust
impl Solution {
   pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = std::collections::HashMap::new();
        cache.insert(1, 1);
        cache.insert(2, 2);
        return Self::cache_climb_stairs(n, &mut cache);
    }

    pub fn cache_climb_stairs(n: i32, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if n == 1 || n == 2 {
            return *cache.get(&n).unwrap();
        }
        return Self::get_or_put(n - 1, cache) + Self::get_or_put(n - 2, cache);
    }

    pub fn get_or_put(n: i32, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if let Some(x) = cache.get(&n) {
            return *x;
        } else {
            let res =
                Self::cache_climb_stairs(n - 1, cache) + Self::cache_climb_stairs(n - 2, cache);
            cache.insert(n, res);
            return res;
        }
    }
}
```

## 167. 两数之和 II - 输入有序数组
```Rust
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        if numbers.len() < 2 {
            return vec![];
        }

        let mut cache = std::collections::HashMap::new();
        for (i, x) in numbers.iter().enumerate() {
            let sub = target - *x;
            if let Some(v) = cache.get(&sub) {
                return vec![*v + 1, i as i32 + 1];
            }
            cache.insert(*x, i as i32);
        }

        return vec![];
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

## 509. 斐波那契数
```Rust
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut cache = std::collections::HashMap::new();
        return cacheFib(n, &mut cache);
    }
}

pub fn cacheFib(n: i32, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if n == 1 || n == 0 {
            return n;
        }
        let mut n_one: i32;
        if let Some(x) = cache.get(&(n - 1)) {
            n_one = *x;
        } else {
            n_one = cacheFib(n - 1, cache);
            cache.insert(n - 1, n_one);
        }
        let mut n_two: i32;
        if let Some(x) = cache.get(&(n - 2)) {
            n_two = *x;
        } else {
            n_two = cacheFib(n - 2, cache);
            cache.insert(n - 2, n_two);
        }

        return n_one + n_two;
}
```
    