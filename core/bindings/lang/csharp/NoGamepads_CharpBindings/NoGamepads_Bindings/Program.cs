using CppSharp;
using CppSharp.AST;
using CppSharp.AST.Extensions;
using CppSharp.Generators;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;

namespace NoGamepads_Bindings;

internal static class Program
{
    public static void Main()
    {
        ConsoleDriver.Run(new NoGamepadsLibrary());
        FixRefProperties();
    }
    
    private static void FixRefProperties()
    {
        var directory = "NoGamepads_Core/Generated";
        var files = Directory.GetFiles(directory, "*.cs", SearchOption.AllDirectories);

        foreach (var file in files)
        {
            var code = File.ReadAllText(file);
            var tree = CSharpSyntaxTree.ParseText(code);
            var root = tree.GetRoot();

            var newRoot = root.ReplaceNodes(
                root.DescendantNodes().OfType<PropertyDeclarationSyntax>(),
                (prop, _) =>
                {
                    // internal ref FfiXXXUnion.__Internal __Instance => ref __instance;
                    if (prop.Identifier.Text != "__Instance")
                        return prop;

                    if (prop.Type is RefTypeSyntax refType &&
                        prop.ExpressionBody?.Expression is RefExpressionSyntax refExpr &&
                        refType.Type.ToString().EndsWith(".__Internal", StringComparison.Ordinal) &&
                        refExpr.Expression is IdentifierNameSyntax fieldRef &&
                        fieldRef.Identifier.Text == "__instance")
                    {
                        var newType = refType.Type.WithTriviaFrom(refType);
                        var newExpr = fieldRef.WithTriviaFrom(refExpr);

                        return prop
                            .WithType(newType)
                            .WithExpressionBody(SyntaxFactory.ArrowExpressionClause(newExpr))
                            .WithSemicolonToken(prop.SemicolonToken);
                    }

                    return prop;
                });

            if (!newRoot.IsEquivalentTo(root))
            {
                File.WriteAllText(file, newRoot.ToFullString());
            }
        }
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
            
        var module = options.AddModule("nogamepads_c");
        module.OutputNamespace = "NoGamepads_Sharp";
        module.IncludeDirs.Add("NoGamepads_Bindings/Native");
        module.Headers.Add("nogamepads_data.h");
    }
}

