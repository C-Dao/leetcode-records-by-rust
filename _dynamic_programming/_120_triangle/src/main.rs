/*
 * @lc app=leetcode id=120 lang=rust
 *
 * [120] Triangle
 *
 * https://leetcode.com/problems/triangle/description/
 *
 * algorithms
 * Medium (52.99%)
 * Likes:    7663
 * Dislikes: 461
 * Total Accepted:    566.8K
 * Total Submissions: 1M
 * Testcase Example:  '[[2],[3,4],[6,5,7],[4,1,8,3]]'
 *
 * Given a triangle array, return the minimum path sum from top to bottom.
 *
 * For each step, you may move to an adjacent number of the row below. More
 * formally, if you are on index i on the current row, you may move to either
 * index i or index i + 1 on the next row.
 *
 *
 * Example 1:
 *
 *
 * Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 * Output: 11
 * Explanation: The triangle looks like:
 * ⁠  2
 * ⁠ 3 4
 * ⁠6 5 7
 * 4 1 8 3
 * The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined
 * above).
 *
 *
 * Example 2:
 *
 *
 * Input: triangle = [[-10]]
 * Output: -10
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= triangle.length <= 200
 * triangle[0].length == 1
 * triangle[i].length == triangle[i - 1].length + 1
 * -10^4 <= triangle[i][j] <= 10^4
 *
 *
 *
 * Follow up: Could you do this using only O(n) extra space, where n is the
 * total number of rows in the triangle?
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn minimum_total_recursion(mut triangle: Vec<Vec<i32>>) -> i32 {
        let mut minimum_total_cache = vec![];
        Self::helper(0, 0, &mut minimum_total_cache, &mut triangle)
    }

    fn helper(
        i: usize,
        j: usize,
        minimum_total_cache: &mut Vec<Vec<bool>>,
        triangle: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let m = triangle.len();
        let n = triangle[i].len();

        if minimum_total_cache.len() < i + 1 {
            minimum_total_cache.push(vec![false; n])
        };

        if i < m && j < n {
            if i < m - 1 && minimum_total_cache[i][j] == false {
                triangle[i][j] += i32::min(
                    Self::helper(i + 1, j, minimum_total_cache, triangle),
                    Self::helper(i + 1, j + 1, minimum_total_cache, triangle),
                );
                minimum_total_cache[i][j] = true;
            }
            triangle[i][j]
        } else {
            0
        }
    }

    pub fn minimum_total_iteraction(mut triangle: Vec<Vec<i32>>) -> i32 {
        let m = triangle.len();

        for i in 1..m {
            let n = triangle[i].len();
            for j in 0..n {
                if j == 0 {
                    triangle[i][j] += triangle[i - 1][j];
                } else if j == n - 1 {
                    triangle[i][j] += triangle[i - 1][j - 1];
                } else {
                    triangle[i][j] += i32::min(triangle[i - 1][j], triangle[i - 1][j - 1]);
                }
            }
        }

        let mut min = i32::MAX;
        let triangle_last = triangle.last().unwrap();
        for i in 0..triangle_last.len() {
            min = i32::min(min, triangle_last[i]);
        }
        min
    }

    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let m = triangle.len();

        for i in (0..m - 1).rev() {
            let n = triangle[i].len();
            for j in 0..n {
                triangle[i][j] += i32::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
            }
        }

        triangle[0][0]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );

    assert_eq!(
        Solution::minimum_total(vec![
            vec![-7],
            vec![-2, 1],
            vec![-5, -5, 9],
            vec![-4, -5, 4, 4],
            vec![-6, -6, 2, -1, -5],
            vec![3, 7, 8, -3, 7, -9],
            vec![-9, -1, -9, 6, 9, 0, 7],
            vec![-7, 0, -6, -8, 7, 1, -4, 9],
            vec![-3, 2, -6, -9, -7, -6, -9, 4, 0],
            vec![-8, -6, -3, -9, -2, -6, 7, -5, 0, 7],
            vec![-9, -1, -2, 4, -2, 4, 4, -1, 2, -5, 5],
            vec![1, 1, -6, 1, -2, -4, 4, -2, 6, -6, 0, 6],
            vec![-3, -3, -6, -2, -6, -2, 7, -9, -5, -7, -5, 5, 1]
        ]),
        -63
    );

    assert_eq!(
        Solution::minimum_total_recursion(vec![
            vec![2],
            vec![3, 4],
            vec![6, 5, 7],
            vec![4, 1, 8, 3]
        ]),
        11
    );
    assert_eq!(
        Solution::minimum_total_iteraction(vec![
            vec![2],
            vec![3, 4],
            vec![6, 5, 7],
            vec![4, 1, 8, 3]
        ]),
        11
    );
}
