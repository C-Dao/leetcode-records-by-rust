/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 *
 * https://leetcode.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (50.81%)
 * Likes:    6340
 * Dislikes: 666
 * Total Accepted:    981.5K
 * Total Submissions: 1.9M
 * Testcase Example:  '"11"\n"1"'
 *
 * Given two binary strings a and b, return their sum as a binary string.
 *
 *
 * Example 1:
 * Input: a = "11", b = "1"
 * Output: "100"
 * Example 2:
 * Input: a = "1010", b = "1011"
 * Output: "10101"
 *
 *
 * Constraints:
 *
 *
 * 1 <= a.length, b.length <= 10^4
 * a and b consist only of '0' or '1' characters.
 * Each string does not contain leading zeros except for the zero itself.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_count: i32 = a.len() as i32 - 1;
        let mut b_count: i32 = b.len() as i32 - 1;
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut ans = vec![];
        let mut carry = 0;
        while a_count >= 0 || b_count >= 0 {
            let digit_a = if a_count >= 0 {
                a_bytes[a_count as usize] - '0' as u8
            } else {
                0
            };

            let digit_b = if b_count >= 0 {
                b_bytes[b_count as usize] - '0' as u8
            } else {
                0
            };

            let sum = digit_a + digit_b + carry;

            carry = sum / 2;

            let point = sum % 2;

            ans.push(point + '0' as u8);

            a_count -= 1;
            b_count -= 1;
        }

        if carry > 0 {
            ans.push(carry + '0' as u8);
        }

        ans.reverse();
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::add_binary(format!("11"), format!("1")), "100");
}
