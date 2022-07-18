/*
 * @lc app=leetcode id=1051 lang=rust
 *
 * [1051] Height Checker
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sort_heights = heights.clone();

        sort_heights.sort_by(|a, b| a.cmp(b));

        sort_heights
            .iter()
            .zip(heights.iter())
            .filter(|(a, b)| a != b)
            .count() as i32
    }
}
// @lc code=end

fn main() {
    let heights = vec![1, 1, 4, 2, 1, 3];
    println!("{}", Solution::height_checker(heights));
}
