pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut negative = false;
        let mut res: i64 = 0;
        for (i, ch) in s.trim().chars().enumerate() {
            // 正号
            if i == 0 && ch == '+' {
                continue;
            }
            // 负号
            if i == 0 && ch == '-' {
                negative = true;
                continue;
            }
            // 其他字符
            if !ch.is_digit(10) {
                break;
            }
            res = 10 * res + ch.to_digit(10).unwrap() as i64;
            if negative {
                if -res < i32::min_value() as i64 {
                    return i32::min_value();
                }
            } else {
                if res > i32::max_value() as i64 {
                    return i32::max_value();
                }
            }
        }
        if negative {
            -res as i32
        } else {
            res as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_atoi("aa".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("004193333".to_string()), 4193333);
        assert_eq!(Solution::my_atoi("2147483646".to_string()), 2147483646);
        assert_eq!(Solution::my_atoi("2147483648".to_string()), 2147483647);
    }
}
