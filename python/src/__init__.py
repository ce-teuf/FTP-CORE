"""
FTP_CALCULATOR Python wrapper.

This package provides a Python interface to the FTP_CALCULATOR Rust core,
with additional high-level functionalities.
"""

from .wrapper import add, safe_divide, MonWrapper

__all__ = ["add", "safe_divide", "MonWrapper"]
__version__ = "0.1.0"
