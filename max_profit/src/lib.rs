pub fn max_profit(prices: Vec<i32>) -> i32 {
    // Solving with sliding window algo.

    // Store init buy price, update if lower price found.
    let mut buy_price = prices[0];
    // Max profit for tx.
    let mut max_profit = 0;
    // Iterate all price & cal. max profit.
    for price in prices {
        if buy_price > price {
            buy_price = price;
        } else {
            let profit = price - buy_price;
            if max_profit < profit {
                max_profit = profit;
            }
        }
    }
    // Return max profit
    max_profit
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_with_profit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn should_work_with_zero_profit() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
