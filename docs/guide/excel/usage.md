# Advanced Usage with Excel

## Available Functions
| Function          | Description                     | Excel Syntax          |
|-------------------|---------------------------------|-----------------------|
| `RustAdd`         | Adds two integers.              | `=RustAdd(a, b)`      |
| `RustDivide`      | Divides two integers.           | `=RustDivide(a, b)`   |
| `RustGreet`       | Returns a personalized message. | `=RustGreet("Name")`  |

## Examples

### 1. Series calculations
```excel
=RustAdd(A1, B1)  # Adds the values of cells A1 and B1
```

### 2. Error handling
```excel
=IFERROR(RustDivide(A1, B1), "Division error")
```

### 3. Integration with other formulas
```excel
=SUM(RustAdd(A1, B1), RustAdd(C1, D1))  # Sum of two additions
```