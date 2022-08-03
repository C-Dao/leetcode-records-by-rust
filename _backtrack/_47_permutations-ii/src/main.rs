/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut mask = vec![false; nums.len()];
        let mut scratch = vec![0; nums.len()];

        Self::backtrack(&nums, &mut mask, &mut scratch, &mut result, 0);
        result
    }

    fn backtrack(
        nums: &Vec<i32>,
        mask: &mut Vec<bool>,
        scratch: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        k: usize,
    ) {
        if k == nums.len() {
            result.push(scratch.clone());
        } else {
            let mut used = Vec::with_capacity(nums.len());
            for i in 0..nums.len() {
                if !mask[i] && !used.iter().any(|x| *x == nums[i]) {
                    scratch[k] = nums[i];
                    mask[i] = true;
                    Self::backtrack(nums, mask, scratch, result, k + 1);
                    used.push(nums[i]);
                    mask[i] = false;
                }
            }
        }
    }
}
// @lc code=end

fn main() {
    let nums = vec![1, 1, 2];
    println!("{:?}", Solution::permute_unique(nums));
}
