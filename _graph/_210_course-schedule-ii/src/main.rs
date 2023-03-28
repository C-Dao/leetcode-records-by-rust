/*
 * @lc app=leetcode id=210 lang=rust
 *
 * [210] Course Schedule II
 *
 * https://leetcode.com/problems/course-schedule-ii/description/
 *
 * algorithms
 * Medium (47.12%)
 * Likes:    8719
 * Dislikes: 289
 * Total Accepted:    785K
 * Total Submissions: 1.6M
 * Testcase Example:  '2\n[[1,0]]'
 *
 * There are a total of numCourses courses you have to take, labeled from 0 to
 * numCourses - 1. You are given an array prerequisites where prerequisites[i]
 * = [ai, bi] indicates that you must take course bi first if you want to take
 * course ai.
 *
 *
 * For example, the pair [0, 1], indicates that to take course 0 you have to
 * first take course 1.
 *
 *
 * Return the ordering of courses you should take to finish all courses. If
 * there are many valid answers, return any of them. If it is impossible to
 * finish all courses, return an empty array.
 *
 *
 * Example 1:
 *
 *
 * Input: numCourses = 2, prerequisites = [[1,0]]
 * Output: [0,1]
 * Explanation: There are a total of 2 courses to take. To take course 1 you
 * should have finished course 0. So the correct course order is [0,1].
 *
 *
 * Example 2:
 *
 *
 * Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
 * Output: [0,2,1,3]
 * Explanation: There are a total of 4 courses to take. To take course 3 you
 * should have finished both courses 1 and 2. Both courses 1 and 2 should be
 * taken after you finished course 0.
 * So one correct course order is [0,1,2,3]. Another correct ordering is
 * [0,2,1,3].
 *
 *
 * Example 3:
 *
 *
 * Input: numCourses = 1, prerequisites = []
 * Output: [0]
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= numCourses <= 2000
 * 0 <= prerequisites.length <= numCourses * (numCourses - 1)
 * prerequisites[i].length == 2
 * 0 <= ai, bi < numCourses
 * ai != bi
 * All the pairs [ai, bi] are distinct.
 *
 *
 */

struct Solution {}

// @lc code=start
use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut indegrees, graph) = Self::build_graph(num_courses as usize, &prerequisites);

        let mut queue = VecDeque::new();
        let mut order = vec![];

        for i in 0..indegrees.len() {
            if indegrees[i] == 0 {
                queue.push_back(i);
            }
        }

        while !queue.is_empty() {
            let course = queue.pop_front().unwrap();

            order.push(course as i32);

            for next_course in graph.get(&course).unwrap() {
                indegrees[*next_course] -= 1;
                if indegrees[*next_course] == 0 {
                    queue.push_back(*next_course);
                }
            }
        }

        if order.len() == num_courses as usize {
            order
        } else {
            vec![]
        }
    }

    fn build_graph(
        num_courses: usize,
        prerequisites: &Vec<Vec<i32>>,
    ) -> (Vec<usize>, HashMap<usize, Vec<usize>>) {
        let mut graph = HashMap::new();
        let mut indegrees = vec![0; num_courses];
        for i in 0..num_courses {
            graph.insert(i, vec![]);
        }

        for prerequisty in prerequisites {
            graph
                .get_mut(&(prerequisty[1] as usize))
                .unwrap()
                .push(prerequisty[0] as usize);
            indegrees[prerequisty[0] as usize] += 1;
        }

        (indegrees, graph)
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), [0, 1]);
}
