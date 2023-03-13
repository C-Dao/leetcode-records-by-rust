/*
 * @lc app=leetcode id=873 lang=rust
 *
 * [873] Length of Longest Fibonacci Subsequence
 *
 * https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/description/
 *
 * algorithms
 * Medium (48.64%)
 * Likes:    1778
 * Dislikes: 62
 * Total Accepted:    55K
 * Total Submissions: 113.6K
 * Testcase Example:  '[1,2,3,4,5,6,7,8]'
 *
 * A sequence x1, x2, ..., xn is Fibonacci-like if:
 *
 *
 * n >= 3
 * xi + xi+1 == xi+2 for all i + 2 <= n
 *
 *
 * Given a strictly increasing array arr of positive integers forming a
 * sequence, return the length of the longest Fibonacci-like subsequence of
 * arr. If one does not exist, return 0.
 *
 * A subsequence is derived from another sequence arr by deleting any number of
 * elements (including none) from arr, without changing the order of the
 * remaining elements. For example, [3, 5, 8] is a subsequence of [3, 4, 5, 6,
 * 7, 8].
 *
 *
 * Example 1:
 *
 *
 * Input: arr = [1,2,3,4,5,6,7,8]
 * Output: 5
 * Explanation: The longest subsequence that is fibonacci-like: [1,2,3,5,8].
 *
 * Example 2:
 *
 *
 * Input: arr = [1,3,7,11,12,14,18]
 * Output: 3
 * Explanation: The longest subsequence that is fibonacci-like: [1,11,12],
 * [3,11,14] or [7,11,18].
 *
 *
 * Constraints:
 *
 *
 * 3 <= arr.length <= 1000
 * 1 <= arr[i] < arr[i + 1] <= 10^9
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for i in 0..arr.len() {
            map.insert(arr[i], i as i32);
        }

        let mut dp = vec![vec![0; arr.len()]; arr.len()];
        let mut ans = 2;

        for i in 1..arr.len() {
            for j in 0..i {
                // f(i, j) = f(j, k) + 1
                let k = *map.get(&(arr[i] - arr[j])).unwrap_or(&-1);
                dp[i][j] = if k >= 0 && k < j as i32 {
                    dp[j][k as usize] + 1
                } else {
                    2
                };
                ans = i32::max(ans, dp[i][j] as i32);
            }
        }

        if ans > 2 {
            ans
        } else {
            0
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::len_longest_fib_subseq(vec![1,3,7,11,12,14,18]), 3);
}
