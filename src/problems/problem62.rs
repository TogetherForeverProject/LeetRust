// Title: Unique Paths
// Difficulty: Medium
// Category: Matrix
// Link: https://leetcode.com/problems/unique-paths/
// Last Executed: Mon, 2023-07-31 21:52:08 AEST

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];

        // Initialize the first row and first column to 1
        // Calculate the number of unique paths for each cell in the grid.
        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

pub fn solve() {
    // Input: m = 3, n = 7
    let m = 3;
    let n = 7;

    let unique_paths = Solution::unique_paths(m, n);

    println!("Number of unique paths: {}", unique_paths);
}
