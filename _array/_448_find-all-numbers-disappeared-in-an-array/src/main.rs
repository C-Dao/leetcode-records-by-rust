/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 *
 * https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/description/
 *
 * algorithms
 * Easy (59.29%)
 * Likes:    6920
 * Dislikes: 396
 * Total Accepted:    604.9K
 * Total Submissions: 1M
 * Testcase Example:  '[4,3,2,7,8,2,3,1]'
 *
 * Given an array nums of n integers where nums[i] is in the range [1, n],
 * return an array of all the integers in the range [1, n] that do not appear
 * in nums.
 * 
 * 
 * Example 1:
 * Input: nums = [4,3,2,7,8,2,3,1]
 * Output: [5,6]
 * Example 2:
 * Input: nums = [1,1]
 * Output: [2]
 * 
 * 
 * Constraints:
 * 
 * 
 * n == nums.length
 * 1 <= n <= 10^5
 * 1 <= nums[i] <= n
 * 
 * 
 * 
 * Follow up: Could you do it without extra space and in O(n) runtime? You may
 * assume the returned list does not count as extra space.
 * 
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let index = i32::abs(nums[i]) as usize - 1;
            if nums[index] > 0 {
                nums[index] *= -1;
            }
        }

        nums.into_iter()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}
// @lc code=end

struct Solution2 {}

// @lc code=start
impl Solution2 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut vec_set = vec![false; nums.len() + 1];
        let mut result: Vec<i32> = vec![];
        for num in nums {
            vec_set[num as usize] = true;
        }

        for i in 1..vec_set.len() {
            if !vec_set[i] {
                result.push(i as i32);
            } else {
                continue;
            }
        }

        result
    }
}
// @lc code=end

fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let nums2 = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", Solution::find_disappeared_numbers(nums));
    println!("{:?}", Solution2::find_disappeared_numbers(nums2));
}
