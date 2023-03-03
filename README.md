# Monte Carlo Simulations with Rust

This project generates a series of random walks based on the price history of a cryptocurrency retrieved from the Coingecko API.

The script allows for command line arguments to be passed in to specify the cryptocurrency, the number of simulations, and the number of steps in the simulation.

The results are then printed to the console, showing the three percentile simulations (5th, 50th, and 95th) in a graph format.

## How Fast?

This table shows the execution time in milliseconds for the given number of simulations, along with the percentage of outlier measurements. Outliers are measurements that fall outside of the expected range of values and may indicate issues with the performance test setup or environment.

| Number of simulations | Time (ms)       | Outliers             |
| --------------------- | --------------- | -------------------- |
| 10                    | 0.352 - 0.363   | 1 (1.00%) outlier    |
| 100                   | 0.493 - 0.499   | 8 (8.00%) outliers   |
| 1000                  | 1.243 - 1.251   | 5 (5.00%) outliers   |
| 10000                 | 6.798 - 6.833   | 3 (3.00%) outliers   |
| 100000                | 75.084 - 75.296 | 6 (6.00%) outliers   |
| 1000000               | 913.64 - 917.72 | 13 (13.00%) outliers |

You can run the benchmarks by using the following command:

```
$ cargo bench
```

## Dependencies

This project requires the following dependencies:

- `rand_distr`: Used to generate the normal distribution for the random walks.
- `coingecko`: Used to retrieve cryptocurrency price history.
- `chrono`: Used to manipulate time and date information.
- `rayon`: Used to parallelise the random walks.
- `itertools`: Used to calculate the percentiles of the simulations.

## Usage

To use this project, first install the dependencies:

```
$ cargo build
```

Next, run the script using the following command:

```
$ cargo run -- --coin <coin_name> --simulations <num_simulations> --steps <num_steps>
```

Where:

- `<coin_name>`: Name of the cryptocurrency to retrieve price data for. Default: "bitcoin".
- `<num_simulations>`: Number of simulations to perform. Default: 1000.
- `<num_steps>`: Number of steps to simulate. Default: 7.

## Example

```
$ cargo run -- --coin ethereum --simulations 10000 --steps 14
```

This will perform 10,000 simulations for Ethereum, with 14 steps each (i.e. 14 days).
