pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut best = i32::MAX;

        for i in 0..n - 2 {
            // 跳过相同的数字
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            // 双指针
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                // 如果和为 target 直接返回答案
                if sum == target {
                    return sum;
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
                // 根据差值的绝对值来更新答案
                if (sum - target).abs() < (best - target).abs() {
                    best = sum;
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0016() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3], 1), 6);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}
