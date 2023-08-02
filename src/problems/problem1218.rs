// Title: Longest Arithmetic Subsequence of Given Difference
// Difficulty: Medium
// Category: Longest Increasing Subsequence
// Link: https://leetcode.com/problems/longest-arithmetic-subsequence-of-given-difference/
// Last Executed: Wed, 2023-08-02 11:28:02 AEST

struct Solution;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp = vec![0; 200_001]; // The given constraints ensure that the array size won't exceed 200,000
        let mut max_length = 0;

        for num in arr {
            let index = (num + 100_000) as usize; // Shift the index to handle negative numbers
            let prev_length = dp[index - difference as usize];
            let length = prev_length + 1;
            dp[index] = length;
            max_length = max_length.max(length);
        }

        max_length
    }
}

pub fn solve() {
    // Input: arr = [1,2,3,4], difference = 1
    let arr = vec![1,2,3,4];
    let difference = 1;
    let result = Solution::longest_subsequence(arr, difference);
    println!("Length of the longest subsequence: {}", result);
}
