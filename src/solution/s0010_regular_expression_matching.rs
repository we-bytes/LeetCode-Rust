pub struct Solution {}

impl Solution {
    // 动态规划
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 2..=p.len() {
            dp[0][i] = p[i - 1] == b'*' && dp[0][i - 2];
        }
        (1..=s.len()).for_each(|i| {
            (1..=p.len()).for_each(|j| {
                dp[i][j] = match p[j - 1] {
                    b'*' => {
                        dp[i][j - 2] || dp[i - 1][j] && (p[j - 2] == b'.' || p[j - 2] == s[i - 1])
                    }
                    b'.' => dp[i - 1][j - 1],
                    other => dp[i - 1][j - 1] && s[i - 1] == other,
                }
            })
        });
        dp[s.len()][p.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0010() {
        assert_eq!(
            Solution::is_match("aa".to_string(), "a".to_string(),),
            false
        );
        assert_eq!(
            Solution::is_match("aa".to_string(), "a*".to_string(),),
            true
        );
        assert_eq!(
            Solution::is_match("ab".to_string(), ".*".to_string(),),
            true
        );
    }
}
