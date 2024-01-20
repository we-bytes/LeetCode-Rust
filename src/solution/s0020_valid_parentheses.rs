pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        };
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '(' => stack.push(')'),
                '}' | ']' | ')' if Some(ch) != stack.pop() => return false,
                _ => (),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0020() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }
}
