// Title: Uncrossed Lines
// Difficulty: Medium
// Category: Longest Common Subsequence
// Link: https://leetcode.com/problems/uncrossed-lines/
// Last Executed: Thu, 2023-08-03 14:57:39 AEST

struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();

        // Create a 1D dp array with length (n+1)
        // dp[j] represents the maximum number of connecting lines using the first i elements of nums1 and the first j elements of nums2
        let mut dp = vec![0; n + 1];

        // Previous variable to store the value of dp[i-1][j-1]
        let mut prev = 0;

        // Iterate over the dp array and fill it using dynamic programming
        for i in 1..=m {
            prev = 0; // Reset the prev variable for each row
            for j in 1..=n {
                let temp = dp[j]; // Store the current value of dp[j] to be used in the next iteration

                if nums1[i - 1] == nums2[j - 1] {
                    // If the current elements are equal, draw a connecting line and update the dp value
                    dp[j] = prev + 1;
                } else {
                    // If the current elements are not equal, find the maximum of the two options: ignore nums1[i-1] or ignore nums2[j-1]
                    dp[j] = dp[j].max(dp[j - 1]);
                }

                prev = temp; // Update the prev variable with the current value of dp[j-1] for the next iteration
            }
        }

        // The value at dp[n] will give us the maximum number of connecting lines
        dp[n]
    }
}

pub fn solve() {
    // Input: nums1 = [1,4,2], nums2= [1,2,4]
    let nums1 = vec![1, 4, 2];
    let nums2 = vec![1, 2, 4];
    let result = Solution::max_uncrossed_lines(nums1, nums2);

    println!("The maximum number of connecting lines is: {}", result);
}
