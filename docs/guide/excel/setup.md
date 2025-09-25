# Excel Add-In Configuration

## Prerequisites
- **Windows** (Excel-DNA does not natively support Linux/macOS).
- **.NET SDK 6.0** installed.

## Steps

### 1. Compile the add-in
```bash
cd excel/AddIn
dotnet build --configuration Release
```

### 2. Install the `.xll` file
1. Copy `excel/AddIn/bin/Release/TonAddIn.xll` to an accessible folder.
2. In Excel:
    - Go to `File > Options > Add-ins > Browse`.
    - Select `TonAddIn.xll`.

### 3. Verify the installation
- A new category **"Ton Add-In"** appears in Excel functions.
- Test with:
  ```excel
  =RustAdd(1, 1)  # Should return 2
  ```

## Troubleshooting
| Issue                     | Solution                                  |
|---------------------------|-------------------------------------------|
| `.xll` not loading        | Check the architecture (64-bit).          |
| Function not found        | Ensure `ftp_core_bindings_c.dll` is in `excel/Interop/`. |
```