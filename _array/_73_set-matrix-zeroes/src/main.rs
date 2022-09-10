/*
 * @lc app=leetcode id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
 *
 * https://leetcode.com/problems/set-matrix-zeroes/description/
 *
 * algorithms
 * Medium (49.01%)
 * Likes:    8792
 * Dislikes: 542
 * Total Accepted:    804.6K
 * Total Submissions: 1.6M
 * Testcase Example:  '[[1,1,1],[1,0,1],[1,1,1]]'
 *
 * Given an m x n integer matrix matrix, if an element is 0, set its entire row
 * and column to 0's.
 * 
 * You must do it in place.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 * Output: [[1,0,1],[0,0,0],[1,0,1]]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
 * Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * m == matrix.length
 * n == matrix[0].length
 * 1 <= m, n <= 200
 * -2^31 <= matrix[i][j] <= 2^31 - 1
 * 
 * 
 * 
 * Follow up:
 * 
 * 
 * A straightforward solution using O(mn) space is probably a bad idea.
 * A simple improvement uses O(m + n) space, but still not the best
 * solution.
 * Could you devise a constant space solution?
 * 
 * 
 */

struct Solution {}

// @lc code=start
use std::cmp;
use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        let mut row_set: HashSet<i32> = HashSet::new();
        let mut col_set: HashSet<i32> = HashSet::new();

        for i in 0..row_len {
            for j in 0..col_len {
                if matrix[i][j] == 0 {
                    row_set.insert(i as i32);
                    col_set.insert(j as i32);
                }
            }
        }

        for i in 0..row_len {
            for j in 0..col_len {
                if row_set.contains(&(i as i32)) || col_set.contains(&(j as i32)) {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    pub fn set_zeroes_another_edition(matrix: &mut Vec<Vec<i32>>) {
        let (row_len, col_len) = (matrix.len(), matrix[0].len());
        let (mut flag_row_zero, mut flag_col_zero) = (false, false);

        for i in 0..cmp::max(row_len, col_len) {
            if matrix[i][0] == 0 && i < row_len {
                flag_col_zero = true;
            }
            if matrix[0][i] == 0 && i < col_len {
                flag_row_zero = true;
            }
        }

        for i in 1..row_len {
            for j in 1..col_len {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..row_len {
            for j in 1..col_len {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if flag_row_zero {
            for i in 0..col_len {
                matrix[i][0] = 0;
            }
        }
        if flag_col_zero {
            for i in 0..row_len {
                matrix[0][i] = 0;
            }
        }
    }
}
// @lc code=end

fn main() {
    let mut vec_1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let mut vec_2 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut vec_1);
    Solution::set_zeroes_another_edition(&mut vec_2);
    assert_eq!(vec_1, vec_2);
}
