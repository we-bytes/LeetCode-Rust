pub struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 寻找非降序的 nums[i-1]
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        // 寻找 nums[j] > nums[i-1]
        if i != 0 {
            let mut j = nums.len() - 1;
            while j > 0 && nums[i - 1] >= nums[j] {
                j -= 1;
            }
            // 交互 nums[i-1] 和 nums[j]
            nums.swap(i - 1, j); // C++交换数组中两个元素的值 swap(nums[i], nums[j])
        }
        // 反转 nums[i-1] 之后的元素
        nums[i..].reverse() // C++指定位置反转数组中一部分 reverse(nums.begin() + i + 1, nums.end())
    }
    pub fn next_permutation2(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 {
            if nums[i - 1] < nums[i] {
                let mut j = i;
                while j < nums.len() && nums[i - 1] < nums[j] {
                    j += 1;
                }
                nums.swap(i - 1, j - 1);
                break;
            }
            i -= 1;
        }
        nums[i..].sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0031() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);
    }
}
