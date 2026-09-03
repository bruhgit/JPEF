# <p align="center">☕ Java Portable Executable Format (JPEF)</p>

<p align="center">
  <b>High-performance, memory-safe packaging engine written in Rust that converts Java Archive (<code>.jar</code>) files into native Windows (<code>.exe</code>), Linux (<code>.elf</code>), and macOS (<code>.app</code>) executables with universal multi-language bindings.</b>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2021_Edition-dea584?style=for-the-badge&logo=rust&logoColor=black" alt="Rust">
  <img src="https://img.shields.io/badge/Platforms-Windows%20%7C%20Linux%20%7C%20macOS-blue?style=for-the-badge" alt="Platforms">
  <img src="https://img.shields.io/badge/Bindings-C%20%7C%20C%2B%2B%20%7C%20TS%20%7C%20C%23%20%7C%20Go%20%7C%20Py%20%7C%20Java-brightgreen?style=for-the-badge" alt="Bindings">
  <img src="https://img.shields.io/badge/License-MIT-orange?style=for-the-badge" alt="License">
</p>

---

## ⚡ Rust Core & Universal Multi-Language SDK

The **JPEF** core is built from scratch in **Rust** for safety, zero-overhead performance, and fearless concurrency. It compiles simultaneously as:
1. A standalone high-speed CLI tool (`jpef` / `jpef.exe`).
2. A universal C-ABI dynamic shared library (`jpef.dll` / `libjpef.so` / `libjpef.dylib`).

This design provides first-class, idiomatic SDK bindings for every major programming environment:

| Language | Integration Type | Location |
| :--- | :--- | :--- |
| 🦀 **Rust** | Native Crates & CLI (`cargo add jpef`) | `src/` |
| 🇨 **C** | Pure C99/C11 Header API | `bindings/c/include/jpef.h` |
| ➕ **C++** | Modern C++17/C++20 RAII Wrapper | `bindings/cpp/include/jpef.hpp` |
| 📜 **TypeScript & JS** | Fast Koffi FFI wrapper + `.d.ts` types for Node.js, Bun & Deno | `bindings/typescript/` |
| 🔷 **C# / .NET** | High-performance P/Invoke bindings for .NET 8 / 9 / 10 | `bindings/csharp/` |
| 🐹 **Go** | Pure Go zero-dependency bindings (`syscall.NewLazyDLL`) | `bindings/go/` |
| 🐍 **Python** | Native `ctypes` wrapper with type hints | `bindings/python/` |
| ☕ **Java** | Java 21+ Foreign Function & Memory API (Project Panama) | `bindings/java/` |

---

## 🚀 CLI Usage (Rust Binary)

### Building from Source
```bash
cargo build --release
```

Compiled binaries:
* `target/release/jpef.exe` (Standalone CLI)
* `target/release/jpef.dll` (C-ABI Shared Library)

---

### Commands

#### 1. Inspect a JAR File (`inspect`)
Analyzes the JAR manifest, detects `Main-Class`, scans bytecode versions, and checks dependencies:
```bash
./target/release/jpef inspect MyApp.jar
```

#### 2. Convert to Native Executables (`convert`)
Cross-compile a `.jar` into standalone native executables for all platforms:
```bash
./target/release/jpef convert MyApp.jar -t exe,elf,app -o dist/ --icon logo.png
```

#### 3. Advanced Configuration (JVM Args, Heap & Console)
```bash
./target/release/jpef convert MyApp.jar -t exe,elf,app -o dist/ \
  --name "SuperApp" \
  --console \
  --min-heap 256m \
  --max-heap 2048m \
  --jvm-arg "-Dfile.encoding=UTF-8"
```

---

## 🌐 Language Bindings & SDK Examples

### 1. C (`bindings/c/`)
```c
#include "jpef.h"
#include <stdio.h>

int main() {
    JpefConfig *cfg = jpef_config_new();
    jpef_config_set_jar_path(cfg, "MyApp.jar");
    jpef_config_set_output_dir(cfg, "dist");
    jpef_config_set_targets(cfg, JPEF_TARGET_ALL);

    JpefResult *res = jpef_convert(cfg);
    if (jpef_result_is_success(res)) {
        printf("Conversion succeeded! Produced %zu artifacts.\n", jpef_result_get_artifact_count(res));
    }
    jpef_result_free(res);
    jpef_config_free(cfg);
    return 0;
}
```
**Compile:**
```bash
gcc -Ibindings/c/include bindings/c/example.c target/release/jpef.dll -o example_c.exe
```

