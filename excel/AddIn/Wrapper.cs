using System;
using System.Runtime.InteropServices;
using ExcelDna.Integration;
using System.Text;

public static class RustFunctions
{
    private const string RustLibraryName = "ftp_core_bindings_c.dll";

    [DllImport(RustLibraryName, EntryPoint = "add")]
    private static extern int Add(int a, int b);

    [ExcelFunction(
        Description = "Additionne deux entiers via Rust",
        Category = "FTP Calculator"
    )]
    public static object RustAdd(
        [ExcelArgument(Name = "a", Description = "Premier nombre")] int a,
        [ExcelArgument(Name = "b", Description = "Deuxième nombre")] int b
    )
    {
        try
        {
            return Add(a, b);
        }
        catch (Exception ex)
        {
            return ExcelError.ExcelErrorValue;
        }
    }

    // Add more FTP-specific functions here
}