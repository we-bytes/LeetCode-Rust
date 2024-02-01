# LeetCode-Rust
Solutions to LeetCode by Rust EveryDay

- https://github.com/aylei/leetcode-rust/tree/master/src/solution
- https://github.com/halfrost/LeetCode-Go


## 答题模板
```rust
pub struct Solution {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {}
}
```

- 返回变量: ans
- 变量长度: n
- 循环变量: i, j
- HashMap: map
- 字符串转数组: let seq: Vec<char> = s.chars().collect();