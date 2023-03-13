/*
 * @lc app=leetcode id=97 lang=rust
 *
 * [97] Interleaving String
 *
 * https://leetcode.com/problems/interleaving-string/description/
 *
 * algorithms
 * Medium (36.69%)
 * Likes:    6194
 * Dislikes: 378
 * Total Accepted:    366K
 * Total Submissions: 982.1K
 * Testcase Example:  '"aabcc"\n"dbbca"\n"aadbbcbcac"'
 *
 * Given strings s1, s2, and s3, find whether s3 is formed by an interleaving
 * of s1 and s2.
 *
 * An interleaving of two strings s and t is a configuration where s and t are
 * divided into n and m substrings respectively, such that:
 *
 *
 * s = s1 + s2 + ... + sn
 * t = t1 + t2 + ... + tm
 * |n - m| <= 1
 * The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 +
 * t3 + s3 + ...
 *
 *
 * Note: a + b is the concatenation of strings a and b.
 *
 *
 * Example 1:
 *
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
 * Output: true
 * Explanation: One way to obtain s3 is:
 * Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
 * Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" =
 * "aadbbcbcac".
 * Since s3 can be obtained by interleaving s1 and s2, we return true.
 *
 *
 * Example 2:
 *
 *
 * Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
 * Output: false
 * Explanation: Notice how it is impossible to interleave s2 with any other
 * string to obtain s3.
 *
 *
 * Example 3:
 *
 *
 * Input: s1 = "", s2 = "", s3 = ""
 * Output: true
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= s1.length, s2.length <= 100
 * 0 <= s3.length <= 200
 * s1, s2, and s3 consist of lowercase English letters.
 *
 *
 *
 * Follow up: Could you solve it using only O(s2.length) additional memory
 * space?
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let s3_bytes = s3.as_bytes();

        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];

        dp[0][0] = true;

        for i in 0..s1.len() {
            dp[i + 1][0] = s1_bytes[i] == s3_bytes[i] && dp[i][0];
        }

        for j in 0..s2.len() {
            dp[0][j + 1] = s2_bytes[j] == s3_bytes[j] && dp[0][j];
        }

        for i in 0..s1.len() {
            for j in 0..s2.len() {
                dp[i + 1][j + 1] = (s1_bytes[i] == s3_bytes[i + j + 1] && dp[i][j + 1])
                    || (s2_bytes[j] == s3_bytes[i + j + 1] && dp[i + 1][j])
            }
        }

        dp[s1.len()][s2.len()]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ),
        false
    );
}
