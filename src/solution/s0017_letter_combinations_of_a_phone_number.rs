pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // 用一个哈希表存储数字和字母的映射关系
        let map: Vec<&str> = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        // 初始的结果为空数组
        let mut res: Vec<String> = Vec::new();
        // 如果输入的字符串为空，则直接返回空数组
        if digits.is_empty() {
            return res;
        }
        // 递归生成所有可能的字母组合
        Self::backtrack(&map, &digits, 0, String::new(), &mut res);
        res
    }

    fn backtrack(map: &[&str], digits: &str, index: usize, path: String, res: &mut Vec<String>) {
        // 如果已经到达字符串末尾，说明已经生成了一种可能的组合，将其添加到结果数组中
        if index == digits.len() {
            res.push(path);
            return;
        }
        // 取出当前数字对应的所有可能的字母
        let letters = map[digits[index..=index].parse::<usize>().unwrap()];
        // 遍历每一个可能的字母
        for c in letters.chars() {
            // 生成新的组合，加入当前字母
            let mut new_path = path.clone();
            new_path.push(c);
            // 继续递归生成下一个数字的字母组合
            Self::backtrack(map, digits, index + 1, new_path, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0017() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            ["a", "b", "c"]
        );
    }
}
