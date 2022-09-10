/*
 * @lc app=leetcode id=1089 lang=rust
 *
 * [1089] Duplicate Zeros
 *
 * https://leetcode.com/problems/duplicate-zeros/description/
 *
 * algorithms
 * Easy (51.32%)
 * Likes:    1793
 * Dislikes: 565
 * Total Accepted:    286.8K
 * Total Submissions: 558.8K
 * Testcase Example:  '[1,0,2,3,0,4,5,0]'
 *
 * Given a fixed-length integer array arr, duplicate each occurrence of zero,
 * shifting the remaining elements to the right.
 * 
 * Note that elements beyond the length of the original array are not written.
 * Do the above modifications to the input array in place and do not return
 * anything.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: arr = [1,0,2,3,0,4,5,0]
 * Output: [1,0,0,2,3,0,0,4]
 * Explanation: After calling your function, the input array is modified to:
 * [1,0,0,2,3,0,0,4]
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: arr = [1,2,3]
 * Output: [1,2,3]
 * Explanation: After calling your function, the input array is modified to:
 * [1,2,3]
 * 
 * 
 * 
 * Constraints:
 * 
 * 
 * 1 <= arr.length <= 10^4
 * 0 <= arr[i] <= 9
 * 
 * 
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
