/*
 * @lc app=leetcode id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
 *
 * https://leetcode.com/problems/squares-of-a-sorted-array/description/
 *
 * algorithms
 * Easy (71.73%)
 * Likes:    5809
 * Dislikes: 158
 * Total Accepted:    1.1M
 * Total Submissions: 1.5M
 * Testcase Example:  '[-4,-1,0,3,10]'
 *
 * Given an integer array nums sorted in non-decreasing order, return an array
 * of the squares of each number sorted in non-decreasing order.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [-4,-1,0,3,10]
 * Output: [0,1,9,16,100]
 * Explanation: After squaring, the array becomes [16,1,0,9,100].
 * After sorting, it becomes [0,1,9,16,100].
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [-7,-3,2,3,11]
 * Output: [4,9,9,49,121]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums is sorted in non-decreasing order.
 * 
 * 
 * 
 * Follow up: Squaring each element and sorting the new array is very trivial,
 * could you find an O(n) solution using a different approach?
 */


struct Solution {}

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut start, mut end, mut point) = (0, nums.len() - 1, nums.len() - 1);
        let mut result = vec![0; nums.len()];
        loop {
            if nums[start] + nums[end] > 0 {
                result[point] = nums[end] * nums[end];
                if point == 0 {
                    break;
                }
                point -= 1;
                if end == 0 {
                    break;
                }
                end -= 1;
            } else {
                result[point] = nums[start] * nums[start];
                if point == 0 {
                    break;
                }
                point -= 1;
                if start == nums.len() - 1 {
                    break;
                }
                start += 1;
            }
        }
        result
    }
}
// @lc code=end

fn main() {
    Solution::sorted_squares(vec![-4, -1, 0, 3, 10]);
}
