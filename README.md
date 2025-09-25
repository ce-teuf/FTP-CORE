# FTP Calculator

A high-performance Funds Transfer Pricing calculator built in Rust with multi-language bindings.

## Architecture
ftp_core/ # Rust core library
ftp_core_bindings_c/ # C bindings for Excel
ftp_core_bindings_pyo3/ # Python bindings
excel/ # Excel add-in (C#)
python/ # Python wrapper


## Quick Start

```bash
# Build everything
make all

# Test
make test

# Build documentation
make docs
```
Full documentation available at: https://your-username.github.io/FTP_CALCULATOR

License
MIT License


## Additional Recommendations:

1. **Add proper error handling** in all Rust functions
2. **Implement comprehensive tests** for the Excel add-in
3. **Add validation** for matrix dimensions in FTP calculations
4. **Create example files** for each binding type
5. **Add logging** for better debugging

The main structural issues are the missing documentation content and some configuration problems in the bindings. The core Rust logic appears solid, but the integration points need attention.


