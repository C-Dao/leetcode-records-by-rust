/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

struct Solution {}

// @lc code=start
use std::cmp;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut start, mut sum, mut ans) = (0, 0, i32::MAX);

        for end in 0..nums.len() {
            sum += nums[end];
            while sum >= target {
                ans = cmp::min(ans, (end - start + 1) as i32);
                sum -= nums[start];
                start += 1;
            }
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
}
