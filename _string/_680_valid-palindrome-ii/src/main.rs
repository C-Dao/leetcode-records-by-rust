/*
 * @lc app=leetcode id=680 lang=rust
 *
 * [680] Valid Palindrome II
 *
 * https://leetcode.com/problems/valid-palindrome-ii/description/
 *
 * algorithms
 * Easy (39.42%)
 * Likes:    6611
 * Dislikes: 341
 * Total Accepted:    561.5K
 * Total Submissions: 1.4M
 * Testcase Example:  '"aba"'
 *
 * Given a string s, return true if the s can be palindrome after deleting at
 * most one character from it.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "aba"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "abca"
 * Output: true
 * Explanation: You could delete the character 'c'.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "abc"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^5
 * s consists of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let (mut lp, mut rp, s_bytes) = (0, s.len() - 1, s.as_bytes());

        while lp < s.len() / 2 {
            if s_bytes[lp] != s_bytes[rp] {
                break;
            }

            lp += 1;
            rp -= 1;
        }

        lp == s.len() / 2
            || Self::is_palindrome(s_bytes, lp, rp - 1)
            || Self::is_palindrome(s_bytes, lp + 1, rp)
    }

    fn is_palindrome(s_bytes: &[u8], mut start: usize, mut end: usize) -> bool {
        while start < end {
            if s_bytes[start] != s_bytes[end] {
                break;
            }
            start += 1;
            end -= 1;
        }
        start >= end
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::valid_palindrome("aba".to_string()), true)
}
