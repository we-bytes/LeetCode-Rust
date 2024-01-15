pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let map = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut num = num;
        let mut ans = String::new();
        let mut iter = map.into_iter();

        while let Some((k, v)) = iter.next() {
            // 如果输入的参数值小于当前项，跳过即可
            if num >= v {
                // NOTE: Rust 的整数除法会截断小数部分
                ans.push_str(&k.repeat((num / v) as usize));
                num %= v; // 取余数
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0012() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
