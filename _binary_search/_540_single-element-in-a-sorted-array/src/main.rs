/*
 * @lc app=leetcode id=540 lang=rust
 *
 * [540] Single Element in a Sorted Array
 *
 * https://leetcode.com/problems/single-element-in-a-sorted-array/description/
 *
 * algorithms
 * Medium (58.62%)
 * Likes:    6745
 * Dislikes: 117
 * Total Accepted:    352.5K
 * Total Submissions: 603.1K
 * Testcase Example:  '[1,1,2,3,3,4,4,8,8]'
 *
 * You are given a sorted array consisting of only integers where every element
 * appears exactly twice, except for one element which appears exactly once.
 *
 * Return the single element that appears only once.
 *
 * Your solution must run in O(log n) time and O(1) space.
 *
 *
 * Example 1:
 * Input: nums = [1,1,2,3,3,4,4,8,8]
 * Output: 2
 * Example 2:
 * Input: nums = [3,3,7,7,10,11,11]
 * Output: 10
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * 0 <= nums[i] <= 10^5
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() / 2);

        while left <= right {
            let mid = (left + right) / 2;
            let index = mid * 2;

            if index < nums.len() - 1
                && nums[index] != nums[index + 1]
                && (index == 0 || nums[index - 2] == nums[index - 1])
            {
                return nums[index];
            };

            if index < nums.len() - 1 && nums[index] != nums[index + 1] {
                right = mid - 1;
            } else {
                left = mid + 1;
            };
        }

        nums[nums.len() - 1]
    }

    pub fn single_non_duplicate_edition_2(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
       
        for num in nums {
            ans ^= num;
        }
       
        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );
    assert_eq!(
        Solution::single_non_duplicate_edition_2(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );

}
