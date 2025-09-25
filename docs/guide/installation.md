# Installation

This guide explains how to install and configure **FTP_CALCULATOR** for Rust, Python, and Excel.

---

## Prerequisites

### For all environments
- **Git**: To clone the repository.
  ```bash
  sudo apt install git  # Linux (Debian/Ubuntu)
  ```
- **Rust**: To compile the crates.
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```

### For Rust and Python
- **Python 3.8+**:
  ```bash
  sudo apt install python3 python3-pip  # Linux
  ```
- **Build tools**:
  ```bash
  sudo apt install build-essential  # Linux
  ```

### For Excel (Windows only)
- **.NET SDK 6.0**:
  Download from [dotnet.microsoft.com](https://dotnet.microsoft.com/download).
- **Excel 2016 or later** (for the add-in).

---

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/ton_org/FTP_CALCULATOR.git
cd FTP_CALCULATOR
```

### 2. Install Rust dependencies
```bash
cargo install cbindgen  # To generate C headers
```

### 3. Install Python dependencies
```bash
pip install -r python/requirements.txt
```

### 4. Compile Rust bindings
#### For Python (PyO3)
```bash
cd crates/ftp_core_bindings_pyo3
maturin develop  # Installs the Python module locally
```

#### For Excel (C)
```bash
cd crates/ftp_core_bindings_c
cargo build --release
```

### 5. Configure the Excel add-in (Windows)
1. **Compile the C# project**:
   ```bash
   cd excel/AddIn
   dotnet build
   ```
2. **Copy the `.xll` file**:
    - The `TonAddIn.xll` file is generated in `excel/AddIn/bin/Debug/`.
    - Copy it to an accessible folder (e.g., `~/Desktop/`).

3. **Load in Excel**:
    - Open Excel.
    - Go to `File > Options > Add-ins > Browse`.
    - Select `TonAddIn.xll`.

---

## Verification
- **Test the Python bindings**:
  ```bash
  python -c "from ftp_core_python import add; print(add(2, 3))"  # Should display 5
  ```
- **Test the Excel add-in**:
  In an Excel cell, type:
  ```excel
  =RustAdd(2, 3)  # Should return 5
  ```