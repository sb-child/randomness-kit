use rand::RngExt as _;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::shannon_entropy;

#[allow(unused)]
fn optimize() {
    fn optimize_parameters(training_data: &[(f64, f64)]) {
        use rand::{Rng, rng};
        println!("开始优化参数 (数据点数量: {})...", training_data.len());
        let mut rng = rng();

        let mut best_loss = f64::INFINITY;
        let mut best_params = (0.0, 0.0, 0.0);

        for _ in 0..200_000 {
            let try_a = rng.random_range(1.0..1000.0);
            let try_b = rng.random_range(0.0..100.0);
            let try_k = rng.random_range(0.1..1.5);

            let loss = calculate_loss(training_data, try_a, try_b, try_k);

            if loss < best_loss {
                best_loss = loss;
                best_params = (try_a, try_b, try_k);
            }
        }

        println!(
            "粗略搜索完成。当前最佳: A={:.4}, B={:.4}, K={:.4}, Loss={:.6}",
            best_params.0, best_params.1, best_params.2, best_loss
        );

        let mut current_a = best_params.0;
        let mut current_b = best_params.1;
        let mut current_k = best_params.2;
        let step_size = 2.0;

        for i in 0..100000 {
            let current_step = step_size * (0.999_f64.powi(i));

            let try_a = current_a + rng.random_range(-current_step..current_step);
            let try_b = current_b + rng.random_range(-current_step..current_step);
            let try_k = current_k + rng.random_range(-current_step..current_step);

            let loss = calculate_loss(training_data, try_a, try_b, try_k);

            if loss < best_loss {
                best_loss = loss;
                current_a = try_a;
                current_b = try_b;
                current_k = try_k;
            }
        }

        println!("BEST_A: {:.20}", current_a);
        println!("BEST_B: {:.20}", current_b);
        println!("BEST_K: {:.20}", current_k);
        println!("MSE Loss: {:.8}", best_loss);
    }

    fn calculate_loss(data: &[(f64, f64)], a: f64, b: f64, k: f64) -> f64 {
        let mut sum_sq_error = 0.0;
        let c = 8.0;

        for &(n, h_actual) in data {
            if n <= 1.0 {
                continue;
            }

            let fitted_h = c - a * (n + b).powf(-k);

            let diff = h_actual - fitted_h;
            sum_sq_error += diff * diff;
        }

        sum_sq_error / (data.len() as f64)
    }
    let data_points: Vec<(f64, f64)> = (1000..10000)
        .into_par_iter()
        .map(|i| {
            let shannon_entropy: f64 = (0..100)
                .into_par_iter()
                .map(|_| {
                    let r = libsodium_rs::random::bytes(i);
                    let ent = shannon_entropy::entropy(&r);
                    ent
                })
                .filter(|x| !x.is_nan())
                .sum::<f64>()
                / 100.0;
            (i as f64, shannon_entropy)
        })
        .collect();
    optimize_parameters(&data_points);
}

#[allow(unused)]
fn lut() {
    const SAMPLES: usize = 200_000;
    const MAX_N: usize = 1000;

    println!("static SMALL_N_ENTROPY: [f64; {}] = [", MAX_N);

    print!("0.0, ");

    for n in 1..MAX_N {
        let sum_entropy: f64 = (0..SAMPLES)
            .into_par_iter()
            .map(|_| {
                let r: Vec<u8> = libsodium_rs::random::bytes(n);
                shannon_entropy::entropy(&r)
            })
            .sum();

        let avg_entropy = sum_entropy / (SAMPLES as f64);

        print!("{:.20}, ", avg_entropy);
        if n % 10 == 0 {
            println!("");
        }
    }
    println!("];");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn run_optimize() {
        optimize();
    }

    #[test]
    #[ignore]
    fn run_lut() {
        lut();
    }
}
