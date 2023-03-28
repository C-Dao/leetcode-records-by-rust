/*
 * @lc app=leetcode id=329 lang=rust
 *
 * [329] Longest Increasing Path in a Matrix
 *
 * https://leetcode.com/problems/longest-increasing-path-in-a-matrix/description/
 *
 * algorithms
 * Hard (51.52%)
 * Likes:    7744
 * Dislikes: 115
 * Total Accepted:    432.4K
 * Total Submissions: 825.6K
 * Testcase Example:  '[[9,9,4],[6,6,8],[2,1,1]]'
 *
 * Given an m x n integers matrix, return the length of the longest increasing
 * path in matrix.
 *
 * From each cell, you can either move in four directions: left, right, up, or
 * down. You may not move diagonally or move outside the boundary (i.e.,
 * wrap-around is not allowed).
 *
 *
 * Example 1:
 *
 *
 * Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
 * Output: 4
 * Explanation: The longest increasing path is [1, 2, 6, 9].
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
 * Output: 4
 * Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally
 * is not allowed.
 *
 *
 * Example 3:
 *
 *
 * Input: matrix = [[1]]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1 <= m, n <= 200
 * 0 <= matrix[i][j] <= 2^31 - 1
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return 0;
        }

        let mut lens = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut longest = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let len = Self::dfs(i, j, &matrix, &mut lens);
                longest = i32::max(longest, len);
            }
        }

        longest
    }

    fn dfs(i: usize, j: usize, matrix: &Vec<Vec<i32>>, lens: &mut Vec<Vec<i32>>) -> i32 {
        if lens[i][j] != 0 {
            return lens[i][j];
        }

        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut len = 1;
        for dir in dirs {
            let row = i as i32 + dir.0;
            let col = j as i32 + dir.1;

            if row >= 0
                && row < matrix.len() as i32
                && col >= 0
                && col < matrix[0].len() as i32
                && matrix[row as usize][col as usize] > matrix[i][j]
            {
                let path = Self::dfs(row as usize, col as usize, matrix, lens);

                len = i32::max(path + 1, len);
            }
        }

        lens[i][j] = len;
        len
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    );
}
