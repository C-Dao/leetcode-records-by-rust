/*
 * @lc app=leetcode id=1833 lang=rust
 *
 * [1833] Maximum Ice Cream Bars
 *
 * https://leetcode.com/problems/maximum-ice-cream-bars/description/
 *
 * algorithms
 * Medium (65.17%)
 * Likes:    1589
 * Dislikes: 574
 * Total Accepted:    109.1K
 * Total Submissions: 147.6K
 * Testcase Example:  '[1,3,2,4,1]\n7'
 *
 * It is a sweltering summer day, and a boy wants to buy some ice cream bars.
 *
 * At the store, there are n ice cream bars. You are given an array costs of
 * length n, where costs[i] is the price of the i^th ice cream bar in coins.
 * The boy initially has coins coins to spend, and he wants to buy as many ice
 * cream bars as possible.
 *
 * Return the maximum number of ice cream bars the boy can buy with coins
 * coins.
 *
 * Note: The boy can buy the ice cream bars in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: costs = [1,3,2,4,1], coins = 7
 * Output: 4
 * Explanation: The boy can buy ice cream bars at indices 0,1,2,4 for a total
 * price of 1 + 3 + 2 + 1 = 7.
 *
 *
 * Example 2:
 *
 *
 * Input: costs = [10,6,8,7,7,8], coins = 5
 * Output: 0
 * Explanation: The boy cannot afford any of the ice cream bars.
 *
 *
 * Example 3:
 *
 *
 * Input: costs = [1,6,3,1,2,5], coins = 20
 * Output: 6
 * Explanation: The boy can buy all the ice cream bars for a total price of 1 +
 * 6 + 3 + 1 + 2 + 5 = 18.
 *
 *
 *
 * Constraints:
 *
 *
 * costs.length == n
 * 1 <= n <= 10^5
 * 1 <= costs[i] <= 10^5
 * 1 <= coins <= 10^8
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut count = 0;

        costs.sort_unstable();

        for cost in costs {
            coins -= cost;

            if coins >= 0 {
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::max_ice_cream(vec![1, 3, 2, 4, 1], 7), 4);
}
