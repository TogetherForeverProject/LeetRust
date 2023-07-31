// Title: Minimum Path Sum
// Difficulty: Medium
// Category: Matrix
// Link: https://leetcode.com/problems/minimum-path-sum/
// Last Executed: Mon, 2023-07-31 21:52:09 AEST

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];

        // Initialize the first row and first column of dp with the cumulative sum.
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        // Calculate the minimum sum for each cell in the grid.
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = grid[i][j] + dp[i - 1][j].min(dp[i][j - 1]);
            }
        }

        dp[m - 1][n - 1]
    }
}

pub fn solve() {
    // Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
    let grid = vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1],
    ];
    let min_sum = Solution::min_path_sum(grid);
    println!("Minimum sum of the path: {}", min_sum);
}
