/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 *
 * https://leetcode.com/problems/minimum-size-subarray-sum/description/
 *
 * algorithms
 * Medium (43.92%)
 * Likes:    7609
 * Dislikes: 215
 * Total Accepted:    589K
 * Total Submissions: 1.3M
 * Testcase Example:  '7\n[2,3,1,2,4,3]'
 *
 * Given an array of positive integers nums and a positive integer target,
 * return the minimal length of a contiguous subarray [numsl, numsl+1, ...,
 * numsr-1, numsr] of which the sum is greater than or equal to target. If
 * there is no such subarray, return 0 instead.
 *
 *
 * Example 1:
 *
 *
 * Input: target = 7, nums = [2,3,1,2,4,3]
 * Output: 2
 * Explanation: The subarray [4,3] has the minimal length under the problem
 * constraint.
 *
 *
 * Example 2:
 *
 *
 * Input: target = 4, nums = [1,4,4]
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: target = 11, nums = [1,1,1,1,1,1,1,1]
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= target <= 10^9
 * 1 <= nums.length <= 10^5
 * 1 <= nums[i] <= 10^4
 *
 *
 *
 * Follow up: If you have figured out the O(n) solution, try coding another
 * solution of which the time complexity is O(n log(n)).
 */

struct Solution {}

// @lc code=start

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut sp, mut ep, mut sum, mut ans) = (0, 0, 0, i32::MAX);

        while ep < nums.len() {
            sum += nums[ep];

            while sum >= target && sp <= ep {
                ans = i32::min(ans, (ep - sp + 1) as i32);
                sum -= nums[sp];
                sp += 1;
            }

            ep += 1;
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
