"""
Min entropy algorithm module
"""

from collections.abc import Sequence

def measure_entropy(data: str |Sequence[int]) -> float:
    """
    计算数据的最小熵 (Min-entropy: H_infinity).
    
    最小熵表示预测最可能出现的字节所需的比特数：`H_inf = -log2(max(P))`.
    
    返回值：通常在 `0.0..=8.0` bits 之间。若数据为空则返回 `0.0`。
    """
