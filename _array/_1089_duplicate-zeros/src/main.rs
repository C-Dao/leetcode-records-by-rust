/*
 * @lc app=leetcode id=1089 lang=rust
 *
 * [1089] Duplicate Zeros
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut pointer = 0;
        loop {
            if pointer > arr.len() - 1 {
                break;
            }
            if arr[pointer] == 0 {
                arr.insert(pointer, 0);
                arr.pop();
                pointer += 1;
            }
            pointer += 1;
        }
    }
}
// @lc code=end

fn main() {
    let mut arr = vec![1, 5, 2, 0, 6, 8, 0, 6, 0];
    Solution::duplicate_zeros(&mut arr);
    println!("{:?}", arr);
}
