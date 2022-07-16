/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut pointer_one, mut pointer_two): (usize, usize) = (0, 0);
        loop {
            if pointer_one >= nums1.len() || pointer_two >= n as usize {
                break;
            }

            if pointer_one >= (m as usize + pointer_two) {
                nums1.insert(pointer_one, nums2[pointer_two]);
                nums1.pop();
                pointer_two += 1;
                pointer_one += 1;
                continue;
            }

            if nums1[pointer_one] <= nums2[pointer_two] {
                pointer_one += 1;
                continue;
            }
            if nums1[pointer_one] > nums2[pointer_two] {
                nums1.insert(pointer_one, nums2[pointer_two]);
                nums1.pop();
                pointer_two += 1;
                pointer_one += 1;
                continue;
            }
        }
    }
}
// @lc code=end

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    Solution::merge(&mut nums1, 3, &mut nums2, 3);

    println!("{:?}", nums1);
}
