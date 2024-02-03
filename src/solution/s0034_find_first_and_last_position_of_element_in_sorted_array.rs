pub struct Solution {}

impl Solution {
    pub fn search(nums: &Vec<i32>, target: i32, flag: bool) -> usize {
        let mut ans = nums.len();
        let (mut l, mut r) = (0, ans - 1);
        while l <= r {
            let mid = (l + (r - l) / 2);
            if (flag && nums[mid] >= target) || nums[mid] > target {
                if mid > 0 {
                    r = mid - 1;
                    ans = mid;
                } else {
                    ans = mid;
                    break;
                }
            } else {
                l = mid + 1;
            }
        }
        ans
    }
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let mut l = Self::search(&nums, target, true);
        let mut r = Self::search(&nums, target, false);
        if r > 0 {
            r -= 1;
        }
        if (l <= r && r < nums.len() && nums[l] == target && nums[r] == target) {
            return [l as i32, r as i32].to_vec();
        } else {
            return [-1, -1].to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0034() {
        // assert_eq!(
        //     Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        //     vec![3, 4]
        // );
        // assert_eq!(
        //     Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        //     vec![-1, -1]
        // );
        // assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
    }
}
