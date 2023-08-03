// Title: Longest Arithmetic Subsequence
// Difficulty: Medium
// Category: On String
// Link: https://leetcode.com/problems/longest-arithmetic-subsequence/
// Last Executed: Thu, 2023-08-03 14:57:38 AEST

struct Solution;

impl Solution {
    // Helper function to find the minimum and maximum differences between elements in the array
    fn find_deltas(nums: &Vec<i32>) -> (i32, i32) {
        let mut min_val = nums[0];
        let mut max_val = nums[0];
        let mut min_delta = i32::MAX;
        let mut max_delta = i32::MIN;

        for &n in nums.iter().skip(1) {
            min_delta = min_delta.min(n - max_val);
            max_delta = max_delta.max(n - min_val);
            min_val = min_val.min(n);
            max_val = max_val.max(n);
        }

        (min_delta, max_delta)
    }

    // Main function to find the length of the longest arithmetic subsequence
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        // Find the minimum and maximum differences between elements in the array
        let (min_delta, max_delta) = Solution::find_deltas(&nums);

        // Calculate the size of the DP array based on the range of differences
        let size = (max_delta - min_delta + 1) as usize;

        // Initialize the DP array with 0s
        let mut dp = vec![vec![0; nums.len()]; size];

        // Initialize the maximum length of arithmetic subsequence to 2 (minimum possible length)
        let mut max = 2;

        // Iterate through the array to find the longest arithmetic subsequence
        for i in 1..nums.len() {
            for j in 0..i {
                // Calculate the difference between elements at positions i and j
                let delta = (nums[i] - nums[j] - min_delta) as usize;

                // Update the DP array to store the length of the arithmetic subsequence ending at index i
                dp[delta][i] = 2_i32.max(dp[delta][j] + 1);

                // Update the maximum length if needed
                max = max.max(dp[delta][i]);
            }
        }

        max
    }
}

pub fn solve() {
    // Innput: nums = [3,6,9,12]
    let nums = vec![3, 6, 9, 12];

    // Call the function to find the length of the longest arithmetic subsequence
    let result = Solution::longest_arith_seq_length(nums);

    // Print the result
    println!("Length of the longest arithmetic subsequence: {}", result);
}
