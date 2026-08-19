use pyo3::prelude::*;
mod min_entropy;
mod pytypes;
mod shannon_entropy;
mod strings;
mod util;

/// `randomness_kit` root module
#[pymodule(name = "_randomness_kit")]
mod randomness_kit {
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::py_shannon_entropy_algo;

    #[pymodule_export]
    use super::py_min_entropy_algo;

    #[pymodule_export]
    use super::pystrings_frag;

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
#[pymodule(name = "shannon_entropy")]
mod py_shannon_entropy_algo {
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
#[pymodule(name = "min_entropy")]
mod py_min_entropy_algo {
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

#[pymodule(name = "strings_frag")]
mod pystrings_frag {
    use crate::pytypes::BytesOrStr;
    use crate::strings::frag::Fragment;
    use crate::strings::{SmartString, frag};
    use pyo3::prelude::*;

    /// 在字符串或字节序列中寻找出现次数至少为 `t` 次的不重叠片段。
    ///
    /// 本函数会穷举输入数据中所有可能的连续子片段，并计算每个片段在数据中**不重叠**出现的次数。
    /// 最终按照 `(总覆盖长度, 片段长度, 出现次数)` 降序排列并返回前 `count` 个片段。
    ///
    /// ## 索引规则
    /// - 传入 **字符串** (`&str`, `&String`) 时：返回的区间索引按 **Unicode 字符 (char)** 计数。
    /// - 传入 **字节序列** (`&[u8]`, `Vec<u8>`) 时：返回的区间索引按 **字节 (byte)** 计数。
    ///
    /// ## 参数
    /// - `data`: 输入的字符串或字节数据。
    /// - `count`: 最多返回的片段数量 (例如 `100`)。
    /// - `times`: 每个片段最少需要的非重叠出现次数 (例如 `2`)。
    /// - `global_non_overlap`:
    ///   - `false`: 仅要求**同一个片段**内部的多次出现互不重叠。
    ///   - `true`: 额外要求**不同片段之间**占据的位置也互不重叠（按优先级保留先选中的片段）。
    ///
    /// ## 返回值
    /// ```python
    /// list[dict[str, int | list[dict[str, int]]]]
    /// Example: [{'id': 0, 'occurrences': [{'start': 0, 'end': 17}]}]
    /// ```
    #[pyfunction]
    #[pyo3(signature = (
        data, count, times, global_non_overlap
    ) -> "list[dict[str, int | list[dict[str, int]]]]")]
    #[pyo3()]
    fn find_non_overlapping_fragments(
        data: BytesOrStr,
        count: usize,
        times: usize,
        global_non_overlap: bool,
    ) -> PyResult<Vec<Fragment>> {
        let d = match &data {
            BytesOrStr::Str(s) => SmartString::from(s),
            BytesOrStr::Bytes(b) => SmartString::from(b),
        };
        let r = frag::find_non_overlapping_fragments(d, count, times, global_non_overlap);
        Ok(r)
    }
}
