/*
 * @lc app=leetcode id=154 lang=rust
 *
 * [154] Find Minimum in Rotated Sorted Array II
 *
 * https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/description/
 *
 * algorithms
 * Hard (43.30%)
 * Likes:    4018
 * Dislikes: 430
 * Total Accepted:    387.4K
 * Total Submissions: 890K
 * Testcase Example:  '[1,3,5]'
 *
 * Suppose an array of length n sorted in ascending order is rotated between 1
 * and n times. For example, the array nums = [0,1,4,4,5,6,7] might
 * become:
 *
 *
 * [4,5,6,7,0,1,4] if it was rotated 4 times.
 * [0,1,4,4,5,6,7] if it was rotated 7 times.
 *
 *
 * Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results
 * in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
 *
 * Given the sorted rotated array nums that may contain duplicates, return the
 * minimum element of this array.
 *
 * You must decrease the overall operation steps as much as possible.
 *
 *
 * Example 1:
 * Input: nums = [1,3,5]
 * Output: 1
 * Example 2:
 * Input: nums = [2,2,2,0,1]
 * Output: 0
 *
 *
 * Constraints:
 *
 *
 * n == nums.length
 * 1 <= n <= 5000
 * -5000 <= nums[i] <= 5000
 * nums is sorted and rotated between 1 and n times.
 *
 *
 *
 * Follow up: This problem is similar to Find Minimum in Rotated Sorted Array,
 * but nums may contain duplicates. Would this affect the runtime complexity?
 * How and why?
 *
 *
 *
 */

struct Solution {}

// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&nums[right]) {
                Ordering::Greater => {
                    left = mid + 1;
                }
                Ordering::Less => {
                    right = mid;
                }
                Ordering::Equal => right -= 1,
            }
        }

        nums[left]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
}
