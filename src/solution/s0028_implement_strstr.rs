pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // haystack.find(&needle).map_or(-1_i32, |v| v as i32)
        // haystack.find(&needle).unwrap_or(usize::MAX) as i32

        let h = haystack.len();
        let n = needle.len();
        let hb = haystack.as_bytes();
        let nb = needle.as_bytes();

        if h < 1 {
            return 0;
        }
        if h < n {
            return -1;
        }
        for i in (0..=h - n) {
            let mut flag = true;
            for j in (0..n) {
                if hb[i + j] != nb[j] {
                    flag = false;
                    break;
                }
            }

            if flag {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0028() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2)
    }
}
