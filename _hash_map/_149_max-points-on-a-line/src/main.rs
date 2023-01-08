/*
 * @lc app=leetcode id=149 lang=rust
 *
 * [149] Max Points on a Line
 *
 * https://leetcode.com/problems/max-points-on-a-line/description/
 *
 * algorithms
 * Hard (21.01%)
 * Likes:    2906
 * Dislikes: 362
 * Total Accepted:    301.7K
 * Total Submissions: 1.3M
 * Testcase Example:  '[[1,1],[2,2],[3,3]]'
 *
 * Given an array of points where points[i] = [xi, yi] represents a point on
 * the X-Y plane, return the maximum number of points that lie on the same
 * straight line.
 *
 *
 * Example 1:
 *
 *
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= points.length <= 300
 * points[i].length == 2
 * -10^4 <= xi, yi <= 10^4
 * All the points are unique.
 *
 *
 */
struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 {
            return points.len() as i32;
        };

        let mut ans = i32::MIN;

        for i in 0..points.len() {
            let mut angels_map: HashMap<u64, i32> = HashMap::new();

            for j in 0..points.len() {
                if j != i {
                    let angle_bits = ((points[i][1] - points[j][1]) as f64)
                        .atan2((points[i][0] - points[j][0]) as f64)
                        .to_bits();

                        angels_map.insert(angle_bits, angels_map.get(&angle_bits).unwrap_or(&1) + 1);
                };
            }

            for count in angels_map.values() {
                ans = i32::max(ans, *count);
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::max_points(vec![vec![5151, 5150], vec![0, 0], vec![5152, 5151]]),
        2
    );
}
