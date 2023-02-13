/*
 * @lc app=leetcode id=1470 lang=rust
 *
 * [1470] Shuffle the Array
 *
 * https://leetcode.com/problems/shuffle-the-array/description/
 *
 * algorithms
 * Easy (88.42%)
 * Likes:    4400
 * Dislikes: 239
 * Total Accepted:    476.7K
 * Total Submissions: 534.9K
 * Testcase Example:  '[2,5,1,3,4,7]\n3'
 *
 * Given the array nums consisting of 2n elements in the form
 * [x1,x2,...,xn,y1,y2,...,yn].
 *
 * Return the array in the form [x1,y1,x2,y2,...,xn,yn].
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [2,5,1,3,4,7], n = 3
 * Output: [2,3,5,4,1,7]
 * Explanation: Since x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 then the answer is
 * [2,3,5,4,1,7].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3,4,4,3,2,1], n = 4
 * Output: [1,4,2,3,3,2,4,1]
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,1,2,2], n = 2
 * Output: [1,2,1,2]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 500
 * nums.length == 2n
 * 1 <= nums[i] <= 10^3
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (mut ptr_1, mut ptr_2) = (0, n as usize);
        let mut ans = vec![];
        while ptr_2 < (2 * n) as usize && ptr_1 < n as usize {
            ans.push(nums[ptr_1]);
            ans.push(nums[ptr_2]);
            ptr_1 += 1;
            ptr_2 += 1;
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
        [2, 3, 5, 4, 1, 7]
    );
}
