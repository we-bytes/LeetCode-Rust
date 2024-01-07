pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans = 0;

        while x != 0 {
            if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
                return 0;
            }
            ans = ans * 10 + x % 10;
            x /= 10;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-base.pow(31)) as i32), 0);
    }
}
