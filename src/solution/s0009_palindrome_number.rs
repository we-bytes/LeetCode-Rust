pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>() // Rust反转字符串
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0009() {
        assert_eq!(Solution::is_palindrome(-32), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome(9), true);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(2222), true);
        assert_eq!(Solution::is_palindrome(11222211), true);
    }
}
