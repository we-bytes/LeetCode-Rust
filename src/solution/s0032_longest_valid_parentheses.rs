pub struct Solution {}

impl Solution {
    // 动态规划
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let seq: Vec<char> = s.chars().collect();
        let n = seq.len();
        let mut dp: Vec<usize> = vec![0; n]; // vector<int> dp(n, 0);
        for i in 1..n {
            // for (int i = 1; i < n; i++)
            if seq[i] == ')' {
                if seq[i - 1] == '(' {
                    if i >= 2 {
                        dp[i] = dp[i - 2] + 2;
                    } else {
                        dp[i] = 2
                    }
                } else if i - dp[i - 1] > 0 && seq[i - dp[i - 1] - 1] == '(' {
                    if i - dp[i - 1] >= 2 {
                        dp[i] = dp[i - 1] + dp[i - dp[i - 1] - 2] + 2;
                    } else {
                        dp[i] = dp[i - 1] + 2
                    }
                }
                ans = ans.max(dp[i]);
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0032() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses("(((((()()".to_string()),
            4
        );
        assert_eq!(
            Solution::longest_valid_parentheses("((((((((()))".to_string()),
            6
        );
        assert_eq!(Solution::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(")()(((())))(".to_string()),
            10
        );
        assert_eq!(
            Solution::longest_valid_parentheses("(()(((()".to_string()),
            2
        );
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
    }
}
