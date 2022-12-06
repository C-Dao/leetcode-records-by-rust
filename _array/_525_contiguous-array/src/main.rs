/*
 * @lc app=leetcode id=525 lang=rust
 *
 * [525] Contiguous Array
 *
 * https://leetcode.com/problems/contiguous-array/description/
 *
 * algorithms
 * Medium (46.54%)
 * Likes:    6006
 * Dislikes: 241
 * Total Accepted:    285.4K
 * Total Submissions: 609.2K
 * Testcase Example:  '[0,1]'
 *
 * Given a binary array nums, return the maximum length of a contiguous
 * subarray with an equal number of 0 and 1.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [0,1]
 * Output: 2
 * Explanation: [0, 1] is the longest contiguous subarray with an equal number
 * of 0 and 1.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,1,0]
 * Output: 2
 * Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal
 * number of 0 and 1.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * nums[i] is either 0 or 1.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();

        hash_map.insert(0, -1);

        let mut sum: i32 = 0;
        let mut max_length: i32 = 0;

        for i in 0..nums.len() {
            sum += if nums[i] == 0 { -1 } else { 1 };

            if hash_map.contains_key(&sum) {
                max_length = i32::max(max_length, i as i32 - hash_map.get(&sum).unwrap())
            } else {
                hash_map.insert(sum, i as i32);
            }
        }

        max_length
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
}
