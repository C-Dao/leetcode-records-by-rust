/*
 * @lc app=leetcode id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
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
