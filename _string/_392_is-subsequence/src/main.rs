/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 *
 * https://leetcode.com/problems/is-subsequence/description/
 *
 * algorithms
 * Easy (50.67%)
 * Likes:    6192
 * Dislikes: 347
 * Total Accepted:    661.7K
 * Total Submissions: 1.3M
 * Testcase Example:  '"abc"\n"ahbgdc"'
 *
 * Given two strings s and t, return true if s is a subsequence of t, or false
 * otherwise.
 *
 * A subsequence of a string is a new string that is formed from the original
 * string by deleting some (can be none) of the characters without disturbing
 * the relative positions of the remaining characters. (i.e., "ace" is a
 * subsequence of "abcde" while "aec" is not).
 *
 *
 * Example 1:
 * Input: s = "abc", t = "ahbgdc"
 * Output: true
 * Example 2:
 * Input: s = "axc", t = "ahbgdc"
 * Output: false
 *
 *
 * Constraints:
 *
 *
 * 0 <= s.length <= 100
 * 0 <= t.length <= 10^4
 * s and t consist only of lowercase English letters.
 *
 *
 *
 * Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k
 * >= 10^9, and you want to check one by one to see if t has its subsequence.
 * In this scenario, how would you change your code?
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s_len, t_len, s_bytes, t_bytes) = (s.len(), t.len(), s.as_bytes(), t.as_bytes());
        let mut dp = vec![vec![0; s_len + 1]; t_len + 1];
        for i in 0..t_len + 1 {
            dp[i][s_len] = 1;
        }

        for i in (0..t_len).rev() {
            for j in (0..s_len).rev() {
                dp[i][j] = if t_bytes[i] == s_bytes[j] {
                    dp[i + 1][j] | dp[i + 1][j + 1]
                } else {
                    dp[i + 1][j]
                }
            }
        }

        if dp[0][0] == 1 {
            true
        } else {
            false
        }
    }

    pub fn is_subsequence_2(s: String, t: String) -> bool {
        let (mut sp, mut tp, s_len, t_len, s_bytes, t_bytes) =
            (0, 0, s.len(), t.len(), s.as_bytes(), t.as_bytes());
        while sp < s_len && tp < t_len {
            if s_bytes[sp] == t_bytes[tp] {
                sp += 1;
            };
            tp += 1;
        }
        sp == s_len
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
        true
    );
    assert_eq!(
        Solution::is_subsequence_2("abc".to_string(), "ahbgdc".to_string()),
        true
    );
}
