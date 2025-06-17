using CppSharp;
using CppSharp.AST;
using CppSharp.Generators;
using CppSharp.Generators.AST;
using RecordArgABI = CppSharp.Parser.AST.RecordArgABI;

namespace NoGamepads_Bindings;

internal static class Program
{
    public static void Main(string[] args)
    {
        ConsoleDriver.Run(new NoGamepadsLibrary());
    }
}

public class NoGamepadsLibrary : ILibrary
{
    public void Preprocess(Driver driver, ASTContext ctx) { }

    public void Postprocess(Driver driver, ASTContext ctx) { }

    public void SetupPasses(Driver driver) { }

    public void Setup(Driver driver)
    {
        var options = driver.Options;
        options.GeneratorKind = GeneratorKind.CSharp;
        options.OutputDir = "NoGamepads_Core/Generated";
        options.GenerateClassTemplates = false;
            
        var module = options.AddModule("NoGamepads.Native");
        module.OutputNamespace = "NoGamepads_Sharp";
        module.IncludeDirs.Add("NoGamepads_Bindings/Native");
        module.Headers.Add("nogamepads_data.h");
    }
}