/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let middle = start + (end - start) / 2;
            if nums[middle] < target {
                start = middle + 1;
            } else {
                end = middle;
            }
        }
        start as i32
    }
}
// @lc code=end

fn main() {
    println!("{:?}", Solution::search_insert(vec![1, 2, 3, 4, 5], 3));
    println!("{:?}", Solution::search_insert(vec![1, 3, 5, 6], 0));
}
