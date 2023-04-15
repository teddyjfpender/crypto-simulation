use criterion::{criterion_group, criterion_main, Criterion};
mod statistics;
use statistics::statistics::{parallel_walks, calculate_simulation_percentiles};
mod models;
use crate::models::{Walk, SimulationResults};
use rand_distr::Normal;

fn benchmark_main(simulations: usize) {

    // define args
    let steps: usize = 10;
    let start: f64 = 1.0;
    let price: f64 = 1.0;

    // create a random walk with this new mean and standard deviation
    let normal_distribution: Normal<f64> = Normal::new(1.0, 0.5).unwrap();
    // parallelise the random walks
    let walks: Vec<Walk> = parallel_walks(simulations, &normal_distribution, steps, start);
    // calculate percentile of walks
    let _walk_percentile: SimulationResults = calculate_simulation_percentiles(&walks, steps, price);
}

fn criterion_benchmark(c: &mut Criterion) {
    let simulations = vec![10, 100, 1000, 10000, 100000, 1000000];
    for sim in simulations {
        c.bench_function(format!("Benchmark With {} simulations", sim).as_str(), |b| b.iter(|| benchmark_main(sim)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
