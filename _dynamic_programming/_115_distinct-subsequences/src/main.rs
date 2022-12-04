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
        let (s_len, t_len, s_bytes, t_bytes) = (s.len(), t.len(), s.as_bytes(), t.as_bytes());

        if s_len < t_len {
            return 0;
        };

        let mut dp = vec![vec![0; t_len + 1]; s_len + 1];

        for i in 0..=s_len {
            dp[i][t_len] = 1;
        }

        for i in (0..s_len).rev() {
            for j in (0..t_len).rev() {
                dp[i][j] = if s_bytes[i] != t_bytes[j] {
                    dp[i + 1][j]
                } else {
                    dp[i + 1][j + 1] + dp[i + 1][j]
                }
            }
        }

        dp[0][0]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
        3
    );
}
