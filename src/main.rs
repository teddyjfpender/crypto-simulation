use rand_distr::{Distribution, Normal};
use coingecko::{CoinGeckoClient, response::coins::MarketChart};
use chrono::{NaiveDate, NaiveDateTime};
use rayon::prelude::*;
use itertools::Itertools; // for `percentile`
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::env;

#[derive(Debug)]
struct Walk {
    walk: Vec<f64>,
}

#[derive(Debug)]
struct RandomWalks {
    walks: Vec<Walk>
}

#[derive(Debug)]
struct SimulationResults {
    fifth: Walk,
    fiftieth: Walk,
    ninety_fifth: Walk,
}

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

fn daily_price_changes(chart: &MarketChart) -> Vec<f64> {
    let mut changes: Vec<f64> = Vec::new();
    let prices: &Vec<Vec<f64>> = &chart.prices;
    for i in 1..prices.len() {
        let prev_price: f64 = prices[i - 1][1];
        let curr_price: f64 = prices[i][1];
        changes.push(curr_price / prev_price);
    }
    return changes;

}


fn mean(data: &Vec<f64>) -> Option<f64> {
    // calculate the sum of the data
    let sum: f64 = data.iter().fold(0.0, |sum, x| sum + x);
    // calcualte the total number of entries
    let count: usize = data.len();
    // calcualte the mean with a match function
    let mean = match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None
    };

    return mean;
}

fn standard_deviation(data: &Vec<f64>) -> Option<f64> {
    let std = match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            // calculate the variance
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);
                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    };

    return std;
}

fn random_walk(distribution: &Normal<f64>, num_steps: usize, start: f64) -> Walk {
    // initialise the walk vector
    let mut walk: Vec<f64> = Vec::new();
    // push the starting value
    walk.push(start);
    // loop through values 
    for i in 1..num_steps {
        let growth_rate = distribution.sample(&mut rand::thread_rng());
        walk.push(walk[i-1] * growth_rate);
    }

    Walk {
        walk: walk
    }
}

fn parallel_walks(num_walks: usize, distribution: &Normal<f64>, num_steps: usize, start: f64) -> Vec<Walk> {
    let walks: Vec<Walk> = (0..num_walks).into_par_iter().map(|_| {
        random_walk(distribution, num_steps, start)
    }).collect();
    // This last expression Rust is used to return the value
    return walks
}

fn calculate_simulation_percentiles(walks: &Vec<Walk>, num_steps: usize, num_walks: usize) -> SimulationResults {
    // Initialise empty vectors to hold the percentile values
    let mut fifth_vec: Walk = Walk { walk: Vec::with_capacity(num_steps) };
    let mut fiftieth_vec: Walk = Walk { walk: Vec::with_capacity(num_steps) };
    let mut ninety_fifth_vec: Walk = Walk { walk: Vec::with_capacity(num_steps) };

    // Calculate percentiles for each step
    for i in 0..num_steps {
        let mut steps_values: Vec<f64> = walks.par_iter().map(|walk| walk.walk[i]).collect();
        steps_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        fifth_vec.walk.push(percentile(&steps_values, 5.0));
        fiftieth_vec.walk.push(percentile(&steps_values, 50.0));
        ninety_fifth_vec.walk.push(percentile(&steps_values, 95.0));
    }

    SimulationResults { 
        fifth: fifth_vec, 
        fiftieth: fiftieth_vec, 
        ninety_fifth: ninety_fifth_vec 
    }
}

fn percentile(data: &[f64], p: f64) -> f64 {
    let n = data.len();
    let rank = (p / 100.0 * (n as f64 - 1.0)).round() as usize;
    data[rank]
}