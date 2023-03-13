/*
 * @lc app=leetcode id=1143 lang=rust
 *
 * [1143] Longest Common Subsequence
 *
 * https://leetcode.com/problems/longest-common-subsequence/description/
 *
 * algorithms
 * Medium (58.99%)
 * Likes:    8757
 * Dislikes: 101
 * Total Accepted:    555.2K
 * Total Submissions: 944.7K
 * Testcase Example:  '"abcde"\n"ace"'
 *
 * Given two strings text1 and text2, return the length of their longest common
 * subsequence. If there is no common subsequence, return 0.
 *
 * A subsequence of a string is a new string generated from the original string
 * with some characters (can be none) deleted without changing the relative
 * order of the remaining characters.
 *
 *
 * For example, "ace" is a subsequence of "abcde".
 *
 *
 * A common subsequence of two strings is a subsequence that is common to both
 * strings.
 *
 *
 * Example 1:
 *
 *
 * Input: text1 = "abcde", text2 = "ace"
 * Output: 3
 * Explanation: The longest common subsequence is "ace" and its length is 3.
 *
 *
 * Example 2:
 *
 *
 * Input: text1 = "abc", text2 = "abc"
 * Output: 3
 * Explanation: The longest common subsequence is "abc" and its length is 3.
 *
 *
 * Example 3:
 *
 *
 * Input: text1 = "abc", text2 = "def"
 * Output: 0
 * Explanation: There is no such common subsequence, so the result is 0.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= text1.length, text2.length <= 1000
 * text1 and text2 consist of only lowercase English characters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1_bytes = text1.as_bytes();
        let text2_bytes = text2.as_bytes();
        let mut dp = vec![vec![0; text2_bytes.len() + 1]; text1_bytes.len() + 1];

        // dp[i][j] 表示 "长度为 i 的 text1 子序列" 和 "长度为 j 的 text2 子序列" 之间的最长子序列的长度

        for i in 0..text1_bytes.len() {
            for j in 0..text2_bytes.len() {
                if text1_bytes[i] == text2_bytes[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = i32::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
        3
    );
}
