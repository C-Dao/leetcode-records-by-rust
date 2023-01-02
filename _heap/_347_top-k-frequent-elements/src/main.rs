/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 *
 * https://leetcode.com/problems/top-k-frequent-elements/description/
 *
 * algorithms
 * Medium (65.00%)
 * Likes:    12195
 * Dislikes: 448
 * Total Accepted:    1.3M
 * Total Submissions: 2M
 * Testcase Example:  '[1,1,1,2,2,3]\n2'
 *
 * Given an integer array nums and an integer k, return the k most frequent
 * elements. You may return the answer in any order.
 *
 *
 * Example 1:
 * Input: nums = [1,1,1,2,2,3], k = 2
 * Output: [1,2]
 * Example 2:
 * Input: nums = [1], k = 1
 * Output: [1]
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^5
 * -10^4 <= nums[i] <= 10^4
 * k is in the range [1, the number of unique elements in the array].
 * It is guaranteed that the answer is unique.
 *
 *
 *
 * Follow up: Your algorithm's time complexity must be better than O(n log n),
 * where n is the array's size.
 *
 */

struct Solution {}

// @lc code=start
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq)]
struct Entry {
    num: i32,
    count: i32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Entry {
    fn new(num: i32, count: i32) -> Self {
        Self { num, count }
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums_map = HashMap::new();

        for i in 0..nums.len() {
            nums_map.insert(nums[i], *nums_map.get(&nums[i]).unwrap_or(&0) + 1);
        }

        let mut min_heap = BinaryHeap::new();

        for (num, count) in nums_map.into_iter() {
            min_heap.push(Reverse(Entry::new(num, count)));

            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }

        min_heap
            .into_iter()
            .map(|Reverse(Entry { num, count: _ })| num)
            .collect()
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), [2, 1]);
}
