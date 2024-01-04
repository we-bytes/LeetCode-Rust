use std::cmp::min;
pub struct Solution {}

// 二分删除log(m+n)
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let min_left = (nums1.len() + nums2.len() + 1) / 2;
        let min_right = (nums1.len() + nums2.len() + 2) / 2;
        (findk(&nums1, 0, &nums2, 0, min_left) + findk(&nums1, 0, &nums2, 0, min_right)) as f64
            / 2.0
    }
}

fn findk(v1: &Vec<i32>, i: usize, v2: &Vec<i32>, j: usize, k: usize) -> i32 {
    // 当nums1 删除完，则直接返回j+k-1位置的数字，nums2删完同理
    if i >= v1.len() {
        return v2[j + k - 1];
    }
    if j >= v2.len() {
        return v1[i + k - 1];
    }
    // k==1时表示找最小的数字
    if k == 1 {
        return min(v1[i], v2[j]);
    }
    let max1 = if (i + k / 2 - 1) < v1.len() {
        v1[i + k / 2 - 1]
    } else {
        i32::MAX
    };
    let max2 = if (j + k / 2 - 1) < v2.len() {
        v2[j + k / 2 - 1]
    } else {
        i32::MAX
    };
    return if max1 > max2 {
        findk(v1, i, v2, j + k / 2, k - k / 2)
    } else {
        findk(v1, i + k / 2, v2, j, k - k / 2)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
