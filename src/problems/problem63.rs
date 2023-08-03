// Title: Unique Paths II
// Difficulty: Medium
// Category: Matrix
// Link: https://leetcode.com/problems/unique-paths-ii/
// Last Executed: Thu, 2023-08-03 14:57:50 AEST

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![0; n];
        let mut has_obstacle = false;

        // Initialize the first row to 1 if there are no obstacles
        for j in 0..n {
            if grid[0][j] == 1 {
                has_obstacle = true;
            }
            dp[j] = if has_obstacle { 0 } else { 1 };
        }

        // Calculate the number of unique paths for each row in the grid.
        for i in 1..m {
            dp[0] = if grid[i][0] == 1 { 0 } else { dp[0] };
            for j in 1..n {
                if grid[i][j] == 1 {
                    dp[j] = 0;
                } else {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n - 1]
    }
}

pub fn solve() {
    // Input: grid = [[0,0,0],[0,1,0],[0,0,0]]
    let grid = vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0],
    ];
    let unique_paths = Solution::unique_paths_with_obstacles(grid);
    println!("Number of unique paths: {}", unique_paths);
}
