/*
 * @lc app=leetcode id=76 lang=rust
 *
 * [76] Minimum Window Substring
 *
 * https://leetcode.com/problems/minimum-window-substring/description/
 *
 * algorithms
 * Hard (39.42%)
 * Likes:    13808
 * Dislikes: 604
 * Total Accepted:    938.6K
 * Total Submissions: 2.3M
 * Testcase Example:  '"ADOBECODEBANC"\n"ABC"'
 *
 * Given two strings s and t of lengths m and n respectively, return the
 * minimum window substring of s such that every character in t (including
 * duplicates) is included in the window. If there is no such substring, return
 * the empty string "".
 *
 * The testcases will be generated such that the answer is unique.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "ADOBECODEBANC", t = "ABC"
 * Output: "BANC"
 * Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C'
 * from string t.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "a", t = "a"
 * Output: "a"
 * Explanation: The entire string s is the minimum window.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "a", t = "aa"
 * Output: ""
 * Explanation: Both 'a's from t must be included in the window.
 * Since the largest window of s only has one 'a', return empty string.
 *
 *
 *
 * Constraints:
 *
 *
 * m == s.length
 * n == t.length
 * 1 <= m, n <= 10^5
 * s and t consist of uppercase and lowercase English letters.
 *
 *
 *
 * Follow up: Could you find an algorithm that runs in O(m + n) time?
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut hash_map = HashMap::new();
        let (s_len, t_len, s_bytes, t_bytes) = (s.len(), t.len(), s.as_bytes(), t.as_bytes());
        let (mut lp, mut rp, mut min_st, mut min_ed, mut min_len) = (0, 0, 0, 0, i32::MAX);

        for i in 0..t_len {
            hash_map.insert(
                t_bytes[i],
                hash_map.get(&t_bytes[i]).unwrap_or(&0).to_owned() + 1,
            );
        }

        let mut count = hash_map.len();

        while rp < s_len || (count == 0 && rp == s_len) {
            if count > 0 {
                if hash_map.contains_key(&s_bytes[rp]) {
                    hash_map.insert(s_bytes[rp], hash_map.get(&s_bytes[rp]).unwrap() - 1);
                    if hash_map.get(&s_bytes[rp]).unwrap_or(&0).to_owned() == 0 {
                        count -= 1;
                    }
                };

                rp += 1;
            } else {
                if (rp as i32 - lp as i32) < min_len {
                    min_len = rp as i32 - lp as i32;
                    min_st = lp;
                    min_ed = rp;
                };

                if hash_map.contains_key(&s_bytes[lp]) {
                    hash_map.insert(s_bytes[lp], hash_map.get(&s_bytes[lp]).unwrap() + 1);
                    if hash_map.get(&s_bytes[lp]).unwrap_or(&0).to_owned() == 1 {
                        count += 1;
                    }
                };
                
                lp += 1;
            };
        }

        if min_len < i32::MAX {
            String::from_utf8(s_bytes[min_st..min_ed].to_vec()).unwrap()
        } else {
            "".to_string()
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC"
    );
}
