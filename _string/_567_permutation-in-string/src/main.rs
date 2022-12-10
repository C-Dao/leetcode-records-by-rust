/*
 * @lc app=leetcode id=567 lang=rust
 *
 * [567] Permutation in String
 *
 * https://leetcode.com/problems/permutation-in-string/description/
 *
 * algorithms
 * Medium (44.25%)
 * Likes:    7691
 * Dislikes: 255
 * Total Accepted:    518.4K
 * Total Submissions: 1.2M
 * Testcase Example:  '"ab"\n"eidbaooo"'
 *
 * Given two strings s1 and s2, return true if s2 contains a permutation of s1,
 * or false otherwise.
 *
 * In other words, return true if one of s1's permutations is the substring of
 * s2.
 *
 *
 * Example 1:
 *
 *
 * Input: s1 = "ab", s2 = "eidbaooo"
 * Output: true
 * Explanation: s2 contains one permutation of s1 ("ba").
 *
 *
 * Example 2:
 *
 *
 * Input: s1 = "ab", s2 = "eidboaoo"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s1.length, s2.length <= 10^4
 * s1 and s2 consist of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1_len, s2_len, s1_bytes, s2_bytes) =
            (s1.len(), s2.len(), s1.as_bytes(), s2.as_bytes());

        let mut alphabet_counts = vec![0; 26];

        if s1_len > s2_len {
            return false;
        };

        for i in 0..s1_len {
            alphabet_counts[(s1_bytes[i] - b'a') as usize] += 1;
            alphabet_counts[(s2_bytes[i] - b'a') as usize] -= 1;
        }

        if Self::check_all_zero(&alphabet_counts) {
            return true;
        };

        for i in s1_len..s2_len {
            alphabet_counts[(s2_bytes[i] - b'a') as usize] -= 1;
            alphabet_counts[(s2_bytes[i - s1_len] - b'a') as usize] += 1;

            if Self::check_all_zero(&alphabet_counts) {
                return true;
            };
        }
        
        false
    }

    fn check_all_zero(counts: &Vec<i32>) -> bool {
        counts.iter().find(|&c| c != &0).is_none()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
        true
    );
}
