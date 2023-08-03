// Title: Russian Dolls Envelopes
// Difficulty: Hard
// Category: On String
// Link: https://leetcode.com/problems/russian-doll-envelopes/
// Last Executed: Thu, 2023-08-03 14:57:47 AEST

struct Solution;

use std::cmp::Reverse;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        // Sort the envelopes by width in ascending order.
        // If two envelopes have the same width, sort them by height in descending order.
        let mut envs: Vec<_> = envelopes.iter().map(|envelope| (envelope[0], Reverse(envelope[1]))).collect();
        envs.sort_unstable();

        // Perform the Longest Increasing Subsequence (LIS) algorithm using binary search.
        let mut max_dp = Vec::new();
        for (_, Reverse(height)) in envs {
            // Find the position to insert the current height in the `max_dp` array using binary search.
            let insert_pos = max_dp.binary_search(&height).unwrap_or_else(|pos| pos);
            if insert_pos < max_dp.len() {
                // If the height can be inserted into the middle of the `max_dp` array,
                // update the value at that position to be the new height.
                max_dp[insert_pos] = height;
            } else {
                // If the height should be appended to the end of the `max_dp` array,
                // push it into the array.
                max_dp.push(height);
            }
        }

        max_dp.len() as i32
    }
}

pub fn solve() {
    // Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
    let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
    let result = Solution::max_envelopes(envelopes);
    println!("Maximum number of Russian doll envelopes: {}", result);
}
