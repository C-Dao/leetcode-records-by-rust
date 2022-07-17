/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        loop {
            if left == right || nums.len() == 0 {
                break;
            }
            if nums[left] == val {
                nums[left] = nums[right - 1];
                right -= 1;
            } else {
                left += 1;
            }
        }
        right as i32
    }
}
// @lc code=end

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    println!("{},{:?}", Solution::remove_element(&mut nums, 3), nums);
}
