// Topic: Dynamic Programming
// Title: House Robber
// Difficulty: Medium
// Category: Fibonacci Style
// Link: https://leetcode.com/problems/house-robber/
// Last Executed: Wed, 2023-08-02 11:28:04 AEST

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        }

        let mut dp = vec![0; n];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);

        for i in 2..n {
            dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
        }

        dp[n - 1]
    }
}

pub fn solve() {
    // Input: nums = [1,2,3,1]
    let nums = vec![1, 2, 3, 1];
    let max_money = Solution::rob(nums);

    println!("Maximum amount of money that can be robbed: {}", max_money);
}
