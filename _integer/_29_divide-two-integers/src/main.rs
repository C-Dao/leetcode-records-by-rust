/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 *
 * https://leetcode.com/problems/divide-two-integers/description/
 *
 * algorithms
 * Medium (17.48%)
 * Likes:    3729
 * Dislikes: 12175
 * Total Accepted:    560.2K
 * Total Submissions: 3.2M
 * Testcase Example:  '10\n3'
 *
 * Given two integers dividend and divisor, divide two integers without using
 * multiplication, division, and mod operator.
 *
 * The integer division should truncate toward zero, which means losing its
 * fractional part. For example, 8.345 would be truncated to 8, and -2.7335
 * would be truncated to -2.
 *
 * Return the quotient after dividing dividend by divisor.
 *
 * Note: Assume we are dealing with an environment that could only store
 * integers within the 32-bit signed integer range: [−2^31, 2^31 − 1]. For this
 * problem, if the quotient is strictly greater than 2^31 - 1, then return 2^31
 * - 1, and if the quotient is strictly less than -2^31, then return -2^31.
 *
 *
 * Example 1:
 *
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * Explanation: 10/3 = 3.33333.. which is truncated to 3.
 *
 *
 * Example 2:
 *
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * Explanation: 7/-3 = -2.33333.. which is truncated to -2.
 *
 *
 *
 * Constraints:
 *
 *
 * -2^31 <= dividend, divisor <= 2^31 - 1
 * divisor != 0
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        };

        let mut negative: i32 = 2;
        let mut ans = 0;

        if dividend > 0 {
            negative -= 1;
            dividend = -dividend;
        };

        if divisor > 0 {
            negative -= 1;
            divisor = -divisor;
        };

        while dividend <= divisor {
            let mut subtractor = divisor;
            let mut quotient = 1;
            while subtractor >= -i32::pow(2, 30) && dividend <= subtractor + subtractor {
                quotient += quotient;
                subtractor += subtractor;
            }
            ans += quotient;
            dividend -= subtractor;
        }

        if negative == 1 {
            -ans
        } else {
            ans
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
}
