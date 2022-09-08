/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p0, mut p2, mut i) = (0, nums.len() - 1, 0);
        while i <= p2 && p2 != 0 {
            while p2 != 0 && i <= p2 && nums[i] == 2 {
                nums.swap(i, p2);
                p2 -= 1;
            }
            while i >= p0 && nums[i] == 0 {
                nums.swap(i, p0);
                p0 += 1;
            }

            i += 1;
        }
    }
}
// @lc code=end

fn main() {
    let mut nums = vec![2, 2];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, vec![2, 2]);
}
