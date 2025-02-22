use std::time::Instant;

use kand::ohlcv::sma::sma;
use rayon::prelude::*;

#[allow(clippy::unwrap_used)]
fn main() {
    // Generate sample prices with more realistic data
    let prices: Vec<f64> = (0..10_000_000)
        .map(|i| f64::from(i).sin().mul_add(100.0, 1000.0))
        .collect();
    let mut seq_output = vec![0.0; prices.len()];
    let mut par_outputs: Vec<Vec<f64>> = (0..10).map(|_| vec![0.0; prices.len()]).collect();

    // Common SMA parameters
    let period = 14;
    let iterations = 16;

    println!("=== SMA Performance Test ===");
    println!("Data points: {}", prices.len());
    println!("Period: {period}");
    println!("Iterations: {iterations}");
    println!("-------------------------");

    // Warm up the CPU
    for _ in 0..10 {
        sma(&prices, period, &mut seq_output).unwrap();
    }

    // Time sequential execution
    let start = Instant::now();
    for _ in 0..iterations {
        sma(&prices, period, &mut seq_output).unwrap();
    }
    let seq_duration = start.elapsed();
    let seq_avg = seq_duration / iterations;

    // Time parallel execution
    let start = Instant::now();
    par_outputs.par_iter_mut().for_each(|output| {
        sma(&prices, period, output).unwrap();
    });
    let par_duration = start.elapsed();
    let par_avg = par_duration / iterations;

    println!("\nSequential Execution");
    println!("  Total time: {seq_duration:.2?}");
    println!("  Avg time/iter: {seq_avg:.2?}");
    println!(
        "  Last 5 results: {:?}",
        &seq_output[seq_output.len() - 5..]
    );

    println!("\nParallel Execution");
    println!("  Total time: {par_duration:.2?}");
    println!("  Avg time/iter: {par_avg:.2?}");
    println!(
        "  Last 5 results: {:?}",
        &par_outputs[0][par_outputs[0].len() - 5..]
    );

    println!(
        "\nSpeedup: {:.2}x",
        seq_duration.as_secs_f64() / par_duration.as_secs_f64()
    );
}