---

### 2. C++ (`bindings/cpp/`)
```cpp
#include "jpef.hpp"
#include <iostream>

int main() {
    auto config = jpef::BuildConfig()
        .set_jar_path("MyApp.jar")
        .set_output_dir("dist")
        .set_app_name("CoolApp")
        .set_targets(jpef::Target::All);

    auto result = jpef::convert(config);
    if (result.is_success()) {
        for (const auto &art : result.artifacts()) {
            std::cout << art.platform << ": " << art.path << "\n";
        }
    }
    return 0;
}
```
**Compile:**
```bash
g++ -std=c++17 -static-libgcc -static-libstdc++ -Ibindings/c/include -Ibindings/cpp/include bindings/cpp/example.cpp target/release/jpef.dll -o example_cpp.exe
```

---

### 3. TypeScript & JavaScript (Node.js / Bun / Deno) (`bindings/typescript/`)
```typescript
import jpef from './bindings/typescript';

// Inspect JAR metadata
const info = jpef.inspect("MyApp.jar");
console.log(`Main-Class: ${info.mainClass}, Min Java Version: ${info.minJavaVersion}`);

// Convert to native executables
const result = jpef.convert({
  jarPath: "MyApp.jar",
  outputDir: "dist",
  appName: "SuperApp",
  targets: ["exe", "elf", "app"],
  isGui: true,
  minHeap: "256m",
  maxHeap: "1024m"
});

console.log(`Successfully generated ${result.artifacts.length} platform binaries.`);
```
**Run:**
```bash
node bindings/typescript/example.js
```

---

### 4. C# / .NET 8+ (`bindings/csharp/`)
```csharp
using Jpef;

var result = JpefClient.Convert(
    jarPath: "MyApp.jar",
    outputDir: "dist",
    appName: "DotNetApp",
    targets: TargetPlatform.All,
    isGui: true
);

if (result.Success) {
    foreach (var art in result.Artifacts) {
        Console.WriteLine($"{art.Platform}: {art.Path}");
    }
}
```
**Run:**
```bash
dotnet run --project bindings/csharp/JpefApp.csproj
```

---

### 5. Go (`bindings/go/`)
```go
package main

import (
    "fmt"
    jpef "."
)

func main() {
    res, err := jpef.Convert(jpef.ConvertOptions{
        JarPath:   "MyApp.jar",
        OutputDir: "dist",
        AppName:   "GoApp",
        Targets:   jpef.TargetAll,
    })
    if err == nil && res.Success {
        for _, art := range res.Artifacts {
            fmt.Printf("%s: %s\n", art.Platform, art.Path)
        }
    }
}
```
**Run:**
```bash
go run .
```

---

### 6. Python (`bindings/python/`)
```python
import jpef_rs

# Inspect JAR file
info = jpef_rs.inspect("MyApp.jar")
print(f"Main-Class: {info.main_class}, Java Requirement: {info.min_java_version}+")

# Convert to native binaries
res = jpef_rs.convert(
    jar_path="MyApp.jar",
    output_dir="dist",
    app_name="PyApp",
    targets=["exe", "elf", "app"]
)
print(f"Generated {len(res.artifacts)} native executables successfully.")
```
**Run:**
```bash
python bindings/python/example.py
```

---

### 7. Java 21+ Project Panama (`bindings/java/`)
```java
import jpef.Jpef;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        Jpef.ConvertResult res = Jpef.convert(
            "MyApp.jar",
            "dist",
            "JavaApp",
            true,
            List.of("-Dfile.encoding=UTF-8")
        );
        System.out.println("Status: " + res.success());
    }
}
```
**Compile & Run (Java 21+ Foreign Function API):**
```bash
javac --enable-preview --source 21 bindings/java/jpef/Jpef.java bindings/java/Example.java
java --enable-preview --enable-native-access=ALL-UNNAMED -cp bindings/java Example
```

---

## 🧪 Testing

### Rust Core Unit Tests
```bash
cargo test
```

### Multi-Language Integration Tests
Each language directory includes an automated test runner and verification suite confirming FFI stability with `jpef.dll` / `libjpef.so`.

---

## 🤝 Contributing & License

Contributions, bug reports, and optimizations are welcome! Licensed under the [MIT License](LICENSE).

<p align="center">
  Crafted by <b>omerdev</b> (<a href="https://github.com/bruhgit">@bruhgit</a>)
</p>
