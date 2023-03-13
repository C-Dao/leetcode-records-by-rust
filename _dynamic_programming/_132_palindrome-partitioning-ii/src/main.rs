/*
 * @lc app=leetcode id=132 lang=rust
 *
 * [132] Palindrome Partitioning II
 *
 * https://leetcode.com/problems/palindrome-partitioning-ii/description/
 *
 * algorithms
 * Hard (33.33%)
 * Likes:    4507
 * Dislikes: 104
 * Total Accepted:    235.3K
 * Total Submissions: 697.2K
 * Testcase Example:  '"aab"'
 *
 * Given a string s, partition s such that every substring of the partition is
 * a palindrome.
 *
 * Return the minimum cuts needed for a palindrome partitioning of s.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "aab"
 * Output: 1
 * Explanation: The palindrome partitioning ["aa","b"] could be produced using
 * 1 cut.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "a"
 * Output: 0
 *
 *
 * Example 3:
 *
 *
 * Input: s = "ab"
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 2000
 * s consists of lowercase English letters only.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let mut is_pal = vec![vec![false; s.len()]; s.len()];
        let s_bytes = s.as_bytes();
        for i in 0..s.len() {
            for j in 0..=i {
                if s_bytes[i] == s_bytes[j] && (i <= j + 1 || is_pal[j + 1][i - 1]) {
                    is_pal[j][i] = true;
                }
            }
        }

        let mut dp = vec![0; s.len()];

        for i in 0..s.len() {
            if !is_pal[0][i] {
                dp[i] = i as i32;
                for j in 1..=i {
                    if is_pal[j][i] {
                        dp[i] = i32::min(dp[i], dp[j - 1] + 1);
                    }
                }
            }
        }

        dp[s.len() - 1] as i32
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::min_cut("aab".to_string()), 1);
}
