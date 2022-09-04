/*
 * @lc app=leetcode id=153 lang=rust
 *
 * [153] Find Minimum in Rotated Sorted Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let middle = (right - left) / 2 + left;
            if nums[middle] < nums[right] {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        return nums[left];
    }

    pub fn find_min_another_edition(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        for i in 0..nums.len() {
            if nums[i] < min {
                min = nums[i];
            }
        }
        min
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_min(vec![3, 4, 5, 1, 2]),
        Solution::find_min_another_edition(vec![3, 4, 5, 1, 2])
    );
}
