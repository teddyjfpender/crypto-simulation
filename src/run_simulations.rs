use rand_distr::{Normal};
use coingecko::{CoinGeckoClient, response::coins::MarketChart};
use chrono::{NaiveDate, NaiveDateTime};

use crate::cli::Config;
use crate::statistics::{standard_deviation, mean};
use crate::simulations::{parallel_walks, calculate_simulation_percentiles};
use crate::prices::prices::daily_price_changes;
use crate::models::{Walk, SimulationResults};

pub async fn run_simulation(config: Config) -> SimulationResults {
    let client: CoinGeckoClient = CoinGeckoClient::default();
    let from: NaiveDateTime = NaiveDate::from_ymd_opt(2022,11,1).unwrap().and_hms_opt(0,0,0).unwrap();
    let to: NaiveDateTime = chrono::Local::now().naive_local();
    let coin_history: MarketChart = client.coin_market_chart_range(&config.coin, "usd", from, to).await.unwrap();
    let coin_current_price: f64 = coin_history.prices[coin_history.prices.len() - 1][1];

    let coin_data: Vec<f64> = daily_price_changes(&coin_history);

    let mean_value: f64 = mean(&coin_data).unwrap_or(0.0);
    let std_value: f64 = standard_deviation(&coin_data).unwrap_or(1.0);

    let new_normal: Normal<f64> = Normal::new(mean_value, std_value).unwrap();
    let walks: Vec<Walk> = parallel_walks(config.simulations, &new_normal, config.steps, config.start);

    calculate_simulation_percentiles(&walks, config.steps, coin_current_price)
}
