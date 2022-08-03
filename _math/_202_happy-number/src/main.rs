/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */
use std::collections::HashSet;

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::get_next(n);

        while slow != fast && fast != 1 {
            slow = Self::get_next(slow);
            for i in 0..2 {
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
    println!("Hello, world!");
}
