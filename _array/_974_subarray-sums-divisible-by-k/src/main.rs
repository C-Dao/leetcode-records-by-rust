/*
 * @lc app=leetcode id=974 lang=rust
 *
 * [974] Subarray Sums Divisible by K
 *
 * https://leetcode.com/problems/subarray-sums-divisible-by-k/description/
 *
 * algorithms
 * Medium (53.18%)
 * Likes:    5322
 * Dislikes: 208
 * Total Accepted:    177.6K
 * Total Submissions: 326K
 * Testcase Example:  '[4,5,0,-2,-3,1]\n5'
 *
 * Given an integer array nums and an integer k, return the number of non-empty
 * subarrays that have a sum divisible by k.
 *
 * A subarray is a contiguous part of an array.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [4,5,0,-2,-3,1], k = 5
 * Output: 7
 * Explanation: There are 7 subarrays with a sum divisible by k = 5:
 * [4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2,
 * -3]
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [5], k = 9
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * -10^4 <= nums[i] <= 10^4
 * 2 <= k <= 10^4
 *
 *
 */

struct Solution {}
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut count_map = HashMap::new();
        let mut prefix_mod = 0;
        let mut ans = 0;

        // ( prefix[i] - prefix[j] ) % k == 0, need to init for 1, because of the last prefix[i] % k == 0 need to count in
        count_map.insert(0, 1);

        for i in 0..nums.len() {
            prefix_mod = (prefix_mod + nums[i]).rem_euclid(k); // (prefix_mod+nums[i] % k + k) % k;
            ans += count_map.get(&prefix_mod).unwrap_or(&0);
            count_map.insert(prefix_mod, count_map.get(&prefix_mod).unwrap_or(&0) + 1);
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    assert_eq!(Solution::subarrays_div_by_k(vec![-1, -9, -4, 0], 9), 2);
}
