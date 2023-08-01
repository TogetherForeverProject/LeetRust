// Title: Edit Distance
// Difficulty: Medium
// Category: On String
// Link: https://leetcode.com/problems/edit-distance/
// Last Executed: Wed, 2023-08-02 01:23:55 AEST

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        let m = chars1.len();
        let n = chars2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Base cases: converting an empty string to a non-empty string
        for i in 1..=m {
            dp[i][0] = i as i32;
        }
        for j in 1..=n {
            dp[0][j] = j as i32;
        }

        // Calculate the minimum number of operations for each substring
        for i in 1..=m {
            for j in 1..=n {
                if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }

        dp[m][n]
    }
}

pub fn solve() {
    // Input: word1 = "horse", word2 = "ros"
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    let min_ops = Solution::min_distance(word1, word2);
    println!("Minimum number of operations: {}", min_ops);
}
