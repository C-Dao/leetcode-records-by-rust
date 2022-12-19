/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 *
 * https://leetcode.com/problems/daily-temperatures/description/
 *
 * algorithms
 * Medium (67.48%)
 * Likes:    9650
 * Dislikes: 220
 * Total Accepted:    543.9K
 * Total Submissions: 817.2K
 * Testcase Example:  '[73,74,75,71,69,72,76,73]'
 *
 * Given an array of integers temperatures represents the daily temperatures,
 * return an array answer such that answer[i] is the number of days you have to
 * wait after the i^th day to get a warmer temperature. If there is no future
 * day for which this is possible, keep answer[i] == 0 instead.
 *
 *
 * Example 1:
 * Input: temperatures = [73,74,75,71,69,72,76,73]
 * Output: [1,1,4,2,1,1,0,0]
 * Example 2:
 * Input: temperatures = [30,40,50,60]
 * Output: [1,1,1,0]
 * Example 3:
 * Input: temperatures = [30,60,90]
 * Output: [1,1,0]
 *
 *
 * Constraints:
 *
 *
 * 1 <= temperatures.length <= 10^5
 * 30 <= temperatures[i] <= 100
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans: Vec<i32> = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
                let idx = stack.pop().unwrap();
                ans[idx] = (i - idx) as i32;
            }

            stack.push(i);
        }
        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
}
