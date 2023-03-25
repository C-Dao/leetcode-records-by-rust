/*
 * @lc app=leetcode id=695 lang=rust
 *
 * [695] Max Area of Island
 *
 * https://leetcode.com/problems/max-area-of-island/description/
 *
 * algorithms
 * Medium (70.26%)
 * Likes:    8610
 * Dislikes: 192
 * Total Accepted:    680.3K
 * Total Submissions: 947.8K
 * Testcase Example:  '[[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]'
 *
 * You are given an m x n binary matrix grid. An island is a group of 1's
 * (representing land) connected 4-directionally (horizontal or vertical.) You
 * may assume all four edges of the grid are surrounded by water.
 *
 * The area of an island is the number of cells with a value 1 in the island.
 *
 * Return the maximum area of an island in grid. If there is no island, return
 * 0.
 *
 *
 * Example 1:
 *
 *
 * Input: grid =
 * [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
 * Output: 6
 * Explanation: The answer is not 11, because the island must be connected
 * 4-directionally.
 *
 *
 * Example 2:
 *
 *
 * Input: grid = [[0,0,0,0,0,0,0,0]]
 * Output: 0
 *
 *
 *
 * Constraints:
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1 <= m, n <= 50
 * grid[i][j] is either 0 or 1.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;

impl Solution {

    /* breadth first search */
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col, mut visited) = (
            grid.len(),
            grid[0].len(),
            vec![vec![false; grid[0].len()]; grid.len()],
        );
        let mut ans = 0;
        for i in 0..row {
            for j in 0..col {
                let area = Self::get_area((i, j), &grid, &mut visited);
                ans = i32::max(ans, area);
            }
        }

        ans
    }

    fn get_area(cur: (usize, usize), grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let (row, col) = cur;
        let mut area = 0;

        if grid[row][col] == 0 || grid[row][col] == 1 && visited[row][col] {
            return area;
        };

        queue.push_back(cur);

        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();
            if grid[row][col] == 1 && !visited[row][col] {
                area += 1;
                visited[row][col] = true;

                for dir in dirs.iter() {
                    let next_row = row as i32 + dir.0;
                    let next_col = col as i32 + dir.1;
                    if next_row >= 0
                        && next_row < grid.len() as i32
                        && next_col >= 0
                        && next_col < grid[0].len() as i32
                    {
                        queue.push_back((next_row as usize, next_col as usize));
                    }
                }
            }
        }

        area
    }

    /* deepth first search, iteraction */

    pub fn max_area_of_island_dfs_and_iteraction(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col, mut visited) = (
            grid.len(),
            grid[0].len(),
            vec![vec![false; grid[0].len()]; grid.len()],
        );
        let mut ans = 0;
        for i in 0..row {
            for j in 0..col {
                let area = Self::get_area_dfs_and_iteraction((i, j), &grid, &mut visited);
                ans = i32::max(ans, area);
            }
        }

        ans
    }
    fn get_area_dfs_and_iteraction(
        cur: (usize, usize),
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> i32 {
        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let (row, col) = cur;
        let mut area = 0;

        if grid[row][col] == 0 || grid[row][col] == 1 && visited[row][col] {
            return area;
        };

        stack.push(cur);

        while !stack.is_empty() {
            let (row, col) = stack.pop().unwrap();
            if grid[row][col] == 1 && !visited[row][col] {
                area += 1;
                visited[row][col] = true;

                for dir in dirs.iter() {
                    let next_row = row as i32 + dir.0;
                    let next_col = col as i32 + dir.1;
                    if next_row >= 0
                        && next_row < grid.len() as i32
                        && next_col >= 0
                        && next_col < grid[0].len() as i32
                    {
                        stack.push((next_row as usize, next_col as usize));
                    }
                }
            }
        }

        area
    }

    /* deepth first search, recursion */

    pub fn max_area_of_island_dfs_and_recursion(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col, mut visited) = (
            grid.len(),
            grid[0].len(),
            vec![vec![false; grid[0].len()]; grid.len()],
        );
        let mut ans = 0;
        for i in 0..row {
            for j in 0..col {
                let area = Self::get_area_dfs_and_recursion((i, j), &grid, &mut visited);
                ans = i32::max(ans, area);
            }
        }

        ans
    }
    fn get_area_dfs_and_recursion(
        cur: (usize, usize),
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> i32 {
        let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        let (row, col) = cur;
        let mut area = 0;

        if grid[row][col] == 0 || grid[row][col] == 1 && visited[row][col] {
            return area;
        }

        area += 1;
        visited[row][col] = true;

        for dir in dirs.iter() {
            let next_row = row as i32 + dir.0;
            let next_col = col as i32 + dir.1;
            if next_row >= 0
                && next_row < grid.len() as i32
                && next_col >= 0
                && next_col < grid[0].len() as i32
            {
                area += Self::get_area_dfs_and_recursion(
                    (next_row as usize, next_col as usize),
                    grid,
                    visited,
                );
            }
        }

        area
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::max_area_of_island_dfs_and_iteraction(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::max_area_of_island_dfs_and_recursion(vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]),
        6
    );
}
