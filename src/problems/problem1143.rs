// Title: Longest Common Subsequence
// Difficulty: Medium
// Category: Longest Common Subsequence
// Link: https://leetcode.com/problems/longest-common-subsequence/
// Last Executed: Thu, 2023-08-03 14:57:40 AEST

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        // Convert the  input strings to the character arrays for easy access
        let text1_chars: Vec<char> = text1.chars().collect();
        let text2_chars: Vec<char> = text2.chars().collect();

        // Get the lengths of the strings
        let m = text1_chars.len();
        let n = text2_chars.len();

        // Create a 2D dp table to store the lengths of the longest common subsequence
        // dp[i][j] represents the length of the longest common subsequence between the first i
        // characters and the first j characters
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = 0;
        }

        for j in 0..=n {
            dp[0][j] = 0;
        }

        for i in 1..=m {
            for j in 1..=n {
                if text1_chars[i - 1] == text2_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        dp[m][n]
    }
}

pub fn solve() {
    // Input: text1 = "abcde", text2 = "ace"
    let text1 = String::from("abcde");
    let text2 = String::from("ace");
    let result = Solution::longest_common_subsequence(text1, text2);
    println!("{}", result);
}
