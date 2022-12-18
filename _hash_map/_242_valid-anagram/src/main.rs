/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 *
 * https://leetcode.com/problems/valid-anagram/description/
 *
 * algorithms
 * Easy (61.81%)
 * Likes:    7716
 * Dislikes: 250
 * Total Accepted:    1.9M
 * Total Submissions: 2.9M
 * Testcase Example:  '"anagram"\n"nagaram"'
 *
 * Given two strings s and t, return true if t is an anagram of s, and false
 * otherwise.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 * Input: s = "anagram", t = "nagaram"
 * Output: true
 * Example 2:
 * Input: s = "rat", t = "car"
 * Output: false
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length, t.length <= 5 * 10^4
 * s and t consist of lowercase English letters.
 *
 *
 *
 * Follow up: What if the inputs contain Unicode characters? How would you
 * adapt your solution to such a case?
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        };

        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
        let mut letter_counts = [0; 26];

        for i in 0..s.len() {
            letter_counts[(s_bytes[i] - b'a') as usize] += 1;
        }

        for i in 0..t.len() {
            letter_counts[(t_bytes[i] - b'a') as usize] -= 1;
        }

        letter_counts
            .into_iter()
            .find(|letter| letter != &0)
            .is_none()
    }
}
// @lc code=end

fn main() {
    assert!(Solution::is_anagram(
        "anagram".to_string(),
        "nagaram".to_string()
    ));
}
