/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 *
 * https://leetcode.com/problems/largest-rectangle-in-histogram/description/
 *
 * algorithms
 * Hard (41.33%)
 * Likes:    12860
 * Dislikes: 181
 * Total Accepted:    615.2K
 * Total Submissions: 1.5M
 * Testcase Example:  '[2,1,5,6,2,3]'
 *
 * Given an array of integers heights representing the histogram's bar height
 * where the width of each bar is 1, return the area of the largest rectangle
 * in the histogram.
 *
 *
 * Example 1:
 *
 *
 * Input: heights = [2,1,5,6,2,3]
 * Output: 10
 * Explanation: The above is a histogram where width of each bar is 1.
 * The largest rectangle is shown in the red area, which has an area = 10
 * units.
 *
 *
 * Example 2:
 *
 *
 * Input: heights = [2,4]
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= heights.length <= 10^5
 * 0 <= heights[i] <= 10^4
 *
 *
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut max_area = 0;

        for i in 0..heights.len() {
            while !stack.is_empty() && heights[*stack.last().unwrap() as usize] >= heights[i] {
                let peek_height = heights[stack.pop().unwrap() as usize];
                let calculated_max_width = i as i32 - *stack.last().unwrap_or(&-1) - 1;
                max_area = i32::max(max_area, calculated_max_width * peek_height);
            }
            stack.push(i as i32);
        }

        while stack.last().is_some() {
            let peek_height = heights[stack.pop().unwrap() as usize];
            let calculated_max_width = heights.len() as i32 - *stack.last().unwrap_or(&-1) - 1;
            max_area = i32::max(max_area, calculated_max_width * peek_height);
        }
        max_area
    }

    pub fn largest_rectangle_area_edition2(heights: Vec<i32>) -> i32 {
        Self::backtrack(&heights, 0, heights.len())
    }

    fn backtrack(heights: &Vec<i32>, start: usize, end: usize) -> i32 {
        if start == end {
            return 0;
        };

        if start + 1 == end {
            return heights[start];
        }

        let mut min_index = start;

        for i in start + 1..end {
            if heights[i] < heights[min_index] {
                min_index = i;
            }
        }

        let cur_area = (end - start) as i32 * heights[min_index];
        let left_area = Self::backtrack(heights, start, min_index);
        let right_area = Self::backtrack(heights, min_index + 1, end);

        i32::max(cur_area, i32::max(right_area, left_area))
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area_edition2(vec![2, 1, 5, 6, 2, 3]), 10);
}
