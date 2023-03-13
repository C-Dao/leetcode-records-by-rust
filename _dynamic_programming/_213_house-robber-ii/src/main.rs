/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 *
 * https://leetcode.com/problems/house-robber-ii/description/
 *
 * algorithms
 * Medium (40.23%)
 * Likes:    7814
 * Dislikes: 115
 * Total Accepted:    546.8K
 * Total Submissions: 1.3M
 * Testcase Example:  '[2,3,2]'
 *
 * You are a professional robber planning to rob houses along a street. Each
 * house has a certain amount of money stashed. All houses at this place are
 * arranged in a circle. That means the first house is the neighbor of the last
 * one. Meanwhile, adjacent houses have a security system connected, and it
 * will automatically contact the police if two adjacent houses were broken
 * into on the same night.
 *
 * Given an integer array nums representing the amount of money of each house,
 * return the maximum amount of money you can rob tonight without alerting the
 * police.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,3,2]
 * Output: 3
 * Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money
 * = 2), because they are adjacent houses.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3,1]
 * Output: 4
 * Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
 * Total amount you can rob = 1 + 3 = 4.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,2,3]
 * Output: 3
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 100
 * 0 <= nums[i] <= 1000
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        };

        let ans_1 = Self::dynamic_helper(&nums, 0, nums.len() - 2);
        let ans_2 = Self::dynamic_helper(&nums, 1, nums.len() - 1);

        i32::max(ans_1, ans_2)
    }

    fn dynamic_helper(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut dp = vec![0; end - start + 1];

        dp[0] = nums[start];

        if start < end {
            dp[1] = i32::max(nums[start], nums[start + 1]);
        }

        for i in start + 2..=end {
            dp[(i - start)] = i32::max(dp[((i - start) - 2)] + nums[i], dp[((i - start) - 1)])
        }

        dp[(end - start)]
    }

    pub fn rob_o1_space_edition(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        };

        let ans_1 = Self::dynamic_helper_o1_space_edition(&nums, 0, nums.len() - 2);
        let ans_2 = Self::dynamic_helper_o1_space_edition(&nums, 1, nums.len() - 1);

        i32::max(ans_1, ans_2)
    }

    fn dynamic_helper_o1_space_edition(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut dp = vec![0; 2];

        dp[0] = nums[start];

        if start < end {
            dp[1] = i32::max(nums[start], nums[start + 1]);
        }

        for i in start + 2..=end {
            dp[(i - start) % 2] = i32::max(
                dp[((i - start) - 2) % 2] + nums[i],
                dp[((i - start) - 1) % 2],
            )
        }

        dp[(end - start) % 2]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob_o1_space_edition(vec![1, 2, 3, 1]), 4);
}
