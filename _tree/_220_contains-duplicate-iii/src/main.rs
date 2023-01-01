/*
 * @lc app=leetcode id=220 lang=rust
 *
 * [220] Contains Duplicate III
 *
 * https://leetcode.com/problems/contains-duplicate-iii/description/
 *
 * algorithms
 * Medium (21.71%)
 * Likes:    371
 * Dislikes: 8
 * Total Accepted:    222.6K
 * Total Submissions: 1M
 * Testcase Example:  '[1,2,3,1]\n3\n0'
 *
 * You are given an integer array nums and two integers indexDiff and
 * valueDiff.
 *
 * Find a pair of indices (i, j) such that:
 *
 *
 * i != j,
 * abs(i - j) <= indexDiff.
 * abs(nums[i] - nums[j]) <= valueDiff, and
 *
 *
 * Return true if such pair exists or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,2,3,1], indexDiff = 3, valueDiff = 0
 * Output: true
 * Explanation: We can choose (i, j) = (0, 3).
 * We satisfy the three conditions:
 * i != j --> 0 != 3
 * abs(i - j) <= indexDiff --> abs(0 - 3) <= 3
 * abs(nums[i] - nums[j]) <= valueDiff --> abs(1 - 1) <= 0
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,5,9,1,5,9], indexDiff = 2, valueDiff = 3
 * Output: false
 * Explanation: After trying all the possible pairs (i, j), we cannot satisfy
 * the three conditions, so we return false.
 *
 *
 *
 * Constraints:
 *
 *
 * 2 <= nums.length <= 10^5
 * -10^9 <= nums[i] <= 10^9
 * 1 <= indexDiff <= nums.length
 * 0 <= valueDiff <= 10^9
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut tree_set: BTreeSet<i32> = BTreeSet::new();

        for i in 0..nums.len() {
            // btree_set.floor, 获取键小于或等于给定值的最大值，如果没有返回 None
            let lower = tree_set.range(..=nums[i]).next_back();

            if lower.is_some() && *lower.unwrap() >= nums[i] - value_diff {
                return true;
            }

            // btree_set.ceiling, 获取键大于或等于给定值的最小值，如果没有返回 None
            let upper = tree_set.range(nums[i]..).next();

            if upper.is_some() && *upper.unwrap() <= nums[i] + value_diff {
                return true;
            }

            tree_set.insert(nums[i]);

            if i >= index_diff as usize {
                tree_set.remove(&nums[i - index_diff as usize]);
            };
        }

        false
    }

    pub fn contains_nearby_almost_duplicate_edition_2(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut buckets: HashMap<i32, i32> = HashMap::new();
        let bucket_size = value_diff + 1;
        for i in 0..nums.len() {
            let id = Self::get_bucket_id(nums[i], bucket_size);
            if buckets.contains_key(&id) {
                return true;
            }

            if buckets.contains_key(&(id - 1))
                && buckets.get(&(id - 1)).unwrap() + value_diff >= nums[i]
            {
                return true;
            }
            if buckets.contains_key(&(id + 1))
                && buckets.get(&(id + 1)).unwrap() - value_diff <= nums[i]
            {
                return true;
            }

            buckets.insert(id, nums[i]);

            if i >= index_diff as usize {
                buckets.remove(&Self::get_bucket_id(
                    nums[i - index_diff as usize],
                    bucket_size,
                ));
            };
        }
        false
    }

    fn get_bucket_id(num: i32, bucket_size: i32) -> i32 {
        if num >= 0 {
            num / bucket_size
        } else {
            (num + 1) / bucket_size - 1
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
        false
    );
    assert_eq!(
        Solution::contains_nearby_almost_duplicate_edition_2(vec![1, 5, 9, 1, 5, 9], 2, 3),
        false
    );
}
