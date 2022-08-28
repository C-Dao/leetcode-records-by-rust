/*
 * @lc app=leetcode id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
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
