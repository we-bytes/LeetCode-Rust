pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut m = vec![vec![String::new()]];
        for i in 1..=n as usize {
            let mut v = vec![];
            for j in 0..i {
                for p in m[j].iter() {
                    for q in m[i - 1 - j].iter() {
                        v.push(format!("{}({})", p, q));
                    }
                }
            }
            m.push(v);
        }
        m.into_iter().last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0022() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "()(())", "(())()", "()()()"]
        );
    }
}
