/*
 * @lc app=leetcode id=414 lang=rust
 *
 * [414] Third Maximum Number
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut first = i64::MIN;
        let mut second = i64::MIN;
        let mut third = i64::MIN;
        for num in nums {
            let num = num as i64;
            if num > first {
                third = second;
                second = first;
                first = num;
            } else if num != first && num > second {
                third = second;
                second = num;
            } else if num != first && num != second && num > third {
                third = num;
            }
        }
        if third != i64::MIN {
            third as i32
        } else {
            first as i32
        }
    }
}
// @lc code=end

fn main() {
    let nums = vec![1, 2, 2, 5, 3, 5];
    println!("{}", Solution::third_max(nums));
}
