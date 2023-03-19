/*
 * @lc app=leetcode id=64 lang=rust
 *
 * [64] Minimum Path Sum
 *
 * https://leetcode.com/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (59.87%)
 * Likes:    9531
 * Dislikes: 123
 * Total Accepted:    849.5K
 * Total Submissions: 1.4M
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top
 * left to bottom right, which minimizes the sum of all numbers along its
 * path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 *
 * Example 1:
 *
 *
 * Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
 * Output: 7
 * Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
 *
 *
 * Example 2:
 *
 *
 * Input: grid = [[1,2,3],[4,5,6]]
 * Output: 12
 *
 *
 *
 * Constraints:
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 200
 * 0 <= grid[i][j] <= 100
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_path_sum_recursion(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut min_path_cache = vec![vec![-1; n]; m];
        Self::helper(m - 1, n - 1, &grid, &mut min_path_cache)
    }

    fn helper(i: usize, j: usize, grid: &Vec<Vec<i32>>, min_path_cache: &mut Vec<Vec<i32>>) -> i32 {
        if min_path_cache[i][j] == -1 {
            if i != 0 || j != 0 {
                if i == 0 {
                    min_path_cache[i][j] =
                        grid[i][j] + Self::helper(i, j - 1, grid, min_path_cache);
                } else if j == 0 {
                    min_path_cache[i][j] =
                        grid[i][j] + Self::helper(i - 1, j, grid, min_path_cache);
                } else {
                    min_path_cache[i][j] = grid[i][j]
                        + i32::min(
                            Self::helper(i - 1, j, grid, min_path_cache),
                            Self::helper(i, j - 1, grid, min_path_cache),
                        );
                }
            } else {
                min_path_cache[i][j] = grid[i][j];
            }
        }

        min_path_cache[i][j]
    }

    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 1..m {
            grid[i][0] += grid[i - 1][0];
        }

        for j in 1..n {
            grid[0][j] += grid[0][j - 1];
        }
        for i in 1..m {
            for j in 1..n {
                grid[i][j] += i32::min(grid[i - 1][j], grid[i][j - 1]);
            }
        }

        grid[m - 1][n - 1]
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 5], vec![3, 2, 1]]),
        6
    );
    assert_eq!(
        Solution::min_path_sum_recursion(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(
        Solution::min_path_sum_recursion(vec![vec![1, 2, 5], vec![3, 2, 1]]),
        6
    );
}
