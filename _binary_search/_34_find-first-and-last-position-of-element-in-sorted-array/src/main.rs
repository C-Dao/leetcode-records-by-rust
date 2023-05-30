/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
 *
 * algorithms
 * Medium (40.43%)
 * Likes:    16926
 * Dislikes: 406
 * Total Accepted:    1.5M
 * Total Submissions: 3.7M
 * Testcase Example:  '[5,7,7,8,8,10]\n8'
 *
 * Given an array of integers nums sorted in non-decreasing order, find the
 * starting and ending position of a given target value.
 *
 * If target is not found in the array, return [-1, -1].
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * Example 2:
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * Example 3:
 * Input: nums = [], target = 0
 * Output: [-1,-1]
 *
 *
 * Constraints:
 *
 *
 * 0 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * nums is a non-decreasing array.
 * -10^9 <= target <= 10^9
 *
 *
 */

struct Solution {}

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut tuple = vec![-1, -1];
        for i in 0..nums.len() {
            if target == nums[i] {
                if tuple[0] == -1 {
                    tuple[0] = i as i32;
                    tuple[1] = i as i32;
                } else {
                    tuple[1] = i as i32;
                }
            }
        }

        tuple
    }

    pub fn search_range_by_binary_search(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1; 2];
        }
        vec![
            Self::binary_search(&nums, target, true),
            Self::binary_search(&nums, target, false),
        ]
    }

    pub fn binary_search(nums: &Vec<i32>, target: i32, is_start: bool) -> i32 {
        let (mut start, mut end): (i32, i32) = (0, (nums.len() - 1) as i32);
        let mut index: i32 = -1;
        while start <= end {
            let mid = (start + end) / 2;

            match nums[mid as usize].cmp(&target) {
                Ordering::Less => {
                    start = mid + 1;
                }
                Ordering::Greater => {
                    end = mid - 1;
                }
                Ordering::Equal => {
                    if is_start {
                        end = mid - 1;
                    } else {
                        start = mid + 1
                    };
                    index = mid;
                }
            }
        }
        index
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), [3, 4]);
    assert_eq!(
        Solution::search_range_by_binary_search(vec![5, 7, 7, 8, 8, 10], 8),
        [3, 4]
    );
    assert_eq!(
        Solution::search_range_by_binary_search(vec![5, 7, 7, 8, 8, 10], 6),
        [-1, -1]
    );
    assert_eq!(Solution::search_range_by_binary_search(vec![], 6), [-1, -1]);
    assert_eq!(
        Solution::search_range_by_binary_search(vec![2, 2], 2),
        [0, 1]
    );
}
