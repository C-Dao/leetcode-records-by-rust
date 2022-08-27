/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */

struct Solution {}

// @lc code=start
use std::cmp;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|a| a[0]);
        let mut merged_intervals: Vec<Vec<i32>> = vec![];
        for i in 0..intervals.len() {
            let l_val = intervals[i][0];
            let r_val = intervals[i][1];
            if merged_intervals.len() == 0 || merged_intervals.last().unwrap()[1] < l_val {
                merged_intervals.push(vec![l_val, r_val]);
            } else {
                merged_intervals.last_mut().unwrap()[1] =
                    cmp::max(merged_intervals.last().unwrap()[1], r_val);
            }
        }
         merged_intervals
    }
}
// @lc code=end

fn main() {
    assert_eq!(vec![vec![1,6],vec![8,10],vec![15,18]]
        , Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]));
}
