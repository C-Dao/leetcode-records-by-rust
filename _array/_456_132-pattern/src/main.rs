/*
 * @lc app=leetcode id=456 lang=rust
 *
 * [456] 132 Pattern
 *
 * https://leetcode.com/problems/132-pattern/description/
 *
 * algorithms
 * Medium (32.42%)
 * Likes:    5018
 * Dislikes: 281
 * Total Accepted:    155.1K
 * Total Submissions: 478.3K
 * Testcase Example:  '[1,2,3,4]'
 *
 * Given an array of n integers nums, a 132 pattern is a subsequence of three
 * integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] <
 * nums[k] < nums[j].
 *
 * Return true if there is a 132 pattern in nums, otherwise, return false.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,4]
 * Output: false
 * Explanation: There is no 132 pattern in the sequence.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [3,1,4,2]
 * Output: true
 * Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [-1,3,2,0]
 * Output: true
 * Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1,
 * 3, 0] and [-1, 2, 0].
 *
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 2 * 10^5
 * -10^9 <= nums[i] <= 10^9
 *
 *
 */

struct Solution {}

// @lc code=start
use std::cmp;
use std::collections::VecDeque;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut deque = VecDeque::new();
        let mut part_2 = i32::MIN;
        for i in (0..nums.len()).rev() {
            if nums[i] < part_2 {
                return true;
            }
            while !deque.is_empty() && *deque.back().unwrap() < nums[i] {
                part_2 = cmp::max(part_2, *deque.back().unwrap());
                deque.pop_back();
            }
            deque.push_back(nums[i]);
        }

        false
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
    assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
    assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    assert_eq!(Solution::find132pattern(vec![3, 5, 0, 3, 4]), true);
    assert_eq!(Solution::find132pattern(vec![1, 0, 1, -4, -3]), false);
}
