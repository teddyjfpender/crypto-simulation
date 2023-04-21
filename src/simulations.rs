use rand_distr::{Normal, Distribution};

use crate::models::{Walk, SimulationResults};
use crate::statistics::{percentile};
use rayon::prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

pub fn random_walk(distribution: &Normal<f64>, num_steps: usize, start: f64) -> Walk {
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

pub fn parallel_walks(num_walks: usize, distribution: &Normal<f64>, num_steps: usize, start: f64) -> Vec<Walk> {
    let walks: Vec<Walk> = (0..num_walks).into_par_iter().map(|_| {
        random_walk(distribution, num_steps, start)
    }).collect();
    // This last expression Rust is used to return the value
    return walks
}

pub fn calculate_simulation_percentiles(walks: &Vec<Walk>, num_steps: usize, price: f64) -> SimulationResults {
    // Initialise empty vectors to hold the percentile values
    let mut fifth_vec: Walk = Walk { walk: Vec::with_capacity(num_steps) };
    let mut fiftieth_vec: Walk = Walk { walk: Vec::with_capacity(num_steps) };
    let mut ninety_fifth_vec: Walk = Walk { walk: Vec::with_capacity(num_steps) };

    // Calculate percentiles for each step
    for i in 0..num_steps {
        let mut steps_values: Vec<f64> = walks.par_iter().map(|walk| walk.walk[i]).collect();
        steps_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        fifth_vec.walk.push(percentile(&steps_values, 5.0) * price);
        fiftieth_vec.walk.push(percentile(&steps_values, 50.0) * price);
        ninety_fifth_vec.walk.push(percentile(&steps_values, 95.0) * price);
    }

    SimulationResults { 
        fifth: fifth_vec, 
        fiftieth: fiftieth_vec, 
        ninety_fifth: ninety_fifth_vec 
    }
}

