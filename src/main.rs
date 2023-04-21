mod cli;
mod statistics;
mod simulations;
mod prices;
mod models;
mod run_simulations;

use crate::cli::Config;
use crate::models::SimulationResults;
use crate::run_simulations::run_simulation;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let walk_percentile: SimulationResults = run_simulation(config).await;
    println!("{:#?}", walk_percentile);
}