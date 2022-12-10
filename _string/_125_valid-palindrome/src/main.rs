/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (42.44%)
 * Likes:    5495
 * Dislikes: 6354
 * Total Accepted:    1.7M
 * Total Submissions: 3.8M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * A phrase is a palindrome if, after converting all uppercase letters into
 * lowercase letters and removing all non-alphanumeric characters, it reads the
 * same forward and backward. Alphanumeric characters include letters and
 * numbers.
 *
 * Given a string s, return true if it is a palindrome, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: s = " "
 * Output: true
 * Explanation: s is an empty string "" after removing non-alphanumeric
 * characters.
 * Since an empty string reads the same forward and backward, it is a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 2 * 10^5
 * s consists only of printable ASCII characters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let (mut lp, mut rp, s_bytes) = (0, s.len() - 1, s.as_bytes());

        while lp < rp {
            if !(s_bytes[lp] as char).is_ascii_alphanumeric() {
                lp += 1;
            } else if !(s_bytes[rp] as char).is_ascii_alphanumeric() {
                rp -= 1;
            } else {
                let char_lp = (s_bytes[lp] as char).to_ascii_lowercase();
                let char_rp = (s_bytes[rp] as char).to_ascii_lowercase();

                if char_lp != char_rp {
                    return false;
                };

                lp += 1;
                rp -= 1;
            };
        }

        true
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    )
}
