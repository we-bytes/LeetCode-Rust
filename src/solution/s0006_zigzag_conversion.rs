use core::num;

pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut arr = vec![String::new(); num_rows];
        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
        println!("{:?}", iter);
        iter.zip(s.chars()).for_each(|(i, c)| arr[i].push(c));
        arr.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0006() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
        assert_eq!(Solution::convert("AY".to_string(), 2), "AY");
    }
}
