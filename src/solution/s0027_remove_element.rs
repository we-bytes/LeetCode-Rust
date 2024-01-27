pub struct Solution {}

impl Solution {
    // 双指针优化
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let (mut left, mut right) = (0_usize, nums.len());
        while left < right {
            if nums[left] == val {
                nums[left] = nums[right - 1];
                right -= 1;
            } else {
                left += 1;
            }
        }
        println!("{:?}", nums);
        left as i32
    }
    // retain函数
    pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0027() {
        let mut vec1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut vec1, 2), 5);
        assert_eq!(vec1[0..5], [0, 1, 4, 0, 3]);
        assert_eq!(Solution::remove_element(&mut vec![], 2), 0);
        assert_eq!(
            Solution::remove_element(&mut vec![1, 2, 2, 2, 2, 2, 2], 2),
            1
        );
        assert_eq!(
            Solution::remove_element(&mut vec![2, 2, 2, 2, 2, 2, 2], 2),
            0
        );
        assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
        assert_eq!(Solution::remove_element(&mut vec![2], 3), 1);
    }
}
