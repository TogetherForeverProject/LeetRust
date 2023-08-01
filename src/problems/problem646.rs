// Title: Maximum Length of Pair Chain
// Difficulty: Medium
// Category: Longest Increasing Subsequence
// Link: https://leetcode.com/problems/maximum-length-of-pair-chain/
// Last Executed: Wed, 2023-08-02 01:23:52 AEST

struct Solution;

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        if pairs.is_empty() {
            return 0;
        }

        pairs.sort_by_key(|pair| pair[1]);

        let mut max_length = 1;
        let mut prev_end = pairs[0][1];

        for pair in pairs.iter().skip(1) {
            if pair[0] > prev_end {
                max_length += 1;
                prev_end = pair[1];
            }
        }

        max_length
    }
}

pub fn solve() {
    // Input: pairs = [[1,2],[2,3],[3,4]]
    let pairs = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    let result = Solution::find_longest_chain(pairs);
    println!("Length of longest chain: {}", result);
}
