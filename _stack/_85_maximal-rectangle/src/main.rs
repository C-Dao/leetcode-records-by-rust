/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 *
 * https://leetcode.com/problems/maximal-rectangle/description/
 *
 * algorithms
 * Hard (43.30%)
 * Likes:    7998
 * Dislikes: 128
 * Total Accepted:    328.6K
 * Total Submissions: 741.7K
 * Testcase Example:  '[["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]'
 *
 * Given a rows x cols binary matrix filled with 0's and 1's, find the largest
 * rectangle containing only 1's and return its area.
 *
 *
 * Example 1:
 *
 *
 * Input: matrix =
 * [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * Output: 6
 * Explanation: The maximal rectangle is shown in the above picture.
 *
 *
 * Example 2:
 *
 *
 * Input: matrix = [["0"]]
 * Output: 0
 *
 *
 * Example 3:
 *
 *
 * Input: matrix = [["1"]]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * rows == matrix.length
 * cols == matrix[i].length
 * 1 <= row, cols <= 200
 * matrix[i][j] is '0' or '1'.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_area = 0;
        let mut heights: Vec<i32> = vec![0; matrix[0].len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                heights[j] = if matrix[i][j] == '0' {
                    0
                } else {
                    heights[j] + 1
                };
            }

            max_area = i32::max(max_area, Self::largest_rectangle_area(&heights));
        }

        max_area
    }

    fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut max_area = 0;

        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] > heights[i] {
                let peek_height = heights[stack.pop().unwrap() as usize];
                let calculated_max_width = i as i32 - *stack.last().unwrap_or(&-1) - 1;
                max_area = i32::max(max_area, calculated_max_width * peek_height);
            }

            stack.push(i as i32);
        }

        while !stack.is_empty() {
            let peek_height = heights[stack.pop().unwrap() as usize];
            let calculated_max_width = heights.len() as i32 - *stack.last().unwrap_or(&-1) - 1;
            max_area = i32::max(max_area, calculated_max_width * peek_height);
        }

        max_area
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        6
    );
}
