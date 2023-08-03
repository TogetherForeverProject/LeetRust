// Title: Distinct Subsequences
// Difficulty: Hard
// Category: On String
// Link: https://leetcode.com/problems/distinct-subsequences/
// Last Executed: Thu, 2023-08-03 14:57:41 AEST

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        // Convert the input strings into character vectors for easy access
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        // Get the lengths of the strings as signed integers
        let m = s_chars.len() as i32;
        let n = t_chars.len() as i32;

        // Create a DP array to store the results of subproblems
        let mut dp = vec![vec![-1; n as usize]; m as usize];

        // Nested helper function to perform recursive calculation
        fn helper(i: i32, j: i32, s: &Vec<char>, t: &Vec<char>, dp: &mut Vec<Vec<i32>>) -> i32 {
            // Base cases: if i < j, or j < 0, there are no valid subsequences
            if i < j {
                return 0;
            }
            if j < 0 {
                return 1;
            }
            // If the result is already computed, return it from the DP array
            if dp[i as usize][j as usize] != -1 {
                return dp[i as usize][j as usize];
            }
            // Check if the characters at the current indices match
            if s[i as usize] == t[j as usize] {
                // If they match, we have two choices:
                // 1. Include the current character in the subsequence, and check subsequences for the remaining strings.
                // 2. Skip the current character and check subsequences for the remaining strings.
                dp[i as usize][j as usize] = helper(i - 1, j - 1, s, t, dp) + helper(i - 1, j, s, t, dp);
            } else {
                // If the characters don't match, skip the current character and check subsequences for the remaining strings.
                dp[i as usize][j as usize] = helper(i - 1, j, s, t, dp);
            }
            // Return the result from the DP array
            dp[i as usize][j as usize]
        }

        // Call the helper function with initial parameters to calculate the number of distinct subsequences
        helper(m - 1, n - 1, &s_chars, &t_chars, &mut dp)
    }
}

pub fn solve() {
    // Input: s = "rabbbit", t = "rabbit"
    let s = "rabbbit".to_string();
    let t = "rabbit".to_string();
    let result = Solution::num_distinct(s, t);
    println!("Number of distinct subsequences: {}", result);
}
