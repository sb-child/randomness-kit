mod func;
mod lut;
mod optimize;

pub use func::{entropy, metric_entropy, normalized_metric_entropy};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_entropy_functions() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};

        println!("n, shannon_entropy, metric_entropy, normalized_metric_entropy");
        for i in 1..2000 {
            let shannon_entropy: f64 = (0..100)
                .into_par_iter()
                .map(|_| {
                    let r = libsodium_rs::random::bytes(i);
                    let ent = entropy(&r);
                    ent
                })
                .filter(|x| !x.is_nan())
                .sum::<f64>()
                / 100.0;
            let metric_entropy: f64 = (0..100)
                .into_par_iter()
                .map(|_| {
                    let r = libsodium_rs::random::bytes(i);
                    let ent = metric_entropy(&r);
                    ent
                })
                .filter(|x| !x.is_nan())
                .sum::<f64>()
                / 100.0;
            let normalized_metric_entropy: f64 = (0..100)
                .into_par_iter()
                .map(|_| {
                    let r = libsodium_rs::random::bytes(i);
                    let ent = normalized_metric_entropy(&r);
                    ent
                })
                .filter(|x| !x.is_nan())
                .sum::<f64>()
                / 100.0;

            println!("{i}, {shannon_entropy}, {metric_entropy}, {normalized_metric_entropy}");
        }
    }
}
