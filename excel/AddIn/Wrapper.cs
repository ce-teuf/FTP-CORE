using System;
using System.Runtime.InteropServices;
using ExcelDna.Integration;

public static class RustFunctions
{
    // Chemin relatif vers la bibliothèque Rust (ajuste selon ton OS)
    private const string RustLibraryName =
        #if WINDOWS
            "ftp_core_bindings_c.dll";
        #else
            "ftp_core_bindings_c.so";
        #endif

    // Import de la fonction Rust 'add'
    [DllImport(RustLibraryName, EntryPoint = "add")]
    public static extern int Add(int a, int b);

    // Import de la fonction Rust 'safe_divide'
    [DllImport(RustLibraryName, EntryPoint = "safe_divide")]
    public static extern int SafeDivide(int numerator, int denominator);

    // Fonction exposée à Excel pour l'addition
    [ExcelFunction(
        Description = "Additionne deux entiers (appelle Rust)",
        Category = "Ton Add-In"
    )]
    public static int RustAdd(
        [ExcelArgument(Name = "a", Description = "Premier nombre")] int a,
        [ExcelArgument(Name = "b", Description = "Deuxième nombre")] int b
    )
    {
        return Add(a, b);
    }

    // Fonction exposée à Excel pour la division
    [ExcelFunction(
        Description = "Divise deux entiers (appelle Rust)",
        Category = "Ton Add-In"
    )]
    public static object RustDivide(
        [ExcelArgument(Name = "numerator", Description = "Numérateur")] int numerator,
        [ExcelArgument(Name = "denominator", Description = "Dénominateur")] int denominator
    )
    {
        int result = SafeDivide(numerator, denominator);
        if (result == -1)
        {
            return ExcelError.ExcelErrorDiv0;  // Retourne #DIV/0! en cas d'erreur
        }
        return result;
    }
}
