/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 *
 * https://leetcode.com/problems/word-pattern/description/
 *
 * algorithms
 * Easy (40.27%)
 * Likes:    4971
 * Dislikes: 568
 * Total Accepted:    440.4K
 * Total Submissions: 1.1M
 * Testcase Example:  '"abba"\n"dog cat cat dog"'
 *
 * Given a pattern and a string s, find if s follows the same pattern.
 *
 * Here follow means a full match, such that there is a bijection between a
 * letter in pattern and a non-empty word in s.
 *
 *
 * Example 1:
 *
 *
 * Input: pattern = "abba", s = "dog cat cat dog"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: pattern = "abba", s = "dog cat cat fish"
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: pattern = "aaaa", s = "dog cat cat dog"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= pattern.length <= 300
 * pattern contains only lower-case English letters.
 * 1 <= s.length <= 3000
 * s contains only lowercase English letters and spaces ' '.
 * s does not contain any leading or trailing spaces.
 * All the words in s are separated by a single space.
 *
 *
 */
struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut word_map_byte = HashMap::new();
        let mut byte_map_word = HashMap::new();
        let pattern_bytes = pattern.as_bytes();
        let words = s.split_whitespace().collect::<Vec<&str>>();

        if words.len() != pattern_bytes.len() {
            return false;
        }

        for i in 0..pattern_bytes.len() {
            if word_map_byte.contains_key(&pattern_bytes[i])
                && word_map_byte.get(&pattern_bytes[i]).unwrap() != &words[i]
                || byte_map_word.contains_key(&words[i])
                    && byte_map_word.get(&words[i]).unwrap() != &pattern_bytes[i]
            {
                return false;
            }

            word_map_byte.insert(pattern_bytes[i], words[i]);
            byte_map_word.insert(words[i], pattern_bytes[i]);
        }
        true
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
}
