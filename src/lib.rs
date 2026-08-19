use pyo3::prelude::*;
mod min_entropy;
mod pytypes;
mod shannon_entropy;
mod util;

/// `randomness_kit` root module
#[pymodule(name = "_randomness_kit")]
mod randomness_kit {
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::shannon_entropy_algo;

    #[pymodule_export]
    use super::min_entropy_algo;

    /// test function
    ///
    /// return x * 2
    #[pyfunction]
    fn ping(x: usize) -> usize {
        x * 2
    }

    /// package version
    #[pyfunction]
    fn version() -> PyResult<String> {
        let v = env!("CARGO_PKG_VERSION");
        Ok(v.to_owned())
    }

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        // m.add("double2", m.getattr("double")?)
        // println!("randomness_kit is imported.");
        Ok(())
    }
}

/// Shannon entropy algorithm module
#[pymodule]
mod shannon_entropy_algo {
    use crate::{pytypes, shannon_entropy};
    use pyo3::prelude::*;
    use pytypes::BytesOrStr;

    /// 计算数据的香农熵 (Shannon entropy).
    ///
    /// 衡量数据的随机性/信息密度：`H = -sum(p * log2(p))`.
    ///
    /// 返回值：通常在 `0.0..=8.0` bits 之间。若数据为空则返回 `0.0`。
    #[pyfunction]
    fn measure_entropy(data: BytesOrStr) -> PyResult<f64> {
        let res = match data {
            BytesOrStr::Str(s) => shannon_entropy::entropy(s),
            BytesOrStr::Bytes(b) => shannon_entropy::entropy(b),
        };
        Ok(res)
    }

    /// 计算标准的度量香农熵 (Metric Shannon entropy).
    ///
    /// 将香农熵按 `8-bit` 最大 theoretical 熵 (`8.0` bits) 进行归一化。
    ///
    /// 返回值：通常在 `0.0..=1.0` 之间。
    #[pyfunction]
    fn measure_metric_entropy(data: BytesOrStr) -> PyResult<f64> {
        let res = match data {
            BytesOrStr::Str(s) => shannon_entropy::metric_entropy(s),
            BytesOrStr::Bytes(b) => shannon_entropy::metric_entropy(b),
        };
        Ok(res)
    }

    /// 规范化缩放的香农熵 (Normalized Shannon entropy)。
    ///
    /// 完全随机数据的返回值紧密分布在 `1.0` 周围。
    ///
    /// 返回值：通常在 `0.0..=1.0` 之间。有些故意编写的数据会导致结果超过 `1.0`。
    #[pyfunction]
    fn measure_normalized_metric_entropy(data: BytesOrStr) -> PyResult<f64> {
        let res = match data {
            BytesOrStr::Str(s) => shannon_entropy::normalized_metric_entropy(s),
            BytesOrStr::Bytes(b) => shannon_entropy::normalized_metric_entropy(b),
        };
        Ok(res)
    }
}

/// Min entropy algorithm module
#[pymodule]
mod min_entropy_algo {
    use crate::{min_entropy, pytypes};
    use pyo3::prelude::*;
    use pytypes::BytesOrStr;

    /// 计算数据的最小熵 (Min-entropy: H_infinity).
    ///
    /// 最小熵表示预测最可能出现的字节所需的比特数：`H_inf = -log2(max(P))`.
    ///
    /// 返回值：通常在 `0.0..=8.0` bits 之间。若数据为空则返回 `0.0`。
    #[pyfunction]
    fn measure_entropy(data: BytesOrStr) -> PyResult<f64> {
        let res = match data {
            BytesOrStr::Str(s) => min_entropy::entropy(s),
            BytesOrStr::Bytes(b) => min_entropy::entropy(b),
        };
        Ok(res)
    }
}
