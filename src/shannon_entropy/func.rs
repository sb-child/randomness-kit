use crate::{shannon_entropy::lut::SMALL_N_ENTROPY, util::count_bytes};

/// 计算数据的香农熵 (Shannon entropy).
///
/// 衡量数据的随机性/信息密度：`H = -sum(p * log2(p))`.
///
/// 返回值：通常在 `0.0..=8.0` bits 之间。若数据为空则返回 `0.0`。
pub fn entropy<T: AsRef<[u8]>>(data: T) -> f64 {
    let bytes = data.as_ref();
    if bytes.is_empty() {
        return 0.0;
    }

    let counts = count_bytes(bytes);
    let len_f64 = bytes.len() as f64;

    counts
        .iter()
        .copied()
        .filter(|&c| c > 0)
        .map(|c| {
            let p = (c as f64) / len_f64;
            -p * p.log2()
        })
        .sum()
}

/// 计算标准的度量香农熵 (Metric Shannon entropy).
///
/// 将香农熵按 `8-bit` 最大 theoretical 熵 (`8.0` bits) 进行归一化。
///
/// 返回值：通常在 `0.0..=1.0` 之间。
pub fn metric_entropy<T: AsRef<[u8]>>(data: T) -> f64 {
    let bytes = data.as_ref();
    if bytes.is_empty() {
        return 0.0;
    }
    entropy(bytes) / 8.0
}

/// 规范化缩放的香农熵 (Normalized Shannon entropy)。
///
/// 完全随机数据的返回值紧密分布在 `1.0` 周围。
///
/// 返回值：通常在 `0.0..=1.0` 之间。有些故意编写的数据会导致结果超过 `1.0`。
pub fn normalized_metric_entropy<T: AsRef<[u8]>>(data: T) -> f64 {
    let bytes = data.as_ref();
    let n = bytes.len();
    if n == 0 {
        return 0.0;
    }
    if n <= 999 {
        scaled_metric_entropy_small_n(bytes)
    } else {
        scaled_metric_entropy_large_n(bytes)
    }
}

fn scaled_metric_entropy_small_n<T: AsRef<[u8]>>(data: T) -> f64 {
    let bytes = data.as_ref();
    let n = bytes.len();
    let h = entropy(bytes);
    let divisor = SMALL_N_ENTROPY.get(n).copied().unwrap_or(0.0);
    if divisor <= 0.0 {
        0.0
    } else {
        (h / divisor).min(1.0)
    }
}

fn scaled_metric_entropy_large_n<T: AsRef<[u8]>>(data: T) -> f64 {
    const BEST_A: f64 = 241.73498474635633215257;
    const BEST_B: f64 = 2.44558154522179371781;
    const BEST_K: f64 = 1.03150230468322812172;
    const C: f64 = 8.0;
    fn fit_model(n: f64, a: f64, b: f64, k: f64) -> f64 {
        let base = n + b;
        let decay = base.powf(-k);
        C - a * decay
    }
    let bytes = data.as_ref();
    let n = bytes.len() as f64;
    let h = entropy(bytes);
    let divisor = fit_model(n, BEST_A, BEST_B, BEST_K);
    if divisor <= 0.0 {
        0.0
    } else {
        (h / divisor).min(1.0)
    }
}
