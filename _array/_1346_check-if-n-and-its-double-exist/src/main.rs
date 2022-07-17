/*
 * @lc app=leetcode id=1346 lang=rust
 *
 * [1346] Check If N and Its Double Exist
 */

use std::collections::HashSet;
struct Solution {}

// @lc code=start
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut mask = HashSet::new();
        let mut result = false;
        for i in 0..arr.len() {
            if mask.contains(&(arr[i] * 2)) || (arr[i] % 2 == 0 && mask.contains(&(arr[i] / 2))) {
                result = true;
                break;
            }
            mask.insert(arr[i]);
        }
        result
    }
}
// @lc code=end

fn main() {
    let arr = vec![10, 2, 5, 3];
    assert_eq!(Solution::check_if_exist(arr), true);
}
