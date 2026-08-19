use pyo3::{FromPyObject, IntoPyObject};
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};
use rayon::slice::ParallelSliceMut as _;

use crate::strings::SmartString;
use std::collections::HashMap;
use std::hash::Hash;

/// 代表片段在原数据中的半开区间位置 `[start, end)`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPyObject, FromPyObject)]
pub struct FragmentInterval {
    /// 起始索引 (包含)
    pub start: usize,
    /// 结束索引 (不包含)
    pub end: usize,
}

impl FragmentInterval {
    /// 创建一个新的位置区间。
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// 代表在输入数据中找到的一个高频片段及其所有出现的位置。
#[derive(Debug, Clone, PartialEq, Eq, IntoPyObject, FromPyObject)]
pub struct Fragment {
    /// 片段在结果列表中的序号 (从 0 开始)
    pub id: usize,
    /// 该片段在输入数据中所有互不重叠出现的区间列表
    pub occurrences: Vec<FragmentInterval>,
}

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
/// - `data`: 输入的字符串或字节数据，支持 `&str`、`String`、`&[u8]`、`Vec<u8>` 等。
/// - `count`: 最多返回的片段数量 (例如 `100`)。
/// - `times`: 每个片段最少需要的非重叠出现次数 (例如 `2`)。
/// - `global_non_overlap`:
///   - `false`: 仅要求**同一个片段**内部的多次出现互不重叠。
///   - `true`: 额外要求**不同片段之间**占据的位置也互不重叠（按优先级保留先选中的片段）。
pub fn find_non_overlapping_fragments<'a>(
    data: impl Into<SmartString<'a>>,
    count: usize,
    times: usize,
    global_non_overlap: bool,
) -> Vec<Fragment> {
    match data.into() {
        SmartString::Bytes(bytes) => {
            find_non_overlapping_fragments_generic(bytes, count, times, global_non_overlap)
        }
        SmartString::Str(s) => {
            let chars: Vec<char> = s.chars().collect();
            find_non_overlapping_fragments_generic(&chars, count, times, global_non_overlap)
        }
    }
}

fn find_non_overlapping_fragments_generic<T>(
    data: &[T],
    count: usize,
    times: usize,
    global_non_overlap: bool,
) -> Vec<Fragment>
where
    T: Eq + Hash + Sync + Send,
{
    if data.is_empty() || times < 1 || count < 1 {
        return Vec::new();
    }
    let n = data.len();
    let max_len = n / times;
    if max_len == 0 {
        return Vec::new();
    }
    type Candidate = (usize, usize, usize, Vec<FragmentInterval>);
    let mut candidates: Vec<Candidate> = (1..=max_len)
        .into_par_iter()
        .flat_map(|length| {
            let mut slice_positions: HashMap<&[T], Vec<usize>> = HashMap::new();
            for i in 0..=(n - length) {
                slice_positions
                    .entry(&data[i..i + length])
                    .or_default()
                    .push(i);
            }
            let mut local_candidates = Vec::new();
            for (_sub, positions) in slice_positions {
                if positions.len() < times {
                    continue;
                }
                let mut occurrences = Vec::with_capacity(positions.len());
                let mut last_end = 0;
                for &pos in &positions {
                    if pos >= last_end {
                        occurrences.push(FragmentInterval::new(pos, pos + length));
                        last_end = pos + length;
                    }
                }
                if occurrences.len() >= times {
                    let total_coverage = length * occurrences.len();
                    local_candidates.push((total_coverage, length, occurrences.len(), occurrences));
                }
            }
            local_candidates
        })
        .collect();
    candidates.par_sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| b.1.cmp(&a.1))
            .then_with(|| b.2.cmp(&a.2))
    });
    let mut result = Vec::with_capacity(count.min(candidates.len()));
    let mut used_mask = if global_non_overlap {
        vec![false; n]
    } else {
        Vec::new()
    };
    let mut idx = 0;
    for cand in candidates {
        if idx >= count {
            break;
        }
        let occurrences = cand.3;
        if global_non_overlap {
            let has_overlap = occurrences
                .iter()
                .any(|occ| used_mask[occ.start..occ.end].iter().any(|&used| used));
            if has_overlap {
                continue;
            }
            for occ in &occurrences {
                used_mask[occ.start..occ.end].fill(true);
            }
        }
        result.push(Fragment {
            id: idx,
            occurrences,
        });
        idx += 1;
    }
    result
}
