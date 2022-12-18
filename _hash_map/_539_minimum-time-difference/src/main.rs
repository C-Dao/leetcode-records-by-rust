/*
 * @lc app=leetcode id=539 lang=rust
 *
 * [539] Minimum Time Difference
 *
 * https://leetcode.com/problems/minimum-time-difference/description/
 *
 * algorithms
 * Medium (55.85%)
 * Likes:    1375
 * Dislikes: 234
 * Total Accepted:    116.3K
 * Total Submissions: 206.7K
 * Testcase Example:  '["23:59","00:00"]'
 *
 * Given a list of 24-hour clock time points in "HH:MM" format, return the
 * minimum minutes difference between any two time-points in the list.
 *
 * Example 1:
 * Input: timePoints = ["23:59","00:00"]
 * Output: 1
 * Example 2:
 * Input: timePoints = ["00:00","23:59","00:00"]
 * Output: 0
 *
 *
 * Constraints:
 *
 *
 * 2 <= timePoints.length <= 2 * 10^4
 * timePoints[i] is in the format "HH:MM".
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minute_flags = [false; 1440];

        for time_point in time_points {
            let time: Vec<usize> = time_point
                .split(':')
                .map(|str| str.parse().unwrap())
                .collect();
            let minutes = time[0] * 60 + time[1];
            if minute_flags[minutes] {
                return 0;
            }
            minute_flags[minutes] = true;
        }

        let (
            mut min_time_span,
            mut prev_valid_minute,
            mut first_valid_minute,
            mut last_valid_minute,
        ): (i32, i32, i32, i32) = (minute_flags.len() as i32, -1, minute_flags.len() as i32, -1);

        for i in 0..minute_flags.len() {
            if minute_flags[i] {
                if prev_valid_minute > -1 {
                    min_time_span = i32::min((i as i32) - prev_valid_minute, min_time_span);
                }

                prev_valid_minute = i as i32;
                first_valid_minute = i32::min(i as i32, first_valid_minute);
                last_valid_minute = i32::max(i as i32, last_valid_minute);
            }
        }

        min_time_span = i32::min(
            first_valid_minute + (minute_flags.len() as i32) - last_valid_minute,
            min_time_span,
        );

        min_time_span
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]),
        1
    );
}
