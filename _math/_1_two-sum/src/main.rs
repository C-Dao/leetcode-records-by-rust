/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for (index, num) in nums.into_iter().enumerate() {
            let difference = target - num;
            if hash_map.contains_key(&difference) {
                return vec![*hash_map.get(&difference).unwrap(), index as i32];
            } else {
                hash_map.insert(num, index as i32);
            }
        }
        vec![]
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
