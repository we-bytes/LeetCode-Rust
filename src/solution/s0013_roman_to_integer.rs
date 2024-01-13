use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut sum = 0;
        let mut last_value = 0;

        for c in s.chars().rev() {
            // 从后开始迭代
            let value = map[&c];
            if value < last_value {
                sum -= value;
            } else {
                sum += value;
            }
            last_value = value;
        }
        sum
    }

    pub fn roman_to_int_2(s: String) -> i32 {
        let map = ['I', 'V', 'X', 'L', 'C', 'D', 'M']
            .iter()
            .zip([1, 5, 10, 50, 100, 500, 1000])
            .collect::<HashMap<_, _>>();
        s.chars().rev().fold(0, |acc, c| {
            acc + if acc > 4 * map[&c] { -map[&c] } else { map[&c] }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::roman_to_int_2("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int_2("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int_2("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int_2("MCMXCIV".to_string()), 1994);
        assert_eq!(Solution::roman_to_int_2("DCXXI".to_string()), 621);
    }
}
