/*
 * @lc app=leetcode id=1071 lang=rust
 *
 * [1071] Greatest Common Divisor of Strings
 *
 * https://leetcode.com/problems/greatest-common-divisor-of-strings/description/
 *
 * algorithms
 * Easy (51.26%)
 * Likes:    2786
 * Dislikes: 428
 * Total Accepted:    133.9K
 * Total Submissions: 240.6K
 * Testcase Example:  '"ABCABC"\n"ABC"'
 *
 * For two strings s and t, we say "t divides s" if and only if s = t + ... + t
 * (i.e., t is concatenated with itself one or more times).
 *
 * Given two strings str1 and str2, return the largest string x such that x
 * divides both str1 and str2.
 *
 *
 * Example 1:
 *
 *
 * Input: str1 = "ABCABC", str2 = "ABC"
 * Output: "ABC"
 *
 *
 * Example 2:
 *
 *
 * Input: str1 = "ABABAB", str2 = "ABAB"
 * Output: "AB"
 *
 *
 * Example 3:
 *
 *
 * Input: str1 = "LEET", str2 = "CODE"
 * Output: ""
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= str1.length, str2.length <= 1000
 * str1 and str2 consist of English uppercase letters.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return "".to_string();
        }

        let len = Self::gcd(str1.len(), str2.len());
        str1[0..len].to_string()
    }

    fn gcd(a: usize, b: usize) -> usize {
        match b > 0 {
            true => Self::gcd(b, a % b),
            false => a,
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
        "ABC"
    );
}
