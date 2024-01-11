pub struct Solution {}

impl Solution {
    // 双指针
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right, mut ans) = (0, height.len() - 1, 0); // 左右指针
        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            ans = ans.max(area);
            if height[left] < height[right] {
                left += 1
            } else {
                right -= 1
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![6, 9]), 6);
        assert_eq!(Solution::max_area(vec![1, 1, 2, 1, 1, 1]), 5);
    }
}
