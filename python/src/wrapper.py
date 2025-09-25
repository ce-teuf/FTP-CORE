from typing import Union
from . import _core  # Module généré par PyO3 (bindings Rust)

class MonWrapper:
    """
    A high-level wrapper around the Rust core functions.

    Examples:
        >>> wrapper = MonWrapper()
        >>> wrapper.fonction_etendue(5)
        15
    """

    def __init__(self):
        """Initialize the wrapper."""
        pass

    def fonction_etendue(self, x: int) -> int:
        """
        Applies an extended transformation to `x`.

        Args:
            x: An integer input.

        Returns:
            The transformed value.

        Raises:
            ValueError: If `x` is negative.
        """
        if x < 0:
            raise ValueError("x must be positive")
        return _core.add(x, 10)  # Appelle la fonction Rust via les bindings

def add(a: int, b: int) -> int:
    """
    Adds two integers using the Rust core.

    Args:
        a: First integer.
        b: Second integer.

    Returns:
        The sum of `a` and `b`.
    """
    return _core.add(a, b)

def safe_divide(numerator: int, denominator: int) -> Union[int, float]:
    """
    Safely divides two integers.

    Args:
        numerator: The numerator.
        denominator: The denominator.

    Returns:
        The result of the division, or `float('inf')` if denominator is zero.

    Examples:
        >>> safe_divide(10, 2)
        5.0
        >>> safe_divide(10, 0)
        inf
    """
    if denominator == 0:
        return float('inf')
    return numerator / denominator
