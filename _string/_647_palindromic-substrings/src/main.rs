/*
 * @lc app=leetcode id=647 lang=rust
 *
 * [647] Palindromic Substrings
 *
 * https://leetcode.com/problems/palindromic-substrings/description/
 *
 * algorithms
 * Medium (65.85%)
 * Likes:    8324
 * Dislikes: 179
 * Total Accepted:    523.4K
 * Total Submissions: 788.6K
 * Testcase Example:  '"abc"'
 *
 * Given a string s, return the number of palindromic substrings in it.
 *
 * A string is a palindrome when it reads the same backward as forward.
 *
 * A substring is a contiguous sequence of characters within the string.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "abc"
 * Output: 3
 * Explanation: Three palindromic strings: "a", "b", "c".
 *
 *
 * Example 2:
 *
 *
 * Input: s = "aaa"
 * Output: 6
 * Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consists of lowercase English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0;
        let s_bytes = s.as_bytes();
        for i in 0..s.len() as i32 {
            ans += Self::count_palindrome(s_bytes, i, i);
            ans += Self::count_palindrome(s_bytes, i, i + 1);
        }
        ans
    }

    fn count_palindrome(s_bytes: &[u8], mut start: i32, mut end: i32) -> i32 {
        let mut count = 0;
        while start >= 0
            && end < s_bytes.len() as i32
            && s_bytes[start as usize] == s_bytes[end as usize]
        {
            count += 1;
            start -= 1;
            end += 1;
        }
        count
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
}
