/*
 * @lc app=leetcode id=735 lang=rust
 *
 * [735] Asteroid Collision
 *
 * https://leetcode.com/problems/asteroid-collision/description/
 *
 * algorithms
 * Medium (44.33%)
 * Likes:    4443
 * Dislikes: 385
 * Total Accepted:    246.3K
 * Total Submissions: 555K
 * Testcase Example:  '[5,10,-5]'
 *
 * We are given an array asteroids of integers representing asteroids in a
 * row.
 *
 * For each asteroid, the absolute value represents its size, and the sign
 * represents its direction (positive meaning right, negative meaning left).
 * Each asteroid moves at the same speed.
 *
 * Find out the state of the asteroids after all collisions. If two asteroids
 * meet, the smaller one will explode. If both are the same size, both will
 * explode. Two asteroids moving in the same direction will never meet.
 *
 *
 * Example 1:
 *
 *
 * Input: asteroids = [5,10,-5]
 * Output: [5,10]
 * Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never
 * collide.
 *
 *
 * Example 2:
 *
 *
 * Input: asteroids = [8,-8]
 * Output: []
 * Explanation: The 8 and -8 collide exploding each other.
 *
 *
 * Example 3:
 *
 *
 * Input: asteroids = [10,2,-5]
 * Output: [10]
 * Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide
 * resulting in 10.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= asteroids.length <= 10^4
 * -1000 <= asteroids[i] <= 1000
 * asteroids[i] != 0
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = vec![];

        for asteroid in asteroids {
            while !stack.is_empty()
                && *stack.last().unwrap() > 0
                && asteroid < 0
                && i32::abs(asteroid) > *stack.last().unwrap()
            {
                stack.pop();
            }

            if !stack.is_empty()
                && *stack.last().unwrap() > 0
                && asteroid < 0
                && i32::abs(asteroid) == *stack.last().unwrap()
            {
                stack.pop();
            } else if stack.is_empty() || *stack.last().unwrap() < 0 || asteroid > 0 {
                stack.push(asteroid);
            }
        }

        stack
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
}
