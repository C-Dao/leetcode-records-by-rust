/*
 * @lc app=leetcode id=57 lang=rust
 *
 * [57] Insert Interval
 *
 * https://leetcode.com/problems/insert-interval/description/
 *
 * algorithms
 * Medium (37.64%)
 * Likes:    7454
 * Dislikes: 511
 * Total Accepted:    691.3K
 * Total Submissions: 1.8M
 * Testcase Example:  '[[1,3],[6,9]]\n[2,5]'
 *
 * You are given an array of non-overlapping intervals intervals where
 * intervals[i] = [starti, endi] represent the start and the end of the i^th
 * interval and intervals is sorted in ascending order by starti. You are also
 * given an interval newInterval = [start, end] that represents the start and
 * end of another interval.
 *
 * Insert newInterval into intervals such that intervals is still sorted in
 * ascending order by starti and intervals still does not have any overlapping
 * intervals (merge overlapping intervals if necessary).
 *
 * Return intervals after the insertion.
 *
 *
 * Example 1:
 *
 *
 * Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
 * Output: [[1,5],[6,9]]
 *
 *
 * Example 2:
 *
 *
 * Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
 * Output: [[1,2],[3,10],[12,16]]
 * Explanation: Because the new interval [4,8] overlaps with
 * [3,5],[6,7],[8,10].
 *
 *
 *
 * Constraints:
 *
 *
 * 0 <= intervals.length <= 10^4
 * intervals[i].length == 2
 * 0 <= starti <= endi <= 10^5
 * intervals is sorted by starti in ascending order.
 * newInterval.length == 2
 * 0 <= start <= end <= 10^5
 *
 *
 */

struct Solution {}
// @lc code=start
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut index = 0;

        while index < intervals.len() && intervals[index][1] < new_interval[0] {
            ans.push(intervals[index].clone());
            index += 1;
        }

        while index < intervals.len() && intervals[index][0] <= new_interval[1] {
            new_interval = vec![
                i32::min(intervals[index][0], new_interval[0]),
                i32::max(intervals[index][1], new_interval[1]),
            ];
            index += 1;
        }

        ans.push(new_interval);

        while index < intervals.len() {
            ans.push(intervals[index].clone());
            index += 1;
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![vec![1, 5], vec![6, 9]],
    );
}
