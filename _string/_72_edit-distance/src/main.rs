/*
 * @lc app=leetcode id=72 lang=rust
 *
 * [72] Edit Distance
 *
 * https://leetcode.com/problems/edit-distance/description/
 *
 * algorithms
 * Hard (51.59%)
 * Likes:    10439
 * Dislikes: 116
 * Total Accepted:    550.6K
 * Total Submissions: 1M
 * Testcase Example:  '"horse"\n"ros"'
 *
 * Given two strings word1 and word2, return the minimum number of operations
 * required to convert word1 to word2.
 *
 * You have the following three operations permitted on a word:
 *
 *
 * Insert a character
 * Delete a character
 * Replace a character
 *
 *
 *
 * Example 1:
 *
 *
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation:
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 *
 *
 * Example 2:
 *
 *
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation:
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= word1.length, word2.length <= 500
 * word1 and word2 consist of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let col_len = word1.len();
        let row_len = word2.len();
        let word1_bytes = word1.as_bytes();
        let word2_bytes = word2.as_bytes();
        let mut dp = vec![vec![0; col_len + 1]; row_len + 1];

        for i in 0..=row_len {
            dp[i][0] = i;
        }

        for i in 0..=col_len {
            dp[0][i] = i;
        }

        for row in 1..=row_len {
            for col in 1..=col_len {
                if word1_bytes[col - 1] == word2_bytes[row - 1] {
                    dp[row][col] = dp[row - 1][col - 1];
                } else {
                    dp[row][col] = usize::min(
                        dp[row - 1][col - 1],
                        usize::min(dp[row][col - 1], dp[row - 1][col]),
                    ) + 1;
                }
            }
        }
        dp[row_len][col_len] as i32
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::min_distance(String::from("horse"), String::from("ros")),
        3
    );
}
