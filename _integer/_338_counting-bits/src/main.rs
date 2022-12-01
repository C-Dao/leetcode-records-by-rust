/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 *
 * https://leetcode.com/problems/counting-bits/description/
 *
 * algorithms
 * Easy (74.65%)
 * Likes:    8117
 * Dislikes: 384
 * Total Accepted:    699.8K
 * Total Submissions: 929.1K
 * Testcase Example:  '2'
 *
 * Given an integer n, return an array ans of length n + 1 such that for each i
 * (0 <= i <= n), ans[i] is the number of 1's in the binary representation of
 * i.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 2
 * Output: [0,1,1]
 * Explanation:
 * 0 --> 0
 * 1 --> 1
 * 2 --> 10
 *
 *
 * Example 2:
 *
 *
 * Input: n = 5
 * Output: [0,1,1,2,1,2]
 * Explanation:
 * 0 --> 0
 * 1 --> 1
 * 2 --> 10
 * 3 --> 11
 * 4 --> 100
 * 5 --> 101
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= n <= 10^5
 *
 *
 *
 * Follow up:
 *
 *
 * It is very easy to come up with a solution with a runtime of O(n log n). Can
 * you do it in linear time O(n) and possibly in a single pass?
 * Can you do it without using any built-in function (i.e., like
 * __builtin_popcount in C++)?
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 1..=n {
            ans[i as usize] = ans[i as usize & (i as usize - 1)] + 1;
        }
        ans
    }

    pub fn count_bits_2(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 1];
        for i in 1..=n {
            ans[i as usize] = ans[i as usize >> 1] + i & 1;
        }
        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    assert_eq!(Solution::count_bits_2(2), vec![0, 1, 1]);
}
