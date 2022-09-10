/*
 * @lc app=leetcode id=905 lang=rust
 *
 * [905] Sort Array By Parity
 *
 * https://leetcode.com/problems/sort-array-by-parity/description/
 *
 * algorithms
 * Easy (75.72%)
 * Likes:    3678
 * Dislikes: 123
 * Total Accepted:    524.6K
 * Total Submissions: 692.9K
 * Testcase Example:  '[3,1,2,4]'
 *
 * Given an integer array nums, move all the even integers at the beginning of
 * the array followed by all the odd integers.
 * 
 * Return any array that satisfies this condition.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [3,1,2,4]
 * Output: [2,4,3,1]
 * Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be
 * accepted.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [0]
 * Output: [0]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 5000
 * 0 <= nums[i] <= 5000
 * 
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut pointer_start = 0;
        let mut pointer_end = nums.len() - 1;

        loop {
            if pointer_start >= pointer_end {
                break;
            }
            if nums[pointer_start] % 2 == 0 {
                pointer_start += 1;
                continue;
            }

            if nums[pointer_end] % 2 == 1 {
                pointer_end -= 1;
                continue;
            }

            if nums[pointer_end] % 2 == 0 && nums[pointer_start] % 2 == 1 {
                nums.swap(pointer_start, pointer_end);
                pointer_start += 1;
                pointer_end -= 1;
                continue;
            }
        }

        nums
    }
}
// @lc code=end

fn main() {
    let nums = vec![3, 1, 2, 4];
    println!("{:?}", Solution::sort_array_by_parity(nums));
}
