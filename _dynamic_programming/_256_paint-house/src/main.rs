/*
 * @lc app=leetcode id=256 lang=rust
 *
 * [256] Paint House
 *
 * https://leetcode.com/problems/paint-house/description/
 *
 * algorithms
 * Medium (40.23%)
 * Likes:    7814
 * Dislikes: 115
 * Total Accepted:    546.8K
 * Total Submissions: 1.3M
 * Testcase Example:  '[[17,2,17],[16,16,5],[14,3,19]]'
 *
 * There are a row of n houses, each house can be painted with one of the three colors: red, blue or green.
 * The cost of painting each house with a certain color is different. You have to paint all the houses such
 * that no two adjacent houses have the same color.
 * The cost of painting each house with a certain color is represented by a n x 3 cost matrix.
 * For example, costs[0][0] is the cost of painting house 0 with color red; costs[1][2] is the cost of painting
 * house 1 with color green, and so on… Find the minimum cost to paint all houses.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [[17,2,17],[16,16,5],[14,3,19]]
 * Output: 10
 * Explanation:  Paint house 0 into blue, paint house 1 into green, paint house 2 into blue. Minimum cost: 2 + 5 + 3 = 10.
 *
 *
 *
 *
 * Constraints:
 * costs.length == n
 * costs[i].length == 3
 * 1 <= n <= 100
 * 1 <= costs[i][j] <= 20
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; costs.len()]; 3];

        for i in 0..3 {
            dp[i][0] = costs[0][i];
        }

        for i in 1..costs.len() {
            for j in 0..3 {
                dp[j][i] = i32::min(dp[(j + 2) % 3][i - 1], dp[(j + 1) % 3][i - 1]) + costs[i][j];
            }
        }

        i32::min(
            dp[0][costs.len() - 1],
            i32::min(dp[1][costs.len() - 1], dp[2][costs.len() - 1]),
        )
    }

    pub fn min_cost_o1_space_edition(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; 2]; 3];

        for i in 0..3 {
            dp[i][0] = costs[0][i];
        }

        for i in 1..costs.len() {
            for j in 0..3 {
                dp[j][i % 2] = i32::min(dp[(j + 2) % 3][(i - 1) % 2], dp[(j + 1) % 3][(i - 1) % 2])
                    + costs[i][j];
            }
        }

        let last = (costs.len() - 1) % 2;
        i32::min(dp[0][last], i32::min(dp[1][last], dp[2][last]))
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]),
        10
    );
    assert_eq!(
        Solution::min_cost_o1_space_edition(vec![
            vec![17, 2, 17],
            vec![16, 16, 5],
            vec![14, 3, 19]
        ]),
        10
    );
}
