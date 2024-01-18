pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let (mut ret, n) = (vec![], nums.len());
        if n < 4 {
            return ret;
        }
        // 转为i64
        let (mut nums, target) = (
            nums.into_iter().map(|num| num as i64).collect::<Vec<_>>(),
            target as i64,
        );
        nums.sort();
        for i in 0..n - 3 {
            if nums[i] + nums[i + 1] + nums[i + 2] + nums[i + 3] > target {
                break; // 剪枝
            }
            if nums[i] + nums[n - 1] + nums[n - 2] + nums[n - 3] < target {
                continue; // 去重
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // 去重
            }
            for j in i + 1..n - 2 {
                if nums[i] + nums[j] + nums[j + 1] + nums[j + 2] > target {
                    break; // 剪枝
                }
                if nums[i] + nums[j] + nums[n - 1] + nums[n - 2] < target {
                    continue; // 去重
                }
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue; // 去重
                }
                let (mut l, mut r) = (j + 1, n - 1);
                while l < r {
                    let sum = nums[i] + nums[j] + nums[l] + nums[r];
                    if sum == target {
                        ret.push(vec![
                            nums[i] as i32,
                            nums[j] as i32,
                            nums[l] as i32,
                            nums[r] as i32,
                        ]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1; // 去重
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1; // 去重
                        }
                        l += 1;
                        r -= 1;
                    } else if sum < target {
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1; // 去重
                        }
                        l += 1;
                    } else {
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1; // 去重
                        }
                        r -= 1;
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0018() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
        assert_eq!(
            Solution::four_sum(
                vec![0, 0, 0, -1000000000, -1000000000, -1000000000, -1000000000],
                -1000000000
            ),
            vec![vec![-1000000000, 0, 0, 0]]
        );
    }
}
