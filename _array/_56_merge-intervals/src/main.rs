/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 *
 * https://leetcode.com/problems/merge-intervals/description/
 *
 * algorithms
 * Medium (45.31%)
 * Likes:    15688
 * Dislikes: 562
 * Total Accepted:    1.6M
 * Total Submissions: 3.5M
 * Testcase Example:  '[[1,3],[2,6],[8,10],[15,18]]'
 *
 * Given an array of intervals where intervals[i] = [starti, endi], merge all
 * overlapping intervals, and return an array of the non-overlapping intervals
 * that cover all the intervals in the input.
 *
 *
 * Example 1:
 *
 *
 * Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
 * Output: [[1,6],[8,10],[15,18]]
 * Explanation: Since intervals [1,3] and [2,6] overlap, merge them into
 * [1,6].
 *
 *
 * Example 2:
 *
 *
 * Input: intervals = [[1,4],[4,5]]
 * Output: [[1,5]]
 * Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= intervals.length <= 10^4
 * intervals[i].length == 2
 * 0 <= starti <= endi <= 10^4
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|a| a[0]);

        let mut merged_intervals: Vec<Vec<i32>> = vec![];

        for i in 0..intervals.len() {
            if merged_intervals.len() == 0 || merged_intervals.last().unwrap()[1] < intervals[i][0]
            {
                merged_intervals.push(intervals[i].clone());
            } else {
                merged_intervals.last_mut().unwrap()[1] =
                    i32::max(merged_intervals.last().unwrap()[1], intervals[i][1]);
            }
        }
        merged_intervals
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
}
