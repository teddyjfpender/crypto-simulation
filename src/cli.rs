use std::env;

pub struct Config {
    pub coin: String,
    pub start: f64,
    pub simulations: usize,
    pub steps: usize,
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();

        let mut coin = String::from("bitcoin");
        let start = 1.0;
        let mut simulations = 1000;
        let mut steps = 7;

        for i in 1..args.len() {
            match args[i].as_str() {
                "--coin" => coin = args[i + 1].clone(),
                "--simulations" => simulations = args[i + 1].parse().unwrap(),
                "--steps" => steps = args[i + 1].parse().unwrap(),
                _ => (),
            }
        }

        Config {
            coin,
            start,
            simulations,
            steps,
        }
    }
}
