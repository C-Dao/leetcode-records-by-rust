/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 *
 * https://leetcode.com/problems/coin-change/description/
 *
 * algorithms
 * Medium (40.90%)
 * Likes:    15373
 * Dislikes: 356
 * Total Accepted:    1.3M
 * Total Submissions: 3.2M
 * Testcase Example:  '[1,2,5]\n11'
 *
 * You are given an integer array coins representing coins of different
 * denominations and an integer amount representing a total amount of money.
 *
 * Return the fewest number of coins that you need to make up that amount. If
 * that amount of money cannot be made up by any combination of the coins,
 * return -1.
 *
 * You may assume that you have an infinite number of each kind of coin.
 *
 *
 * Example 1:
 *
 *
 * Input: coins = [1,2,5], amount = 11
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 *
 *
 * Example 2:
 *
 *
 * Input: coins = [2], amount = 3
 * Output: -1
 *
 *
 * Example 3:
 *
 *
 * Input: coins = [1], amount = 0
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= coins.length <= 12
 * 1 <= coins[i] <= 2^31 - 1
 * 0 <= amount <= 10^4
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let target = amount as usize;
        let mut dp = vec![amount + 1; target + 1];

        dp[0] = 0;

        for i in 1..=target {
            for j in 0..coins.len() {
                if coins[j] as usize <= i {
                    dp[i] = i32::min(dp[i], dp[i - coins[j] as usize] + 1);
                }
            }
        }

        if dp[target] > amount {
            -1
        } else {
            dp[target]
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
}
