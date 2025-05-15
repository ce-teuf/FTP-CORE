from typing import Optional, Dict, List, Tuple
import numpy as np
import pandas as pd

class PyFtpResult:
    """Container for FTP analysis results with computed matrices.
    
    Attributes:
        input_outstanding: Initial outstanding amounts (NxM matrix)
        input_profiles: Time profiles matrix (NxM)
        input_rate: Initial rates matrix (NxM)
        stock_amort: Amortized stock (optional matrix)
        stock_instal: Instalment stock (optional matrix)
        varstock_amort: New production amortized (optional matrix)
        varstock_instal: New production instalment (optional matrix)
        ftp_rate: FTP rates (optional matrix)
        ftp_int: FTP interest amounts (optional matrix)
        market_rate: Market rates (optional matrix)
    """
    
    # ----- Core Fields -----
    input_outstanding: np.ndarray[np.float64]
    input_profiles: np.ndarray[np.float64]
    input_rate: np.ndarray[np.float64]
    stock_amort: Optional[np.ndarray[np.float64]]
    stock_instal: Optional[np.ndarray[np.float64]]
    varstock_amort: Optional[np.ndarray[np.float64]]
    varstock_instal: Optional[np.ndarray[np.float64]]
    ftp_rate: Optional[np.ndarray[np.float64]]
    ftp_int: Optional[np.ndarray[np.float64]]
    market_rate: Optional[np.ndarray[np.float64]]

    # ----- Constructor -----
    def __init__(
        self,
        input_outstanding: np.ndarray[np.float64],
        input_profiles: np.ndarray[np.float64],
        input_rate: np.ndarray[np.float64],
    ) -> None:
        """Initialize FTP result container.
        
        Args:
            input_outstanding: Matrix of outstanding amounts (shape NxM)
            input_profiles: Matrix of time profiles (shape NxM)
            input_rate: Matrix of initial rates (shape NxM)
        
        Raises:
            ValueError: If matrices have inconsistent shapes
        """
        ...
    
    # ----- Core Methods -----
    
    
    # ----- Special Methods -----
    def __repr__(self) -> str:
        """Official string representation showing matrix shapes."""
        ...
    
    def __str__(self) -> str:
        """User-friendly string representation."""
        ...
    
    def __dir__(self) -> List[str]:
        """List of available attributes for autocompletion."""
        ...

# ----- Module-Level Functions -----
def process_arrays(
    outstanding: np.ndarray[np.float64],
    profiles: np.ndarray[np.float64],
    rates: np.ndarray[np.float64],
) -> PyFtpResult:
    """Main processing function creating FTP results.
    
    Args:
        outstanding: Input outstanding amounts matrix
        profiles: Time profiles matrix
        rates: Initial rates matrix
    
    Returns:
        Fully populated FtpResult object
    
    Raises:
        ValueError: If input matrices have mismatched shapes
    """
    ...

