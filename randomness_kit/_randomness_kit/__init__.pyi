"""
`randomness_kit` root module
"""

from _typeshed import Incomplete

def ping(x: int) -> int:
    """
    test function
    
    return x * 2
    """

def profile() -> str:
    """
    获取本模块的构建模式
    
    返回值: `"debug" | "release"`
    """

def version() -> str:
    """
    package version
    """

def __getattr__(name: str) -> Incomplete: ...
