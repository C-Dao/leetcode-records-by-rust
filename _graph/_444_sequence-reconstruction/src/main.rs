/*
* @lc app=leetcode id=444 lang=rust
*
* [444] Sequence Reconstruction
*
* https://leetcode.com/problems/sequence-reconstruction/description/
*
* algorithms
* Medium (47.12%)
* Likes:    8719
* Dislikes: 289
* Total Accepted:    785K
* Total Submissions: 1.6M
* Testcase Example:  '[1,2,3]\n[[1,2],[1,3]]'
*
* Given an array of integers nums of length n, where nums is an arrangement of integers in the range [1, n].
*
* A 2D array of integers sequences is also provided, where sequences[i] is a subsequence of nums.
*
* Check if nums is the unique shortest subsequence. The shortest subsequence is the shortest sequence and all
* sequences sequences[i] are its subsequences. For a given array of sequences, there may be more than one valid subsequence.
*
* For example, for sequences = [[1,2],[1,3]], there are two shortest subsequences, [1,2,3] and [1,3,2].
*
* And for sequences = [[1,2],[1,3],[1,2,3]], the only possible shortest subsequence is [1,2,3]. [1,2,3,4] is the possible
* super-sequence, but not the shortest.
*
* Returns true if nums is the only shortest subsequence of the sequence, false otherwise.
*
* Subsequence is a sequence in which some elements can be removed from another sequence or none can be removed
* without changing the order of the remaining elements.


*
* Example 1:
*
*
* Input: nums = [1,2,3], sequences = [[1,2],[1,3]]
* Output: false
* Explanation:
* There are two possible super sequences: [1,2,3] and [1,3,2].
* The sequence [1,2] is a subsequence of [1,2,3] and [1,3,2].
* The sequence [1,3] is a subsequence of [1,2,3] and [1,3,2].
* Since nums is not the only super sequence, false is returned.
*
* Example 2:
*
*
* Input: nums = [1,2,3], sequences = [[1,2]]
* Output: false
* Explanation:
* The shortest possible hypersequence is [1,2].
* The sequence [1,2] is its subsequence: [1,2].
* Since nums is not the shortest possible super sequence, false is returned.
*
*
* Example 3:
*
* Input: nums = [1,2,3], sequences = [[1,2],[1,3],[2,3]]
* Output: true
* Explanation:
* The shortest possible super sequence is [1,2,3].
* The sequence [1,2] is a subsequence of it: [1,2,3].
* The sequence [1,3] is a subsequence of it: [1,2,3].
* The sequence [2,3] is a subsequence of: [1,2,3].
* Since nums is the only shortest super sequence, it returns true
*
*
*
* Constraints:
*
* n == nums.length
* 1 <= n <= 104
* nums is an arrangement of all integers in the range [1, n]
* 1 <= sequences.length <= 104
* 1 <= sequences[i].length <= 104
* 1 <= sum(sequences[i].length) <= 105
* 1 <= sequences[i][j] <= n
* All arrays of sequences are unique
* Sequences[i] is a subsequence of nums
*
*/

struct Solution {}

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        let mut graph = HashMap::new();
        let mut indegrees = HashMap::new();

        for seq in sequences.iter() {
            for &num in seq.iter() {
                if num < 1 || num > nums.len() as i32 {
                    return false;
                }

                if !graph.contains_key(&num) {
                    graph.insert(num, HashSet::new());
                }

                if !indegrees.contains_key(&num) {
                    indegrees.insert(num, 0);
                }
            }

            for i in 0..seq.len() - 1 {
                let num1 = seq[i];
                let num2 = seq[i + 1];

                if graph.get(&num1).unwrap().contains(&num2) {
                    graph.get_mut(&num1).unwrap().insert(num2);
                    indegrees.insert(num2, indegrees.get(&num2).unwrap() + 1);
                }
            }
        }

        let mut queue = VecDeque::new();
        let mut ans = vec![];

        for &num in indegrees.keys() {
            if indegrees.get(&num).unwrap() == &0 {
                queue.push_back(num);
            }
        }

        while queue.len() == 1 {
            let num = queue.pop_front().unwrap();
            ans.push(num);

            for &next in graph.get(&num).unwrap() {
                indegrees.insert(next, indegrees.get(&next).unwrap() - 1);
                if indegrees.get(&next).unwrap() == &0 {
                    queue.push_back(next);
                }
            }
        }

        ans == nums
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 2], vec![1, 3]]),
        false
    );
}
