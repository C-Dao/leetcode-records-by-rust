/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 *
 * https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
 *
 * algorithms
 * Medium (33.50%)
 * Likes:    28705
 * Dislikes: 1223
 * Total Accepted:    3.8M
 * Total Submissions: 11.4M
 * Testcase Example:  '"abcabcbb"'
 *
 * Given a string s, find the length of the longest substring without repeating
 * characters.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not
 * a substring.
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= s.length <= 5 * 10^4
 * s consists of English letters, digits, symbols and spaces.
 *
 *
 */

struct Solution {}

// @lc code=start

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut vec_set = vec![0; 256];
        let (s_len, s_bytes) = (s.len(), s.as_bytes());
        let (mut lp, mut rp, mut max) = (0, 0, 0);

        while lp < s_len {
            if lp != 0 {
                vec_set[(s_bytes[lp - 1] - b'a') as usize] = 0;
            };
            while rp < s_len && vec_set[(s_bytes[rp] - b'a') as usize] != 1 {
                vec_set[(s_bytes[rp] - b'a') as usize] = 1;
                rp += 1;
            }
            max = usize::max(max, rp - lp);

            lp += 1;
        }

        max as i32
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
}
