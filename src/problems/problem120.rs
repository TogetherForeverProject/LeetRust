// Title: Triangle
// Difficulty: Medium
// Category: Matrix
// Link: https://leetcode.com/problems/triangle/
// Last Executed: Thu, 2023-08-03 14:57:41 AEST

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone();

        for i in (0..n - 1).rev() {
            for j in 0..=i {
                dp[j] = triangle[i][j] + dp[j].min(dp[j + 1]);
            }
        }

        dp[0]
    }
}

pub fn solve() {
    // Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
    let triangle = vec![
        vec![2],
        vec![3, 4],
        vec![6, 5, 7],
        vec![4, 1, 8, 3],
    ];
    let min_path_sum = Solution::minimum_total(triangle);
    println!("Minimum path sum: {}", min_path_sum);
}
