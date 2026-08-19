from collections.abc import Sequence

def find_non_overlapping_fragments(data: str |Sequence[int], count: int, times: int, global_non_overlap: bool) -> "list[dict[str, int | list[dict[str, int]]]]":
    """
    在字符串或字节序列中寻找出现次数至少为 `t` 次的不重叠片段。
    
    本函数会穷举输入数据中所有可能的连续子片段，并计算每个片段在数据中**不重叠**出现的次数。
    最终按照 `(总覆盖长度, 片段长度, 出现次数)` 降序排列并返回前 `count` 个片段。
    
    ## 索引规则
    - 传入 **字符串** (`&str`, `&String`) 时：返回的区间索引按 **Unicode 字符 (char)** 计数。
    - 传入 **字节序列** (`&[u8]`, `Vec<u8>`) 时：返回的区间索引按 **字节 (byte)** 计数。
    
    ## 参数
    - `data`: 输入的字符串或字节数据。
    - `count`: 最多返回的片段数量 (例如 `100`)。
    - `times`: 每个片段最少需要的非重叠出现次数 (例如 `2`)。
    - `global_non_overlap`:
      - `false`: 仅要求**同一个片段**内部的多次出现互不重叠。
      - `true`: 额外要求**不同片段之间**占据的位置也互不重叠（按优先级保留先选中的片段）。
    
    ## 返回值
    ```python
    list[dict[str, int | list[dict[str, int]]]]
    Example: [{'id': 0, 'occurrences': [{'start': 0, 'end': 17}]}]
    ```
    """
