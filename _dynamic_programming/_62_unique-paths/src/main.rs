/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 *
 * https://leetcode.com/problems/unique-paths/description/
 *
 * algorithms
 * Medium (60.67%)
 * Likes:    13053
 * Dislikes: 374
 * Total Accepted:    1.3M
 * Total Submissions: 2.1M
 * Testcase Example:  '3\n7'
 *
 * There is a robot on an m x n grid. The robot is initially located at the
 * top-left corner (i.e., grid[0][0]). The robot tries to move to the
 * bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move
 * either down or right at any point in time.
 *
 * Given the two integers m and n, return the number of possible unique paths
 * that the robot can take to reach the bottom-right corner.
 *
 * The test cases are generated so that the answer will be less than or equal
 * to 2 * 10^9.
 *
 *
 * Example 1:
 *
 *
 * Input: m = 3, n = 7
 * Output: 28
 *
 *
 * Example 2:
 *
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation: From the top-left corner, there are a total of 3 ways to reach
 * the bottom-right corner:
 * 1. Right -> Down -> Down
 * 2. Down -> Down -> Right
 * 3. Down -> Right -> Down
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= m, n <= 100
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];
        Self::helper(m - 1, n - 1, &mut dp)
    }

    fn helper(i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] == 0 {
            if i == 0 || j == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] = Self::helper(i, j - 1, dp) + Self::helper(i - 1, j, dp);
            }
        }
        dp[i][j]
    }

    pub fn unique_paths_iteraction(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            dp[i][0] = 1;
        }

        for j in 0..n {
            dp[0][j] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }

    pub fn unique_paths_optimization(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];

        for _ in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }

        dp[n - 1]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths_iteraction(3, 7), 28);
    assert_eq!(Solution::unique_paths_optimization(3, 7), 28);
}
