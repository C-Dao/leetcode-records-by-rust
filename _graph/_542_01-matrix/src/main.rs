/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 *
 * https://leetcode.com/problems/01-matrix/description/
 *
 * algorithms
 * Medium (43.98%)
 * Likes:    6621
 * Dislikes: 315
 * Total Accepted:    357.9K
 * Total Submissions: 802.6K
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * Given an m x n binary matrix mat, return the distance of the nearest 0 for
 * each cell.
 *
 * The distance between two adjacent cells is 1.
 *
 *
 * Example 1:
 *
 *
 * Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
 * Output: [[0,0,0],[0,1,0],[0,0,0]]
 *
 *
 * Example 2:
 *
 *
 * Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
 * Output: [[0,0,0],[0,1,0],[1,2,1]]
 *
 *
 *
 * Constraints:
 *
 *
 * m == mat.length
 * n == mat[i].length
 * 1 <= m, n <= 10^4
 * 1 <= m * n <= 10^4
 * mat[i][j] is either 0 or 1.
 * There is at least one 0 in mat.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    /* breadth first search */
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (mat.len(), mat[0].len());
        let mut dists = vec![vec![i32::MAX; cols]; rows];
        let mut queue = VecDeque::new();

        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == 0 {
                    queue.push_back((i, j));
                    dists[i][j] = 0;
                }
            }
        }

        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();
            let dist = dists[row][col];

            for dir in dirs.iter() {
                let next_row = row as i32 + dir.0;
                let next_col = col as i32 + dir.1;

                if next_row >= 0
                    && next_row < rows as i32
                    && next_col >= 0
                    && next_col < cols as i32
                {
                    if dists[next_row as usize][next_col as usize] > dist + 1 {
                        dists[next_row as usize][next_col as usize] = dist + 1;
                        queue.push_back((next_row as usize, next_col as usize));
                    }
                }
            }
        }
        dists
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
    );
}
