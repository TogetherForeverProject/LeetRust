// Title: Best Time to Buy and Sell Stock with Cooldown
// Difficulty: Medium
// Category: Best Time to Buy & Sell Stock / State Machine
// Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
// Last Executed: Thu, 2023-08-03 14:57:47 AEST

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();

        if n < 2 {
            return 0;
        }

        // Initialize arrays to keep track of the maximum profit on each day for each state
        let mut buy = vec![0; n];
        let mut sell = vec![0; n];
        let mut cooldown = vec![0; n];

        // Base cases
        buy[0] = -prices[0];
        sell[0] = 0;
        cooldown[0] = 0;

        for i in 1..n {
            // If we buy on day i, we either continue the cooldown from the previous day (cooldown[i-1])
            // or buy on the day before yesterday and hold (sell[i-2] - prices[i])
            buy[i] = buy[i - 1].max(cooldown[i - 1] - prices[i]);

            // If we sell on day i, we either continue the cooldown from the previous day (cooldown[i-1])
            // or sell on the day before yesterday and buy again (buy[i-1] + prices[i])
            sell[i] = sell[i - 1].max(buy[i - 1] + prices[i]);

            // If we cooldown on day i, we take the maximum profit from the previous day (sell[i-1] or cooldown[i-1])
            cooldown[i] = cooldown[i - 1].max(sell[i - 1]);
        }

        // The maximum profit will be the maximum value among buy[n-1], sell[n-1], and cooldown[n-1]
        sell[n - 1].max(cooldown[n - 1])
    }
}

pub fn solve() {
    // Input: prices = [1,2,3,0,2]
    let prices = vec![1, 2, 3, 0, 2];
    let result = Solution::max_profit(prices);
    println!("The maximum profit is: {}", result);
}

