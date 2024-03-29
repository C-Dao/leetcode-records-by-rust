/*
 * @lc app=leetcode id=875 lang=rust
 *
 * [875] Koko Eating Bananas
 *
 * https://leetcode.com/problems/koko-eating-bananas/description/
 *
 * algorithms
 * Medium (53.85%)
 * Likes:    5666
 * Dislikes: 264
 * Total Accepted:    256.4K
 * Total Submissions: 493.9K
 * Testcase Example:  '[3,6,7,11]\n8'
 *
 * Koko loves to eat bananas. There are n piles of bananas, the i^th pile has
 * piles[i] bananas. The guards have gone and will come back in h hours.
 *
 * Koko can decide her bananas-per-hour eating speed of k. Each hour, she
 * chooses some pile of bananas and eats k bananas from that pile. If the pile
 * has less than k bananas, she eats all of them instead and will not eat any
 * more bananas during this hour.
 *
 * Koko likes to eat slowly but still wants to finish eating all the bananas
 * before the guards return.
 *
 * Return the minimum integer k such that she can eat all the bananas within h
 * hours.
 *
 *
 * Example 1:
 *
 *
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 *
 *
 * Example 2:
 *
 *
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 *
 *
 * Example 3:
 *
 *
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= piles.length <= 10^4
 * piles.length <= h <= 10^9
 * 1 <= piles[i] <= 10^9
 *
 *
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_speed = i32::MIN;

        for pile in piles.iter() {
            max_speed = i32::max(max_speed, *pile);
        }

        let (mut left, mut right) = (1, max_speed);

        while left <= right {
            let mid = (left + right) / 2;
            let hours = Self::get_hours(&piles, mid);

            if hours <= h && (mid == 1 || Self::get_hours(&piles, mid - 1) > h) {
                return mid;
            }

            if hours <= h {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        -1
    }

    fn get_hours(piles: &Vec<i32>, speed: i32) -> i32 {
        piles
            .iter()
            .fold(0, |hours, pile| hours + (pile + speed - 1) / speed)
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
}
