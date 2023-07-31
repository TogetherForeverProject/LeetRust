// Topic: Dynamic Programming
// Title: Delete and Earn
// Difficulty: Medium
// Category: Fibonacci Style
// Link: https://leetcode.com/problems/delete-and-earn/
// Last Executed: Mon, 2023-07-31 19:50:43 AEST

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let max_num = nums.iter().max().cloned().unwrap_or(0) as usize;
        let mut points = vec![0; max_num + 1];

        for &num in &nums {
            points[num as usize] += num;
        }

        let mut dp = vec![0; max_num + 1];
        dp[1] = points[1];

        for i in 2..=max_num {
            dp[i] = dp[i - 1].max(dp[i - 2] + points[i]);
        }

        dp[max_num]
    }
}

pub fn solve() {
    // Input: nums = [3,4,2]
    let nums = vec![3, 4, 2];
    let max_points = Solution::delete_and_earn(nums);

    println!("Maximum number of points: {}", max_points);
}
