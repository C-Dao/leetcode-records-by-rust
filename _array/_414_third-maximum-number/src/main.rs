/*
 * @lc app=leetcode id=414 lang=rust
 *
 * [414] Third Maximum Number
 *
 * https://leetcode.com/problems/third-maximum-number/description/
 *
 * algorithms
 * Easy (32.08%)
 * Likes:    1823
 * Dislikes: 2398
 * Total Accepted:    347.6K
 * Total Submissions: 1.1M
 * Testcase Example:  '[3,2,1]'
 *
 * Given an integer array nums, return the third distinct maximum number in
 * this array. If the third maximum does not exist, return the maximum
 * number.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [3,2,1]
 * Output: 1
 * Explanation:
 * The first distinct maximum is 3.
 * The second distinct maximum is 2.
 * The third distinct maximum is 1.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,2]
 * Output: 2
 * Explanation:
 * The first distinct maximum is 2.
 * The second distinct maximum is 1.
 * The third distinct maximum does not exist, so the maximum (2) is returned
 * instead.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: nums = [2,2,3,1]
 * Output: 1
 * Explanation:
 * The first distinct maximum is 3.
 * The second distinct maximum is 2 (both 2's are counted together since they
 * have the same value).
 * The third distinct maximum is 1.
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= nums.length <= 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 * 
 * 
 * 
 * Follow up: Can you find an O(n) solution?
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut first = i64::MIN;
        let mut second = i64::MIN;
        let mut third = i64::MIN;
        for num in nums {
            let num = num as i64;
            if num > first {
                third = second;
                second = first;
                first = num;
            } else if num != first && num > second {
                third = second;
                second = num;
            } else if num != first && num != second && num > third {
                third = num;
            }
        }
        if third != i64::MIN {
            third as i32
        } else {
            first as i32
        }
    }
}
// @lc code=end

fn main() {
    let nums = vec![1, 2, 2, 5, 3, 5];
    println!("{}", Solution::third_max(nums));
}
