/*
 * @lc app=leetcode id=421 lang=rust
 *
 * [421] Maximum XOR of Two Numbers in an Array
 *
 * https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/description/
 *
 * algorithms
 * Medium (54.48%)
 * Likes:    4612
 * Dislikes: 347
 * Total Accepted:    136.1K
 * Total Submissions: 250K
 * Testcase Example:  '[3,10,5,25,2,8]'
 *
 * Given an integer array nums, return the maximum result of nums[i] XOR
 * nums[j], where 0 <= i <= j < n.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [3,10,5,25,2,8]
 * Output: 28
 * Explanation: The maximum result is 5 XOR 25 = 28.
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [14,70,53,83,49,91,36,80,92,51,66,70]
 * Output: 127
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 2 * 10^5
 * 0 <= nums[i] <= 2^31 - 1
 *
 *
 */

struct Solution {}

// @lc code=start

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    pub fn build_trie(nums: Vec<i32>) -> Trie {
        let mut root = Trie::default();
        for num in nums {
            let mut node = &mut root;
            for i in (0..32).rev() {
                let bit = (num >> i) & 1;
                node = node.children[bit as usize].get_or_insert_with(|| Box::new(Trie::default()));
            }
        }

        root
    }
}
impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let root = Trie::build_trie(nums.clone());

        let mut ans = 0;

        for num in nums {
            let mut node = &root;
            let mut xor = 0;

            for i in (0..32).rev() {
                let bit = (num >> i) & 1;

                if node.children[1 - bit as usize].is_some() {
                    node = node.children[1 - bit as usize].as_ref().unwrap();
                    xor = (xor << 1) + 1;
                } else {
                    node = node.children[bit as usize].as_ref().unwrap();
                    xor = xor << 1;
                }
            }

            ans = i32::max(ans, xor);
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
}
