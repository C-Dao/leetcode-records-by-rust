/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let index = i32::abs(nums[i]) as usize - 1;
            if nums[index] > 0 {
                nums[index] *= -1;
            }
        }

        nums.into_iter()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .map(|(i, _)| i as i32 + 1)
            .collect()
    }
}
// @lc code=end

struct Solution2 {}

// @lc code=start
impl Solution2 {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut vec_set = vec![false; nums.len() + 1];
        let mut result: Vec<i32> = vec![];
        for num in nums {
            vec_set[num as usize] = true;
        }

        for i in 1..vec_set.len() {
            if !vec_set[i] {
                result.push(i as i32);
            } else {
                continue;
            }
        }

        result
    }
}
// @lc code=end

fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let nums2 = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("{:?}", Solution::find_disappeared_numbers(nums));
    println!("{:?}", Solution2::find_disappeared_numbers(nums2));
}
