/*
 * @lc app=leetcode id=438 lang=rust
 *
 * [438] Find All Anagrams in a String
 *
 * https://leetcode.com/problems/find-all-anagrams-in-a-string/description/
 *
 * algorithms
 * Medium (48.60%)
 * Likes:    9255
 * Dislikes: 287
 * Total Accepted:    645K
 * Total Submissions: 1.3M
 * Testcase Example:  '"cbaebabacd"\n"abc"'
 *
 * Given two strings s and p, return an array of all the start indices of p's
 * anagrams in s. You may return the answer in any order.
 *
 * An Anagram is a word or phrase formed by rearranging the letters of a
 * different word or phrase, typically using all the original letters exactly
 * once.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "cbaebabacd", p = "abc"
 * Output: [0,6]
 * Explanation:
 * The substring with start index = 0 is "cba", which is an anagram of "abc".
 * The substring with start index = 6 is "bac", which is an anagram of "abc".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "abab", p = "ab"
 * Output: [0,1,2]
 * Explanation:
 * The substring with start index = 0 is "ab", which is an anagram of "ab".
 * The substring with start index = 1 is "ba", which is an anagram of "ab".
 * The substring with start index = 2 is "ab", which is an anagram of "ab".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length, p.length <= 3 * 10^4
 * s and p consist of lowercase English letters.
 *
 *
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s_len, p_len, s_bytes, p_bytes) = (s.len(), p.len(), s.as_bytes(), p.as_bytes());
        let mut alphabet_counts = vec![0; 26];
        let mut ans = vec![];
        if p.len() > s.len() {
            return ans;
        };

        for i in 0..p_len {
            alphabet_counts[(p_bytes[i] - b'a') as usize] += 1;
            alphabet_counts[(s_bytes[i] - b'a') as usize] -= 1;
        }

        if Self::check_all_zero(&alphabet_counts) {
            ans.push(0);
        };

        for i in p_len..s_len {
            alphabet_counts[(s_bytes[i] - b'a') as usize] -= 1;
            alphabet_counts[(s_bytes[i - p_len] - b'a') as usize] += 1;

            if Self::check_all_zero(&alphabet_counts) {
                ans.push((i - p_len + 1) as i32);
            };
        }

        ans
    }

    fn check_all_zero(counts: &Vec<i32>) -> bool {
        counts.iter().find(|&c| c != &0).is_none()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
        vec![0, 6]
    );
}
