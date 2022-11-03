/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 *
 * https://leetcode.com/problems/palindrome-number/description/
 *
 * algorithms
 * Easy (52.59%)
 * Likes:    7253
 * Dislikes: 2302
 * Total Accepted:    2.5M
 * Total Submissions: 4.7M
 * Testcase Example:  '121'
 *
 * Given an integer x, return true if x is palindrome integer.
 *
 * An integer is a palindrome when it reads the same backward as forward.
 *
 *
 * For example, 121 is a palindrome while 123 is not.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 *
 *
 * Example 2:
 *
 *
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it
 * becomes 121-. Therefore it is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 *
 * Follow up: Could you solve it without converting the integer to a string?
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || x % 10 == 0 && x != 0 {
            return false;
        }

        let mut x_reverted = 0;

        while x > x_reverted {
            x_reverted = x_reverted * 10 + x % 10;
            x = x / 10;
        }

        x == x_reverted || x == x_reverted / 10
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-111), false);
}
