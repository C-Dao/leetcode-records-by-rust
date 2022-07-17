/*
 * @lc app=leetcode id=905 lang=rust
 *
 * [905] Sort Array By Parity
 */
struct Solution {}

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut pointer_start = 0;
        let mut pointer_end = nums.len() - 1;

        loop {
            if pointer_start >= pointer_end {
                break;
            }
            if nums[pointer_start] % 2 == 0 {
                pointer_start += 1;
                continue;
            }

            if nums[pointer_end] % 2 == 1 {
                pointer_end -= 1;
                continue;
            }

            if nums[pointer_end] % 2 == 0 && nums[pointer_start] % 2 == 1 {
                nums.swap(pointer_start, pointer_end);
                pointer_start += 1;
                pointer_end -= 1;
                continue;
            }
        }

        nums
    }
}
// @lc code=end

fn main() {
    let nums = vec![3, 1, 2, 4];
    println!("{:?}", Solution::sort_array_by_parity(nums));
}
