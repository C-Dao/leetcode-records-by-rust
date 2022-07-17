/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}
// @lc code=end

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    println!("{},{:?}", Solution::remove_element(&mut nums, 3), nums);
}
