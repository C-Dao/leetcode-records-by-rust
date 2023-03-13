/*
 * @lc app=leetcode id=746 lang=rust
 *
 * [746] Min Cost Climbing Stairs
 *
 * https://leetcode.com/problems/min-cost-climbing-stairs/description/
 *
 * algorithms
 * Easy (61.16%)
 * Likes:    8862
 * Dislikes: 1383
 * Total Accepted:    758.2K
 * Total Submissions: 1.2M
 * Testcase Example:  '[10,15,20]'
 *
 * You are given an integer array cost where cost[i] is the cost of i^th step
 * on a staircase. Once you pay the cost, you can either climb one or two
 * steps.
 *
 * You can either start from the step with index 0, or the step with index 1.
 *
 * Return the minimum cost to reach the top of the floor.
 *
 *
 * Example 1:
 *
 *
 * Input: cost = [10,15,20]
 * Output: 15
 * Explanation: You will start at index 1.
 * - Pay 15 and climb two steps to reach the top.
 * The total cost is 15.
 *
 *
 * Example 2:
 *
 *
 * Input: cost = [1,100,1,1,1,100,1,1,100,1]
 * Output: 6
 * Explanation: You will start at index 0.
 * - Pay 1 and climb two steps to reach index 2.
 * - Pay 1 and climb two steps to reach index 4.
 * - Pay 1 and climb two steps to reach index 6.
 * - Pay 1 and climb one step to reach index 7.
 * - Pay 1 and climb two steps to reach index 9.
 * - Pay 1 and climb one step to reach the top.
 * The total cost is 6.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= cost.length <= 1000
 * 0 <= cost[i] <= 999
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![cost[0], cost[1]];

        dp.resize(cost.len(), 0);

        for i in 2..cost.len() {
            dp[i] = i32::min(dp[i - 2], dp[i - 1]) + cost[i];
        }

        i32::min(dp[cost.len() - 1], dp[cost.len() - 2])
    }

    pub fn min_cost_climbing_stairs_o1_space_edtion(cost: Vec<i32>) -> i32 {
        let mut dp = vec![cost[0], cost[1]];

        for i in 2..cost.len() {
            dp[i%2] = i32::min(dp[1], dp[0]) + cost[i];
        }

        i32::min(dp[1], dp[0])
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(Solution::min_cost_climbing_stairs_o1_space_edtion(vec![10, 15, 20]), 15);
}
