// Title: Maximal Square
// Difficulty: Medium
// Category: Matrix
// Link: https://leetcode.com/problems/maximal-square/
// Last Executed: Mon, 2023-07-31 21:52:05 AEST

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut max_size = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                    }
                    max_size = max_size.max(dp[i][j]);
                }
            }
        }

        max_size * max_size
    }
}

pub fn solve() {
    // Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let area = Solution::maximal_square(matrix);
    println!("Largest square area: {}", area);
}
