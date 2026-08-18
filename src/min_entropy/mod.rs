use crate::util::count_bytes;

/// 计算数据的最小熵 (Min-entropy: H_infinity).
///
/// 最小熵表示预测最可能出现的字节所需的比特数：`H_inf = -log2(max(P))`.
///
/// 返回值：通常在 `0.0..=8.0` bits 之间。若数据为空则返回 `0.0`。
pub fn entropy<T: AsRef<[u8]>>(data: T) -> f64 {
    let bytes = data.as_ref();
    if bytes.is_empty() {
        return 0.0;
    }
    let counts = count_bytes(bytes);
    let max_count = *counts.iter().max().unwrap_or(&0);
    if max_count == 0 {
        return 0.0;
    }
    let max_p = (max_count as f64) / (bytes.len() as f64);
    -max_p.log2()
}
