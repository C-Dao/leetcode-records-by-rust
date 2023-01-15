/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 *
 * https://leetcode.com/problems/kth-largest-element-in-an-array/description/
 *
 * algorithms
 * Medium (64.92%)
 * Likes:    12786
 * Dislikes: 642
 * Total Accepted:    1.6M
 * Total Submissions: 2.5M
 * Testcase Example:  '[3,2,1,5,6,4]\n2'
 *
 * Given an integer array nums and an integer k, return the k^th largest
 * element in the array.
 *
 * Note that it is the k^th largest element in the sorted order, not the k^th
 * distinct element.
 *
 * You must solve it in O(n) time complexity.
 *
 *
 * Example 1:
 * Input: nums = [3,2,1,5,6,4], k = 2
 * Output: 5
 * Example 2:
 * Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
 * Output: 4
 *
 *
 * Constraints:
 *
 *
 * 1 <= k <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 *
 *
 */

struct Solution {}

// @lc code=start
use rand::Rng;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    /** quick sort, partition */
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target = nums.len() - k as usize;
        let (mut start, mut end) = (0, nums.len() - 1);

        let mut index = Self::partition(&mut nums, start, end);

        while index != target {
            if index > target {
                end = index - 1;
            } else {
                start = index + 1;
            }
            index = Self::partition(&mut nums, start, end);
        }

        nums[index]
    }

    fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
        let randon_num = rand::thread_rng().gen_range(start, end + 1);

        nums.swap(randon_num, end);

        let mut left_ptr = start;

        for i in start..end {
            if nums[i] < nums[end] {
                nums.swap(i, left_ptr);
                left_ptr += 1;
            }
        }

        nums.swap(left_ptr, end);

        left_ptr
    }

    /** min_heap */
    pub fn find_kth_largest_edition_min_heap(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();

        for i in 0..nums.len() {
            min_heap.push(Reverse(nums[i]));

            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }

        let Reverse(ans) = min_heap.peek().unwrap();

        *ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(Solution::find_kth_largest_edition_min_heap(vec![3, 2, 1, 5, 6, 4], 2), 5);
}
