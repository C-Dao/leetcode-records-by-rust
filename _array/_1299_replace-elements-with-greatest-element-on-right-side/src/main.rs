/*
 * @lc app=leetcode id=1299 lang=rust
 *
 * [1299] Replace Elements with Greatest Element on Right Side
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for i in (0..arr.len()).rev() {
            let cur = arr[i];
            arr[i] = max;
            max = if max > cur { max } else { cur };
        }
        arr
    }
}
// @lc code=end

fn main() {
    let arr = vec![17, 18, 5, 4, 6, 1];
    println!("{:?}", Solution::replace_elements(arr));
}
