/*
 * @lc app=leetcode id=1991 lang=rust
 *
 * [1991] Find the Middle Index in Array
 *
 * https://leetcode.com/problems/find-the-middle-index-in-array/description/
 *
 * algorithms
 * Easy (66.93%)
 * Likes:    620
 * Dislikes: 26
 * Total Accepted:    39.3K
 * Total Submissions: 58.6K
 * Testcase Example:  '[2,3,-1,8,4]'
 *
 * Given a 0-indexed integer array nums, find the leftmost middleIndex (i.e.,
 * the smallest amongst all the possible ones).
 * 
 * A middleIndex is an index where nums[0] + nums[1] + ... +
 * nums[middleIndex-1] == nums[middleIndex+1] + nums[middleIndex+2] + ... +
 * nums[nums.length-1].
 * 
 * If middleIndex == 0, the left side sum is considered to be 0. Similarly, if
 * middleIndex == nums.length - 1, the right side sum is considered to be 0.
 * 
 * Return the leftmost middleIndex that satisfies the condition, or -1 if there
 * is no such index.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [2,3,-1,8,4]
 * Output: 3
 * Explanation: The sum of the numbers before index 3 is: 2 + 3 + -1 = 4
 * The sum of the numbers after index 3 is: 4 = 4
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,-1,4]
 * Output: 2
 * Explanation: The sum of the numbers before index 2 is: 1 + -1 = 0
 * The sum of the numbers after index 2 is: 0
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: nums = [2,5]
 * Output: -1
 * Explanation: There is no valid middleIndex.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 100
 * -1000 <= nums[i] <= 1000
 * 
 * 
 * 
 * Note: This question is the same as 724:
 * https://leetcode.com/problems/find-pivot-index/
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let (mut left_sum, total_sum): (i32, i32) = (0, nums.iter().sum());

        for (i, val) in nums.iter().enumerate() {
            if total_sum - left_sum - val == left_sum {
                return i as i32;
            } else {
                left_sum += val;
            }
        }
        -1
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_middle_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(Solution::find_middle_index(vec![1, 2, 3]), -1);
    assert_eq!(Solution::find_middle_index(vec![2, 1, -1]), 0);
    assert_eq!(Solution::find_middle_index(vec![1]), 0);
}
