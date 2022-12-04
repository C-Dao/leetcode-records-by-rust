/*
 * @lc app=leetcode id=713 lang=rust
 *
 * [713] Subarray Product Less Than K
 *
 * https://leetcode.com/problems/subarray-product-less-than-k/description/
 *
 * algorithms
 * Medium (44.33%)
 * Likes:    4842
 * Dislikes: 155
 * Total Accepted:    207.4K
 * Total Submissions: 458.6K
 * Testcase Example:  '[10,5,2,6]\n100'
 *
 * Given an array of integers nums and an integer k, return the number of
 * contiguous subarrays where the product of all the elements in the subarray
 * is strictly less than k.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [10,5,2,6], k = 100
 * Output: 8
 * Explanation: The 8 subarrays that have product less than 100 are:
 * [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
 * Note that [10, 5, 2] is not included as the product of 100 is not strictly
 * less than k.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,2,3], k = 0
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * 1 <= nums[i] <= 1000
 * 0 <= k <= 10^6
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut sp, mut ep, mut product, mut ans) = (0, 0, 1, 0);

        while ep < nums.len() {
            product *= nums[ep];

            while product >= k && sp <= ep {
                product /= nums[sp];
                sp += 1;
            }

            ans += if ep >= sp { ep - sp + 1 } else { 0 };

            ep += 1;
        }

        ans as i32
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        8
    );
}
