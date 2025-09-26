from typing import List

def batch_process(data: List[int]) -> List[int]:
    """
    Processes a batch of integers using the Rust core.

    Args:
        data: A list of integers.

    Returns:
        A list of transformed integers.
    """
    from .wrapper import add
    return [add(x, 1) for x in data]
