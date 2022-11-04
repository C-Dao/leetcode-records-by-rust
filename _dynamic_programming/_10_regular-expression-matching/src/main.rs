/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 *
 * https://leetcode.com/problems/regular-expression-matching/description/
 *
 * algorithms
 * Hard (28.28%)
 * Likes:    9050
 * Dislikes: 1412
 * Total Accepted:    727.9K
 * Total Submissions: 2.6M
 * Testcase Example:  '"aa"\n"a"'
 *
 * Given an input string s and a pattern p, implement regular expression
 * matching with support for '.' and '*' where:
 *
 *
 * '.' Matches any single character.​​​​
 * '*' Matches zero or more of the preceding element.
 *
 *
 * The matching should cover the entire input string (not partial).
 *
 *
 * Example 1:
 *
 *
 * Input: s = "aa", p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "aa", p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the preceding element, 'a'.
 * Therefore, by repeating 'a' once, it becomes "aa".
 *
 *
 * Example 3:
 *
 *
 * Input: s = "ab", p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 20
 * 1 <= p.length <= 30
 * s contains only lowercase English letters.
 * p contains only lowercase English letters, '.', and '*'.
 * It is guaranteed for each appearance of the character '*', there will be a
 * previous valid character to match.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_len = s.len();
        let p_len = p.len();
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();

        let mut f = vec![vec![false; p_len + 1]; s_len + 1];
        f[0][0] = true;
        for i in 0..=s_len {
            for j in 1..=p_len {
                if p_bytes[j - 1] == '*' as u8 {
                    f[i][j] |= f[i][j - 2];
                    if Self::matches(s_bytes, p_bytes, i, j - 1) {
                        f[i][j] |= f[i - 1][j];
                    };
                } else {
                    if Self::matches(s_bytes, p_bytes, i, j) {
                        f[i][j] |= f[i - 1][j - 1];
                    };
                };
            }
        }
        f[s_len][p_len]
    }

    fn matches(s: &[u8], p: &[u8], i: usize, j: usize) -> bool {
        if i == 0 {
            false
        } else if p[j - 1] == '.' as u8 {
            true
        } else {
            s[i - 1] == p[j - 1]
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::is_match("aaa".to_string(), "a".to_string()),false);
}
