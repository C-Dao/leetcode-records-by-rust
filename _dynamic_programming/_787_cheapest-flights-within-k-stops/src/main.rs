/*
 * @lc app=leetcode id=787 lang=rust
 *
 * [787] Cheapest Flights Within K Stops
 *
 * https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
 *
 * algorithms
 * Medium (35.88%)
 * Likes:    7264
 * Dislikes: 319
 * Total Accepted:    333.6K
 * Total Submissions: 910.2K
 * Testcase Example:  '4\n[[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]]\n0\n3\n1'
 *
 * There are n cities connected by some number of flights. You are given an
 * array flights where flights[i] = [fromi, toi, pricei] indicates that there
 * is a flight from city fromi to city toi with cost pricei.
 *
 * You are also given three integers src, dst, and k, return the cheapest price
 * from src to dst with at most k stops. If there is no such route, return
 * -1.
 *
 *
 * Example 1:
 *
 *
 * Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]],
 * src = 0, dst = 3, k = 1
 * Output: 700
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 3 is marked in red and
 * has cost 100 + 600 = 700.
 * Note that the path through cities [0,1,2,3] is cheaper but is invalid
 * because it uses 2 stops.
 *
 *
 * Example 2:
 *
 *
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k
 * = 1
 * Output: 200
 * Explanation:
 * The graph is shown above.
 * The optimal path with at most 1 stop from city 0 to 2 is marked in red and
 * has cost 100 + 100 = 200.
 *
 *
 * Example 3:
 *
 *
 * Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k
 * = 0
 * Output: 500
 * Explanation:
 * The graph is shown above.
 * The optimal path with no stops from city 0 to 2 is marked in red and has
 * cost 500.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= n <= 100
 * 0 <= flights.length <= (n * (n - 1) / 2)
 * flights[i].length == 3
 * 0 <= fromi, toi < n
 * fromi != toi
 * 1 <= pricei <= 10^4
 * There will not be any multiple flights between two cities.
 * 0 <= src, dst, k < n
 * src != dst
 *
 *
 */

struct Solution {}

// @lc code=start

use std::collections::VecDeque;

impl Solution {
    // bidwidth first search
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut adj = vec![vec![]; n as usize];

        for flight in flights {
            adj[flight[0] as usize].push((flight[1], flight[2]));
        }

        let mut dist = vec![i32::MAX; n as usize];
        let mut queue = VecDeque::new();
        let mut level = 0;

        queue.push_back((src, 0));

        while level <= k && !queue.is_empty() {
            let mut size = queue.len();

            while size > 0 {
                size -= 1;
                let (node, costs) = queue.pop_front().unwrap();

                for &(neighbour, price) in adj[node as usize].iter() {
                    if price + costs >= dist[neighbour as usize] {
                        continue;
                    };
                    dist[neighbour as usize] = price + costs;
                    queue.push_back((neighbour, dist[neighbour as usize]));
                }
            }
            level += 1;
        }
        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }

    // dynamic programming
    pub fn find_cheapest_price_dp(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        let mut costs = vec![i32::MAX; n as usize];
        let mut ans = i32::MAX;

        costs[src as usize] = 0;

        for _ in 1..=k + 1 {
            let mut dp = vec![i32::MAX; n as usize];
            for flight in flights.iter() {
                dp[flight[0] as usize] = i32::min(
                    dp[flight[0] as usize],
                    costs[flight[1] as usize] + flight[2],
                );
            }
            costs = dp;
            ans = i32::min(ans, costs[dst as usize]);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end

fn main() {
    assert_eq!(
        Solution::find_cheapest_price(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200]
            ],
            0,
            3,
            1,
        ),
        700
    );
    assert_eq!(
        Solution::find_cheapest_price_dp(
            4,
            vec![
                vec![0, 1, 100],
                vec![1, 2, 100],
                vec![2, 0, 100],
                vec![1, 3, 600],
                vec![2, 3, 200]
            ],
            0,
            3,
            1,
        ),
        700
    );
}
