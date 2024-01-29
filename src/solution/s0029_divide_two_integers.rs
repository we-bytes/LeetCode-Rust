pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let mut ans = 0 as i64; // 0即是商的最高+1位取值
        let mut dend = (dividend as i64).abs();
        let mut absd = (divisor as i64).abs();
        let mut dsor = absd;
        // 每次让除数翻倍, 直到除数比被除数大
        while dend >= dsor {
            dsor <<= 1;
        }
        // 每次除数减半，直到除数比原值小（前面while循环多少次，这里也是多少次）
        while dsor > absd {
            dsor >>= 1;
            ans <<= 1;
            if dend >= dsor {
                // 前面ans已经左移末位一定是0，末尾+1，代表该位值
                ans += 1;
                dend = dend - dsor;
            }
        }
        if (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0) {
            ans as i32 // 同号为正
        } else {
            -ans as i32 // 异号为负
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0029() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2)
    }
}
