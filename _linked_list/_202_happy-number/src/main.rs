/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 *
 * https://leetcode.com/problems/happy-number/description/
 *
 * algorithms
 * Easy (53.70%)
 * Likes:    6156
 * Dislikes: 782
 * Total Accepted:    886.8K
 * Total Submissions: 1.6M
 * Testcase Example:  '19'
 *
 * Write an algorithm to determine if a number n is happy.
 * 
 * A happy number is a number defined by the following process:
 * 
 * 
 * Starting with any positive integer, replace the number by the sum of the
 * squares of its digits.
 * Repeat the process until the number equals 1 (where it will stay), or it
 * loops endlessly in a cycle which does not include 1.
 * Those numbers for which this process ends in 1 are happy.
 * 
 * 
 * Return true if n is a happy number, and false if not.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: n = 19
 * Output: true
 * Explanation:
 * 1^2 + 9^2 = 82
 * 8^2 + 2^2 = 68
 * 6^2 + 8^2 = 100
 * 1^2 + 0^2 + 0^2 = 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: n = 2
 * Output: false
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= n <= 2^31 - 1
 * 
 * 
 */

use std::collections::HashSet;

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::get_next(n);

        while slow != fast && fast != 1 {
            slow = Self::get_next(slow);
            for _ in 0..2 {
                fast = Self::get_next(fast);
            }
        }
        fast == 1
    }

    pub fn is_happy_by_hash_set(mut n: i32) -> bool {
        let mut set = HashSet::new();
        while n != 1 && set.insert(n) {
            n = Self::get_next(n);
        }
        n == 1
    }

    fn get_next(n: i32) -> i32 {
        let mut m = n;
        let mut total = 0;
        while m > 0 {
            let d = m % 10;
            m = m / 10;
            total += d * d;
        }
        total
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::is_happy(19), true);
    assert_eq!(Solution::is_happy(2), false);
    assert_eq!(Solution::is_happy_by_hash_set(19), true);
    assert_eq!(Solution::is_happy_by_hash_set(2), false);
}
