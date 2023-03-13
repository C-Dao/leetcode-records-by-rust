/*
 * @lc app=leetcode id=115 lang=rust
 *
 * [115] Distinct Subsequences
 *
 * https://leetcode.com/problems/distinct-subsequences/description/
 *
 * algorithms
 * Hard (42.83%)
 * Likes:    4641
 * Dislikes: 187
 * Total Accepted:    263.1K
 * Total Submissions: 600.9K
 * Testcase Example:  '"rabbbit"\n"rabbit"'
 *
 * Given two strings s and t, return the number of distinct subsequences of s
 * which equals t.
 *
 * The test cases are generated so that the answer fits on a 32-bit signed
 * integer.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "rabbbit", t = "rabbit"
 * Output: 3
 * Explanation:
 * As shown below, there are 3 ways you can generate "rabbit" from s.
 * rabbbit
 * rabbbit
 * rabbbit
 *
 *
 * Example 2:
 *
 *
 * Input: s = "babgbag", t = "bag"
 * Output: 5
 * Explanation:
 * As shown below, there are 5 ways you can generate "bag" from s.
 * babgbag
 * babgbag
 * babgbag
 * babgbag
 * babgbag
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length, t.length <= 1000
 * s and t consist of English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

        if s.len() < t.len() {
            return 0;
        };

        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

        dp[0][0] = 1;

        for i in 0..s.len() {
            dp[i + 1][0] = 1;

            for j in 0..t.len() {
                if j > i {
                    break;
                }
                if s_bytes[i] == t_bytes[j] {
                    dp[i + 1][j + 1] = dp[i][j] + dp[i][j + 1]
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1]
                }
            }
        }

        dp[s.len()][t.len()]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
        3
    );
}
