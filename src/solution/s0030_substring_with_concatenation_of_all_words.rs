use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if words.is_empty() {
            return ans;
        }

        let (n, m, w) = (s.len(), words.len(), words[0].len());
        if w > n {
            return ans;
        }
        let mut total = HashMap::new();
        for word in words {
            let count = total.entry(word).or_insert(0);
            *count += 1;
        }

        // for (int i = 0; i < w; i++)
        for i in 0..w {
            let mut window: HashMap<String, i32> = HashMap::new();
            let mut cnt = 0; // 有效单词个数
                             // for (int j = i; j + w <= n; j += w)
            for j in (i..=n - w).step_by(w) {
                // 去除头部单词
                if j - i >= m * w {
                    let word = &s[j - m * w..(j - m * w) + w];
                    window.entry(word.to_string()).and_modify(|v| *v -= 1);
                    if total.contains_key(word) && window[word] < total[word] {
                        cnt -= 1;
                    }
                }
                // 新增单词
                let word = &s[j..j + w];
                window
                    .entry(word.to_string())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                if total.contains_key(word) && window[word] <= total[word] {
                    cnt += 1;
                }
                if cnt == m {
                    ans.push((j - (m - 1) * w) as i32)
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0030() {
        assert_eq!(
            Solution::find_substring("mississippi".to_string(), vec!["mississippis".to_string()]),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                "xxwordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![10]
        );
    }
}
