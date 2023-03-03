pub mod prices {
    use coingecko::response::coins::MarketChart;

    pub(crate) fn daily_price_changes(chart: &MarketChart) -> Vec<f64> {
        let mut changes: Vec<f64> = Vec::new();
        let prices: &Vec<Vec<f64>> = &chart.prices;
        for i in 1..prices.len() {
            let prev_price: f64 = prices[i - 1][1];
            let curr_price: f64 = prices[i][1];
            changes.push(curr_price / prev_price);
        }
        return changes;
    }
}