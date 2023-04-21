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

pub fn percentile(data: &[f64], p: f64) -> f64 {
    let n = data.len();
    let rank = (p / 100.0 * (n as f64 - 1.0)).round() as usize;
    data[rank]
}