/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 *
 * https://leetcode.com/problems/search-insert-position/description/
 *
 * algorithms
 * Easy (42.24%)
 * Likes:    9245
 * Dislikes: 456
 * Total Accepted:    1.7M
 * Total Submissions: 3.9M
 * Testcase Example:  '[1,3,5,6]\n5'
 *
 * Given a sorted array of distinct integers and a target value, return the
 * index if the target is found. If not, return the index where it would be if
 * it were inserted in order.
 * 
 * You must write an algorithm with O(log n) runtime complexity.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums contains distinct values sorted in ascending order.
 * -10^4 <= target <= 10^4
 * 
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let middle = start + (end - start) / 2;
            if nums[middle] < target {
                start = middle + 1;
            } else {
                end = middle;
            }
        }
        start as i32
    }
}
// @lc code=end

fn main() {
    println!("{:?}", Solution::search_insert(vec![1, 2, 3, 4, 5], 3));
    println!("{:?}", Solution::search_insert(vec![1, 3, 5, 6], 0));
}
