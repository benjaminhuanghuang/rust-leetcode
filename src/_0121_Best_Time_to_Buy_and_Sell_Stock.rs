/*
https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

Easy
*/
pub struct Solution;

/*
  1. find min buy
*/
impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
      return 0;
    }
    let mut max_profit: i32 = 0;
    let mut lowest_price = i32::MAX;
    for price in prices.iter() {
      if *price < lowest_price {
        lowest_price = *price;
      }
      let profit = *price - lowest_price;
      if profit > max_profit {
        max_profit = profit;
      }
    }
    max_profit
  }
}
