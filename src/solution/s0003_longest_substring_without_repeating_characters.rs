pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let len = seq.len();
        let (mut left, mut right, mut ans) = (0, 0, 0);

        for (right, &c) in seq.iter().enumerate() {
            for idx in left..right {
                if c == seq[idx] {
                    left = idx + 1;
                    break;
                }
            }
            ans = ans.max(right - left + 1); // 更新窗口长度最大值
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
