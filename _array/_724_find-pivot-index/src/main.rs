/*
 * @lc app=leetcode id=724 lang=rust
 *
 * [724] Find Pivot Index
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let (mut left_sum, total_sum): (i32, i32) = (0, nums.iter().sum());

        for (i,val) in nums.iter().enumerate() {
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
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    assert_eq!(Solution::pivot_index(vec![1]), 0);
}
