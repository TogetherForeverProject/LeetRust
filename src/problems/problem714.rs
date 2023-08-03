// Title: Best Time to Buy and Sell Stock with Transaction Fee
// Difficulty: Medium
// Category: Best Time to Buy & Sell Stock / State Machine
// Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
// Last Executed: Thu, 2023-08-03 14:58:15 AEST

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();

        if n < 2 {
            return 0;
        }

        // Initialize arrays to keep track of the maximum profit on each day for holding the stock and having cash
        let mut hold = vec![0; n];
        let mut cash = vec![0; n];

        // Base cases
        hold[0] = -prices[0];
        cash[0] = 0;

        for i in 1..n {
            // If we hold the stock on day i, we either continue holding from the previous day (hold[i-1])
            // or buy on the day before yesterday and hold (cash[i-2] - prices[i])
            hold[i] = hold[i - 1].max(cash[i - 1] - prices[i]);

            // If we have cash on day i, we either continue holding cash from the previous day (cash[i-1])
            // or sell the stock on the day before yesterday and buy again (hold[i-1] + prices[i] - fee)
            cash[i] = cash[i - 1].max(hold[i - 1] + prices[i] - fee);
        }

        // The maximum profit will be the maximum value between hold[n-1] and cash[n-1]
        cash[n - 1].max(hold[n - 1])
    }
}

pub fn solve() {
    // Input: prices = [1,3,2,8,4,9], fee = 2
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    let result = Solution::max_profit(prices, fee);
    println!("The maximum profit is: {}", result);
}
