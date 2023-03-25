/*
 * @lc app=leetcode id=43 lang=rust
 *
 * [43] Multiply Strings
 *
 * https://leetcode.com/problems/multiply-strings/description/
 *
 * algorithms
 * Medium (38.14%)
 * Likes:    5966
 * Dislikes: 2674
 * Total Accepted:    644.9K
 * Total Submissions: 1.6M
 * Testcase Example:  '"2"\n"3"'
 *
 * Given two non-negative integers num1 and num2 represented as strings, return
 * the product of num1 and num2, also represented as a string.
 *
 * Note: You must not use any built-in BigInteger library or convert the inputs
 * to integer directly.
 *
 *
 * Example 1:
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * Example 2:
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 *
 *
 * Constraints:
 *
 *
 * 1 <= num1.length, num2.length <= 200
 * num1 and num2 consist of digits only.
 * Both num1 and num2 do not contain any leading zero, except the number 0
 * itself.
 *
 *
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let (num1_bytes, num2_bytes) = (num1.as_bytes(), num2.as_bytes());

        if num1 == "0" || num2 == "0" {
            return format!("0");
        }

        let mut digits = vec![0; num1.len() + num2.len()];
        for (idx1, ch1) in num1_bytes.iter().rev().enumerate() {
            for (idx2, ch2) in num2_bytes.iter().rev().enumerate() {
                let n1 = ch1 - b'0';
                let n2 = ch2 - b'0';
                let res = n1 * n2 + digits[idx1 + idx2];
                digits[idx1 + idx2] = res % 10;
                digits[idx1 + idx2 + 1] += res / 10;
            }
        }
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }
       
        digits
            .into_iter()
            .rev()
            .map(|d| d.to_string())
            .collect::<String>()
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::multiply(format!("21"), format!("32")), "672");
}
