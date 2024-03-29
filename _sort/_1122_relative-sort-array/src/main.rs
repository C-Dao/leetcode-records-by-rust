/*
 * @lc app=leetcode id=1122 lang=rust
 *
 * [1122] Relative Sort Array
 *
 * https://leetcode.com/problems/relative-sort-array/description/
 *
 * algorithms
 * Easy (68.28%)
 * Likes:    2061
 * Dislikes: 119
 * Total Accepted:    146.4K
 * Total Submissions: 213.8K
 * Testcase Example:  '[2,3,1,3,2,4,6,7,9,2,19]\n[2,1,4,3,9,6]'
 *
 * Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all
 * elements in arr2 are also in arr1.
 *
 * Sort the elements of arr1 such that the relative ordering of items in arr1
 * are the same as in arr2. Elements that do not appear in arr2 should be
 * placed at the end of arr1 in ascending order.
 *
 *
 * Example 1:
 *
 *
 * Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
 * Output: [2,2,2,1,4,3,3,9,6,7,19]
 *
 *
 * Example 2:
 *
 *
 * Input: arr1 = [28,6,22,8,44,17], arr2 = [22,28,8,6]
 * Output: [22,28,8,6,17,44]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= arr1.length, arr2.length <= 1000
 * 0 <= arr1[i], arr2[i] <= 1000
 * All the elements of arr2 are distinct.
 * Each arr2[i] is in arr1.
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut counts = vec![0; 1001];
        let mut ans = vec![0; arr1.len()];
        for num in arr1.iter() {
            counts[*num as usize] += 1;
        }

        let mut index = 0;

        for num in arr2 {
            while counts[num as usize] > 0 {
                ans[index] = num;
                index += 1;
                counts[num as usize] -= 1;
            }
        }

        for num in 0..counts.len() {
            while counts[num] > 0 {
                ans[index] = num as i32;
                index += 1;
                counts[num as usize] -= 1;
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
}
