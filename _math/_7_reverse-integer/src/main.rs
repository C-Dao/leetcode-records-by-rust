/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 *
 * https://leetcode.com/problems/reverse-integer/description/
 *
 * algorithms
 * Medium (26.89%)
 * Likes:    8445
 * Dislikes: 10717
 * Total Accepted:    2.3M
 * Total Submissions: 8.3M
 * Testcase Example:  '123'
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If
 * reversing x causes the value to go outside the signed 32-bit integer range
 * [-2^31, 2^31 - 1], then return 0.
 *
 * Assume the environment does not allow you to store 64-bit integers (signed
 * or unsigned).
 *
 *
 * Example 1:
 *
 *
 * Input: x = 123
 * Output: 321
 *
 *
 * Example 2:
 *
 *
 * Input: x = -123
 * Output: -321
 *
 *
 * Example 3:
 *
 *
 * Input: x = 120
 * Output: 21
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= x <= 2^31 - 1
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut ans: i32 = 0;

        while x != 0 {
            if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
                return 0;
            };
            ans *= 10;
            ans += x % 10;
            x = x / 10;
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(1534236469), 0);
    assert_eq!(Solution::reverse(-2147483648), 0);
    assert_eq!(Solution::reverse(-2147483412), -2143847412);
}
