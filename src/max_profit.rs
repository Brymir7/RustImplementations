pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 1 {
        return 0;
    }
    let mut curr_min_price = prices[0];
    let mut max_profit = 0;
    for i in 0..prices.len() - 1 {
        let curr_price = prices[i];
        curr_min_price = curr_price.min(curr_min_price);
        let next_price = prices[i + 1];
        let curr_profit = next_price - curr_min_price;
        max_profit = max_profit.max(curr_profit);
    }
    return max_profit;
}

pub fn test_max_profit() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(max_profit(vec![1]), 0);
    assert_eq!(max_profit(vec![1, 2]), 1);
}
