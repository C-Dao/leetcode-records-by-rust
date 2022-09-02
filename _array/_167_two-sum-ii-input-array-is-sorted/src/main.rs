/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input Array Is Sorted
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            if target == numbers[left] + numbers[right] {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if target < numbers[left] + numbers[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        vec![]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
