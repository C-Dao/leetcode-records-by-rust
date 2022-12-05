/*
 * @lc app=leetcode id=560 lang=rust
 *
 * [560] Subarray Sum Equals K
 *
 * https://leetcode.com/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (44.19%)
 * Likes:    16396
 * Dislikes: 483
 * Total Accepted:    901.6K
 * Total Submissions: 2.1M
 * Testcase Example:  '[1,1,1]\n2'
 *
 * Given an array of integers nums and an integer k, return the total number of
 * subarrays whose sum equals to k.
 *
 * A subarray is a contiguous non-empty sequence of elements within an
 * array.
 *
 *
 * Example 1:
 * Input: nums = [1,1,1], k = 2
 * Output: 2
 * Example 2:
 * Input: nums = [1,2,3], k = 3
 * Output: 2
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 2 * 10^4
 * -1000 <= nums[i] <= 1000
 * -10^7 <= k <= 10^7
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut hash_map = HashMap::new();

        hash_map.insert(0, 1);

        for i in 0..nums.len() {
            sum += nums[i];

            if hash_map.contains_key(&(sum - k)) {
                count += hash_map.get(&(sum - k)).unwrap();
            }

            hash_map.insert(sum, hash_map.get(&sum).unwrap_or(&0) + 1);
        }

        count
    }

    pub fn subarray_sum_2(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for sp in 0..nums.len() {
            let mut sum = 0;
            for ep in sp..nums.len() {
                sum += nums[ep];
                if sum == k {
                    count += 1;
                }
            }
        }

        count
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum_2(vec![1, 1, 1], 2), 2);
    assert_eq!(
        Solution::subarray_sum(vec![3, 4, 7, 2, -3, 1, 4, 2, 1], 7),
        6
    );
    assert_eq!(
        Solution::subarray_sum_2(vec![3, 4, 7, 2, -3, 1, 4, 2, 1], 7),
        6
    );
}
