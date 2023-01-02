/*
 * @lc app=leetcode id=373 lang=rust
 *
 * [373] Find K Pairs with Smallest Sums
 *
 * https://leetcode.com/problems/find-k-pairs-with-smallest-sums/description/
 *
 * algorithms
 * Medium (38.57%)
 * Likes:    3770
 * Dislikes: 225
 * Total Accepted:    191K
 * Total Submissions: 498.7K
 * Testcase Example:  '[1,7,11]\n[2,4,6]\n3'
 *
 * You are given two integer arrays nums1 and nums2 sorted in ascending order
 * and an integer k.
 *
 * Define a pair (u, v) which consists of one element from the first array and
 * one element from the second array.
 *
 * Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest
 * sums.
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
 * Output: [[1,2],[1,4],[1,6]]
 * Explanation: The first 3 pairs are returned from the sequence:
 * [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * Output: [[1,1],[1,1]]
 * Explanation: The first 2 pairs are returned from the sequence:
 * [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 *
 *
 * Example 3:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3], k = 3
 * Output: [[1,3],[2,3]]
 * Explanation: All possible pairs are returned from the sequence:
 * [1,3],[2,3]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums1.length, nums2.length <= 10^5
 * -10^9 <= nums1[i], nums2[i] <= 10^9
 * nums1 and nums2 both are sorted in ascending order.
 * 1 <= k <= 10^4
 *
 *
 */

struct Solution {}

// @lc code=start
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Entry {
    num_1: i32,
    num_2: i32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.num_1 + self.num_2).cmp(&(other.num_1 + other.num_2))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Entry {
    fn new(num_1: i32, num_2: i32) -> Self {
        Self { num_1, num_2 }
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap = BinaryHeap::new();

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                max_heap.push(Entry::new(nums1[i], nums2[j]));

                if max_heap.len() > k as usize {
                    max_heap.pop();
                }
            }
        }

        max_heap
            .into_iter()
            .map(|Entry { num_1, num_2 }| vec![num_1, num_2])
            .collect()
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
        [[1, 6], [1, 2], [1, 4]]
    );
}
