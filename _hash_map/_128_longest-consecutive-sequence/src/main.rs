/*
 * @lc app=leetcode id=128 lang=rust
 *
 * [128] Longest Consecutive Sequence
 *
 * https://leetcode.com/problems/longest-consecutive-sequence/description/
 *
 * algorithms
 * Medium (49.08%)
 * Likes:    15374
 * Dislikes: 631
 * Total Accepted:    1.1M
 * Total Submissions: 2.2M
 * Testcase Example:  '[100,4,200,1,3,2]'
 *
 * Given an unsorted array of integers nums, return the length of the longest
 * consecutive elements sequence.
 *
 * You must write an algorithm that runs in O(n) time.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [100,4,200,1,3,2]
 * Output: 4
 * Explanation: The longest consecutive elements sequence is [1, 2, 3, 4].
 * Therefore its length is 4.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,3,7,2,5,8,4,6,0,1]
 * Output: 9
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hash_set = HashSet::new();
        let mut longest_streak = 0;

        for num in nums {
            hash_set.insert(num);
        }

        for &num in hash_set.iter() {
            if !hash_set.contains(&(num - 1)) {
                let mut cur = num;
                let mut cur_streak = 1;

                while hash_set.contains(&(cur + 1)) {
                    cur += 1;
                    cur_streak += 1;
                }

                longest_streak = i32::max(cur_streak, longest_streak);
            }
        }

        longest_streak
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}
