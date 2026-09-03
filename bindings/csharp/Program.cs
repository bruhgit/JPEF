using System;
using System.IO;
using Jpef;

namespace JpefApp
{
    class Program
    {
        static void Main(string[] args)
        {
            string sampleJar = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "../../../../../test_sample.jar"));
            if (!File.Exists(sampleJar))
            {
                sampleJar = Path.GetFullPath("test_sample.jar");
            }

            Console.WriteLine("========================================");
            Console.WriteLine($" JPEF C# / .NET Binding v{JpefClient.Version()}");
            Console.WriteLine("========================================\n");

            // 1. Inspect
            Console.WriteLine($"[1] Inspecting JAR: {sampleJar}");
            var info = JpefClient.Inspect(sampleJar);
            Console.WriteLine($"  Main-Class:   {info.MainClass}");
            Console.WriteLine($"  Min Java:     Java {info.MinJavaVersion}+");
            Console.WriteLine($"  Runnable:     {(info.IsRunnable ? "Yes" : "No")}\n");

            // 2. Convert
            Console.WriteLine("[2] Converting to .exe, .elf, and .app via C# / .NET...");
            string outDir = Path.GetFullPath(Path.Combine(AppContext.BaseDirectory, "../../../../../dist_csharp"));
            var result = JpefClient.Convert(
                jarPath: sampleJar,
                outputDir: outDir,
                appName: "SampleAppCSharp",
                targets: TargetPlatform.All,
                isGui: false,
                minHeap: "128m",
                maxHeap: "512m",
                jvmArgs: new[] { "-Dfile.encoding=UTF-8" }
            );

            if (result.Success)
            {
                Console.WriteLine($"\n[SUCCESS] Generated {result.Artifacts.Count} artifact(s) in {result.ElapsedSeconds:F2}s:");
                foreach (var art in result.Artifacts)
                {
                    Console.WriteLine($"  - [{art.Platform}] {art.Path} ({art.SizeBytes / (1024.0 * 1024.0):F2} MB)");
                }
            }
            else
            {
                Console.Error.WriteLine($"\n[FAILED] Conversion failed: {result.Errors}");
                Environment.Exit(1);
            }
        }
    }
}
