/*
 * @lc app=leetcode id=214 lang=rust
 *
 * [214] Shortest Palindrome
 *
 * https://leetcode.com/problems/shortest-palindrome/description/
 *
 * algorithms
 * Hard (31.93%)
 * Likes:    2715
 * Dislikes: 197
 * Total Accepted:    143.6K
 * Total Submissions: 448.2K
 * Testcase Example:  '"aacecaaa"'
 *
 * You are given a string s. You can convert s to a palindrome by adding
 * characters in front of it.
 * 
 * Return the shortest palindrome you can find by performing this
 * transformation.
 * 
 * 
 * Example 1:
 * Input: s = "aacecaaa"
 * Output: "aaacecaaa"
 * Example 2:
 * Input: s = "abcd"
 * Output: "dcbabcd"
 * 
 * 
 * Constraints:
 * 
 * 
 * 0 <= s.length <= 5 * 10^4
 * s consists of lowercase English letters only.
 * 
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let s_bytes = s.as_bytes();
        let len = s_bytes.len();
        let next_vec = Self::kmp_next_vec(&s);

        let mut k = 0;
        for i in (0..len).rev() {
            while k > 0 && s_bytes[i] != s_bytes[k] {
                k = next_vec[k - 1];
            }
            if s_bytes[k] == s_bytes[i] {
                k += 1;
            }
        }

        if k == len {
            s
        } else {
            s_bytes[k..]
                .iter()
                .rev()
                .map(|b| (*b) as char)
                .collect::<String>()
                + s.as_str()
        }
    }

    fn kmp_next_vec(pattern: &String) -> Vec<usize> {
        let pattern_bytes = pattern.as_bytes();
        let len = pattern.as_bytes().len();
        let mut next = vec![0; len];
        for i in 1..len {
            let mut t = next[i - 1];
            while t > 0 && pattern_bytes[i] != pattern_bytes[t] {
                t = next[t - 1];
            }
            if pattern_bytes[t] == pattern_bytes[i] {
               next[i] = t + 1;
            }
        }
        next
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::shortest_palindrome("aacecaaa".to_string()),
        "aaacecaaa"
    );
}
