/*
 * @lc app=leetcode id=926 lang=rust
 *
 * [926] Flip String to Monotone Increasing
 *
 * https://leetcode.com/problems/flip-string-to-monotone-increasing/description/
 *
 * algorithms
 * Medium (59.28%)
 * Likes:    3724
 * Dislikes: 159
 * Total Accepted:    153.7K
 * Total Submissions: 251.9K
 * Testcase Example:  '"00110"'
 *
 * A binary string is monotone increasing if it consists of some number of 0's
 * (possibly none), followed by some number of 1's (also possibly none).
 *
 * You are given a binary string s. You can flip s[i] changing it from 0 to 1
 * or from 1 to 0.
 *
 * Return the minimum number of flips to make s monotone increasing.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "00110"
 * Output: 1
 * Explanation: We flip the last digit to get 00111.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "010110"
 * Output: 2
 * Explanation: We flip to get 011111, or alternatively 000111.
 *
 *
 * Example 3:
 *
 *
 * Input: s = "00011000"
 * Output: 2
 * Explanation: We flip to get 00000000.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^5
 * s[i] is either '0' or '1'.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        // dp[i] = dp[i-1]                    // s[i] == '1'
        // dp[i] = min(count_1, dp[i-1] + 1)  // s[i] == '0' count_1 = count of '1' in s[0..i-1]
        let (mut ans, mut num) = (0, 0);

        for &c in s.as_bytes() {
            if c == b'0' {
                ans = i32::min(num, ans + 1);
            } else {
                num += 1;
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::min_flips_mono_incr("00110".to_string()), 1);
}
