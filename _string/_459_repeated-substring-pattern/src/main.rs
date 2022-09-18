/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 *
 * https://leetcode.com/problems/repeated-substring-pattern/description/
 *
 * algorithms
 * Easy (43.65%)
 * Likes:    3743
 * Dislikes: 344
 * Total Accepted:    262.3K
 * Total Submissions: 600.2K
 * Testcase Example:  '"abab"'
 *
 * Given a string s, check if it can be constructed by taking a substring of it
 * and appending multiple copies of the substring together.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abab"
 * Output: true
 * Explanation: It is the substring "ab" twice.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "aba"
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: s = "abcabcabcabc"
 * Output: true
 * Explanation: It is the substring "abc" four times or the substring "abcabc"
 * twice.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s_bytes = s.as_bytes();
        let s_bytes_len = s_bytes.len();

        let next_vec = Self::kmp_next_vec(&s);

        next_vec[s_bytes_len - 1] != 0
            && s_bytes_len % (s_bytes_len - next_vec[s_bytes_len - 1]) == 0
    }

    fn kmp_next_vec(partten: &String) -> Vec<usize> {
        let partten_bytes = partten.as_bytes();
        let partten_bytes_len = partten_bytes.len();

        let mut next = vec![0; partten_bytes_len];

        for i in 1..partten_bytes_len {
            let mut t = next[i - 1];
            while t > 0 && partten_bytes[i] != partten_bytes[t] {
                t = next[t - 1];
            }

            if partten_bytes[i] == partten_bytes[t] {
                t += 1;
            }

            next[i] = t;
        }
        next
    }

    pub fn repeated_substring_pattern_another_edition(s: String) -> bool {
        for i in 1..=s.len() / 2 {
            if s.len() % i != 0 {
                continue;
            }

            if s == s[0..i].repeat(s.len() / i) {
                return true;
            }
        }

        false
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::repeated_substring_pattern("abab".to_string()), true);
    assert_eq!(
        Solution::repeated_substring_pattern("aba".to_string()),
        false
    );
}
