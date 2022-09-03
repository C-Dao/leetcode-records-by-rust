/*
 * @lc app=leetcode id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 */

struct Solution {}

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut queue, mut ans): (VecDeque<usize>, Vec<i32>) = (VecDeque::new(), vec![]);
        for end in 0..nums.len() {
            while !queue.is_empty() && nums[end as usize] >= nums[*queue.back().unwrap() as usize] {
                queue.pop_back();
            }
            queue.push_back(end as usize);

            if *queue.front().unwrap() + k as usize <= end {
                queue.pop_front();
            }

            if end as i32 >= k - 1 {
                ans.push(nums[*queue.front().unwrap() as usize]);
            }
        }
        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}
