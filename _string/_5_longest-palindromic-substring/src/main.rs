/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 *
 * https://leetcode.com/problems/longest-palindromic-substring/description/
 *
 * algorithms
 * Medium (32.23%)
 * Likes:    21609
 * Dislikes: 1236
 * Total Accepted:    2.1M
 * Total Submissions: 6.5M
 * Testcase Example:  '"babad"'
 *
 * Given a string s, return the longest palindromic substring in s.
 *
 * A string is called a palindrome string if the reverse of that string is the
 * same as the original string.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "cbbd"
 * Output: "bb"
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 1000
 * s consist of only digits and English letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    // dynamic solution
    pub fn another_longest_palindrome(s: String) -> String {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut dp = vec![vec![false; n]; n];
        let mut max_len = 1;
        let mut start = 0;

        if n < 2 {
            return s;
        }

        for i in 0..n {
            dp[i][i] = true;
        }

        for len in 2..=n {
            for i in 0..n {
                let j = i + len - 1;
                if j >= n {
                    break;
                }
                if s_bytes[i] == s_bytes[j] {
                    if j - i == 1 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }
                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    start = i;
                }
            }
        }
        String::from_utf8(s_bytes[start..start + max_len].to_vec()).unwrap()
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let (mut start, mut end, s_len, s_bytes): (i32, i32, i32, &[u8]) =
            (0, 0, s.len() as i32, s.as_bytes());

        for i in 0..s_len {
            let (mut l, mut r): (i32, i32) = (i, i);
            while l > -1 && r < s_len && s_bytes[l as usize] == s_bytes[r as usize] {
                if r - l > end - start {
                    start = l;
                    end = r;
                }
                l -= 1;
                r += 1;
            }

            let (mut l, mut r): (i32, i32) = (i, i + 1);
            while l > -1 && r < s_len && s_bytes[l as usize] == s_bytes[r as usize] {
                if r - l > end - start {
                    start = l;
                    end = r;
                }
                l -= 1;
                r += 1;
            }
        }
        String::from_utf8(s_bytes[start as usize..=end as usize].to_vec()).unwrap()
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
    assert_eq!(Solution::longest_palindrome("cbdd".to_string()), "dd");
    assert_eq!(
        Solution::another_longest_palindrome("babad".to_string()),
        "bab"
    );
    assert_eq!(
        Solution::another_longest_palindrome("cbdd".to_string()),
        "dd"
    );
}
