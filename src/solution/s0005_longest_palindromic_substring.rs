pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let seq: Vec<char> = s.chars().collect();
        let len = s.len();
        if len < 2 {
            return s;
        }
        let (mut idx, mut curr_len, mut curr_start, mut curr_end) = (0, 0, 0, 0);
        while idx < len {
            let (mut i, mut j) = (idx, idx);
            let ch = seq[idx];
            // handle same char, example "aaa" "bbbb"
            while i > 0 && seq[i - 1] == ch {
                i -= 1;
            }
            while j < len - 1 && seq[j + 1] == ch {
                j += 1;
            }
            idx = j + 1; // 记录中心点
                         // 从中心扩散
            while i > 0 && j < len - 1 && seq[i - 1] == seq[j + 1] {
                i -= 1;
                j += 1;
            }
            // 记录最大回文数
            let max_len = j - i + 1;
            if max_len > curr_len {
                curr_len = max_len;
                curr_start = i;
                curr_end = j;
            }
            if max_len >= len - 1 {
                break;
            }
        }
        s[curr_start..curr_end + 1].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
    }
}
