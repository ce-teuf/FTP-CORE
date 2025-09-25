# Quickstart

Get started quickly with **FTP_CALCULATOR** in Rust, Python, and Excel.

---

## With Python

### 1. Import the module
```python
from ftp_core_python import add, safe_divide
```

### 2. Use the functions
```python
# Addition
result = add(10, 20)
print(result)  # Prints 30

# Safe division
result = safe_divide(10, 2)
print(result)  # Prints 5

# Error handling
result = safe_divide(10, 0)
print(result)  # Prints -1 (error)
```

### 3. Advanced example
```python
from ftp_core_python import MonWrapper

wrapper = MonWrapper()
print(wrapper.fonction_etendue(5))  # Prints 15 (5 + 10)
```

---

## With Excel

### 1. Load the add-in
- Open Excel and load `TonAddIn.xll` (see [Installation](installation.md)).

### 2. Use the functions
| Excel Function     | Description                     | Example          |
|--------------------|---------------------------------|------------------|
| `=RustAdd(a, b)`   | Adds two integers.              | `=RustAdd(2, 3)` |
| `=RustDivide(a, b)`| Divides two integers.           | `=RustDivide(10, 2)` |

### 3. Example
```excel
=RustAdd(5, 7)       # Returns 12
=RustDivide(100, 4)  # Returns 25
=RustDivide(10, 0)   # Returns #DIV/0!
```

---

## With Rust (directly)

### 1. Add `ftp_core` as a dependency
In your `Cargo.toml`:
```toml
[dependencies]
ftp_core = { path = "../FTP_CALCULATOR/crates/core" }
```

### 2. Use the functions
```rust
use ftp_core::add;

fn main() {
    let result = add(2, 3);
    println!("{}", result);  # Prints 5
}
```

---

## Next Steps
- [Read the technical documentation](rust/core/index.html).
- [Explore Excel tutorials](excel/tutorials/).
```