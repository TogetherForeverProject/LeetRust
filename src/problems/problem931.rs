// Title: Minimum Falling Path Sum
// Difficulty: Medium
// Category: Matrix
// Link: https://leetcode.com/problems/minimum-falling-path-sum/
// Last Executed: Wed, 2023-08-02 11:28:14 AEST

struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();

        // Create a mutable copy of the last row as the initial DP row
        let mut dp = matrix[n - 1].clone();

        // Calculate the minimum falling path sum using dynamic programming
        for i in (0..n - 1).rev() {
            let mut new_dp = vec![0; n];
            for j in 0..n {
                let mut min_sum = dp[j];
                if j > 0 {
                    min_sum = min_sum.min(dp[j - 1]);
                }
                if j < n - 1 {
                    min_sum = min_sum.min(dp[j + 1]);
                }
                new_dp[j] = matrix[i][j] + min_sum;
            }
            dp = new_dp;
        }

        // Find the minimum sum in the first row, which represents the minimum falling path sum
        dp.iter().cloned().min().unwrap()
    }
}

pub fn solve() {
    // Input: matrix = [[2,1,3],[6,5,4],[7,8,9]]
    let matrix = vec![
        vec![2, 1, 3],
        vec![6, 5, 4],
        vec![7, 8, 9],
    ];
    let min_sum = Solution::min_falling_path_sum(matrix);
    println!("Minimum falling path sum: {}", min_sum);
}
