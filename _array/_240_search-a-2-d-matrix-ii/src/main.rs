/*
* @lc app=leetcode id=240 lang=rust
*
* [240] Search a 2D Matrix II
*
* https://leetcode.com/problems/search-a-2d-matrix-ii/description/
*
* algorithms
* Medium (48.95%)
* Likes:    10429
* Dislikes: 173
* Total Accepted:    789.9K
* Total Submissions: 1.5M
* Testcase Example:  '[[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]\n' +
 '5'
*
* Write an efficient algorithm that searches for a value target in an m x n
* integer matrix matrix. This matrix has the following properties:
*
*
* Integers in each row are sorted in ascending from left to right.
* Integers in each column are sorted in ascending from top to bottom.
*
*
*
* Example 1:
*
*
* Input: matrix =
* [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]],
* target = 5
* Output: true
*
*
* Example 2:
*
*
* Input: matrix =
* [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]],
* target = 20
* Output: false
*
*
*
* Constraints:
*
*
* m == matrix.length
* n == matrix[i].length
* 1 <= n, m <= 300
* -10^9 <= matrix[i][j] <= 10^9
* All the integers in each row are sorted in ascending order.
* All the integers in each column are sorted in ascending order.
* -10^9 <= target <= 10^9
*
*
*/

struct Solution {}

// @lc code=start
use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut row_index, mut col_index): (i32, i32) = (matrix.len() as i32 - 1, 0);

        while row_index >= 0 && col_index < matrix[0].len() as i32 {
            match matrix[row_index as usize][col_index as usize].cmp(&target) {
                Ordering::Greater => {
                    row_index = row_index - 1;
                }
                Ordering::Less => {
                    col_index = col_index + 1;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ),
        true
    );
}
