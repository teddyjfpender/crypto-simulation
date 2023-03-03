use rand_distr::{Normal};
use coingecko::{CoinGeckoClient, response::coins::MarketChart};
use chrono::{NaiveDate, NaiveDateTime};
use std::env;
mod statistics;
use statistics::statistics::{standard_deviation, mean, parallel_walks, calculate_simulation_percentiles};
mod prices;
use crate::prices::prices::daily_price_changes;
mod models;
use crate::models::{Walk, SimulationResults};


#[tokio::main]
async fn main() {
    // get constants from cli args
    let args: Vec<String> = env::args().collect();

    // defualt args
    // coin of interest
    let mut coin: &str = "bitcoin";
    // starting value (will always be 1)
    let start: f64 = 1.0;
    // number of simulations to perform
    let mut simulations: usize = 1000;
    // number of steps ahead to walk (i.e. total days to forecast)
    let mut steps: usize = 7;

    // parse command line args
    for i in 1..args.len() {
        match args[i].as_str() {
            "--coin" => coin = args[i + 1].as_str(),
            "--simulations" => simulations = args[i + 1].parse().unwrap(),
            "--steps" => steps = args[i + 1].parse().unwrap(),
            _ => (),
        }
    }

    // create coingecko client
    let client: CoinGeckoClient = CoinGeckoClient::default();
    let from: NaiveDateTime = NaiveDate::from_ymd_opt(2022,11,1).unwrap().and_hms_opt(0,0,0).unwrap();
    let to: NaiveDateTime = NaiveDate::from_ymd_opt(2023, 3, 2).unwrap().and_hms_opt(0,0,0).unwrap();
    let coin_history: MarketChart = client.coin_market_chart_range(&coin, "usd", from, to).await.unwrap();
    let coin_data: Vec<f64> = daily_price_changes(&coin_history);

    // calcualte the mean of coin
    let mean_value = mean(&coin_data).unwrap_or(0.0);
    // calcualte the standard deviation
    let std_value = standard_deviation(&coin_data).unwrap_or(1.0);

    // create a random walk with this new mean and standard deviation
    let new_normal: Normal<f64> = Normal::new(mean_value, std_value).unwrap();
    // parallelise the random walks
    let walks: Vec<Walk> = parallel_walks(simulations, &new_normal, steps, start);
    // calculate percentile of walks
    let walk_percentile: SimulationResults = calculate_simulation_percentiles(&walks, steps, simulations);
    println!("{:#?}", walk_percentile);

}