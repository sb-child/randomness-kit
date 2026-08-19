"""
Shannon entropy algorithm module
"""

from collections.abc import Sequence

def measure_entropy(data: str |Sequence[int]) -> float:
    """
    计算数据的香农熵 (Shannon entropy).
    
    衡量数据的随机性/信息密度：`H = -sum(p * log2(p))`.
    
    返回值：通常在 `0.0..=8.0` bits 之间。若数据为空则返回 `0.0`。
    """

def measure_metric_entropy(data: str |Sequence[int]) -> float:
    """
    计算标准的度量香农熵 (Metric Shannon entropy).
    
    将香农熵按 `8-bit` 最大 theoretical 熵 (`8.0` bits) 进行归一化。
    
    返回值：通常在 `0.0..=1.0` 之间。
    """

def measure_normalized_metric_entropy(data: str |Sequence[int]) -> float:
    """
    规范化缩放的香农熵 (Normalized Shannon entropy)。
    
    完全随机数据的返回值紧密分布在 `1.0` 周围。
    
    返回值：通常在 `0.0..=1.0` 之间。有些故意编写的数据会导致结果超过 `1.0`。
    """
