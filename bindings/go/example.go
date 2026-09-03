package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	sampleJar, _ := filepath.Abs("../../test_sample.jar")
	if _, err := os.Stat(sampleJar); err != nil {
		sampleJar = "test_sample.jar"
	}

	fmt.Println("========================================")
	fmt.Printf(" JPEF Go Binding v%s\n", Version())
	fmt.Println("========================================\n")

	// 1. Inspect
	fmt.Printf("[1] Inspecting JAR: %s\n", sampleJar)
	info, err := Inspect(sampleJar)
	if err == nil {
		fmt.Printf("  Main-Class:   %s\n", info.MainClass)
		fmt.Printf("  Min Java:     Java %d+\n", info.MinJavaVersion)
		fmt.Printf("  Runnable:     %v\n\n", info.IsRunnable)
	}

	// 2. Convert
	fmt.Println("[2] Converting to .exe, .elf, and .app via Go...")
	outDir, _ := filepath.Abs("../../dist_go")
	res, err := Convert(ConvertOptions{
		JarPath:   sampleJar,
		OutputDir: outDir,
		AppName:   "SampleAppGo",
		Targets:   TargetAll,
		IsGui:     false,
		MinHeap:   "128m",
		MaxHeap:   "512m",
		JvmArgs:   []string{"-Dfile.encoding=UTF-8"},
	})

	if err != nil {
		fmt.Fprintf(os.Stderr, "[FAILED] %v\n", err)
		os.Exit(1)
	}

	if res.Success {
		fmt.Printf("\n[SUCCESS] Generated %d artifact(s) in %.2fs:\n", len(res.Artifacts), res.ElapsedSeconds)
		for _, art := range res.Artifacts {
			fmt.Printf("  - [%s] %s (%.2f MB)\n", art.Platform, art.Path, float64(art.SizeBytes)/(1024.0*1024.0))
		}
	} else {
		fmt.Fprintf(os.Stderr, "[FAILED] %s\n", res.Errors)
		os.Exit(1)
	}
}
