/*
 * @lc app=leetcode id=516 lang=rust
 *
 * [516] Longest Palindromic Subsequence
 *
 * https://leetcode.com/problems/longest-palindromic-subsequence/description/
 *
 * algorithms
 * Medium (59.95%)
 * Likes:    6497
 * Dislikes: 263
 * Total Accepted:    304.4K
 * Total Submissions: 501.7K
 * Testcase Example:  '"bbbab"'
 *
 * Given a string s, find the longest palindromic subsequence's length in s.
 *
 * A subsequence is a sequence that can be derived from another sequence by
 * deleting some or no elements without changing the order of the remaining
 * elements.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "bbbab"
 * Output: 4
 * Explanation: One possible longest palindromic subsequence is "bbbb".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "cbbd"
 * Output: 2
 * Explanation: One possible longest palindromic subsequence is "bb".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consists only of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s_len = s.len();
        let s_bytes = s.as_bytes();
        let mut dp = vec![vec![0; s_len]; s_len];

        for i in (0..s_len).rev() {
            dp[i][i] = 1;
            for j in i + 1..s_len {
                if s_bytes[j] == s_bytes[i] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = i32::max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }

        dp[0][s_len - 1]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
}
