/*
 * @lc app=leetcode id=198 lang=rust
 *
 * [198] House Robber
 *
 * https://leetcode.com/problems/house-robber/description/
 *
 * algorithms
 * Medium (47.81%)
 * Likes:    16909
 * Dislikes: 329
 * Total Accepted:    1.5M
 * Total Submissions: 3.1M
 * Testcase Example:  '[1,2,3,1]'
 *
 * You are a professional robber planning to rob houses along a street. Each
 * house has a certain amount of money stashed, the only constraint stopping
 * you from robbing each of them is that adjacent houses have security systems
 * connected and it will automatically contact the police if two adjacent
 * houses were broken into on the same night.
 *
 * Given an integer array nums representing the amount of money of each house,
 * return the maximum amount of money you can rob tonight without alerting the
 * police.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,7,9,3,1]
 * Output: 12
 * Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house
 * 5 (money = 1).
 * Total amount you can rob = 2 + 9 + 1 = 12.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 400
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];

        if nums.len() > 1 {
            dp[1] = i32::max(nums[0], nums[1]);
        }

        for i in 2..nums.len() {
            dp[i] = i32::max(dp[i - 2] + nums[i], dp[i - 1]);
        }

        dp[nums.len() - 1]
    }

    pub fn rob_o1_space_edition(nums: Vec<i32>) -> i32 {
        let mut dp = vec![nums[0], 0];
        dp[0] = nums[0];

        if nums.len() > 1 {
            dp[1] = i32::max(nums[0], nums[1]);
        }

        for i in 2..nums.len() {
            dp[i % 2] = i32::max(dp[(i - 2) % 2] + nums[i], dp[(i - 1) % 2]);
        }

        dp[(nums.len() - 1) % 2]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_o1_space_edition(vec![1, 2, 3, 1]), 4);
}
