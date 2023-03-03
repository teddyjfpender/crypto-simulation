pub mod statistics {
    use rand_distr::{Normal, Distribution};
    use rayon::prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

    use crate::models::{Walk, SimulationResults};


    pub fn mean(data: &Vec<f64>) -> Option<f64> {
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

    pub fn standard_deviation(data: &Vec<f64>) -> Option<f64> {
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

    pub fn calculate_simulation_percentiles(walks: &Vec<Walk>, num_steps: usize, num_walks: usize) -> SimulationResults {
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

    pub fn percentile(data: &[f64], p: f64) -> f64 {
        let n = data.len();
        let rank = (p / 100.0 * (n as f64 - 1.0)).round() as usize;
        data[rank]
    }
}