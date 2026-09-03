# Java Portable Executable Format (JPEF) - Rust Core & Multi-Language SDK

**JPEF**, Java Archive (`.jar`) dosyalarını yerel (native) **Windows (`.exe`)**, **Linux (`.elf`)** ve **macOS (`.app`)** çalıştırılabilir formatlarına dönüştüren; çekirdeği **Rust** ile baştan yazılmış yüksek performanslı, bellek güvenli ve çok dilli (multi-language) bir platform paketleme aracıdır.

---

## ⚡ Rust Çekirdeği ve Çoklu Dil Desteği

JPEF çekirdeği sıfırdan Rust ile geliştirilmiş olup; hem tek başına çalışan bir CLI ikilisi (`jpef.exe`), hem de evrensel C-ABI dinamik kütüphanesi (`jpef.dll` / `libjpef.so` / `libjpef.dylib`) üretir. Bu sayede aklınıza gelebilecek tüm modern dillerde birinci sınıf uyumluluk kütüphaneleri (bindings) sunar:

- 🦀 **Rust**: Yerel Rust kütüphanesi ve CLI (`cargo add jpef`)
- 🇨 **C**: Saf C99/C11 başlık dosyası (`jpef.h`)
- ➕ **C++**: Modern C++17/C++20 RAII sarmalayıcısı (`jpef.hpp`)
- 📜 **TypeScript & JavaScript**: Node.js, Bun ve Deno için Koffi FFI sarmalayıcısı ve `.d.ts` tip tanımları
- 🔷 **C# / .NET**: .NET 8/9/10 için yüksek performanslı P/Invoke sarmalayıcısı (`Jpef.cs`)
- 🐹 **Go**: Saf Go (`syscall.NewLazyDLL`) sıfır bağımlılıklı binding (`jpef.go`)
- 🐍 **Python**: `ctypes` ile Rust çekirdeğine doğrudan bağlanan ultra hızlı sarmalayıcı (`jpef_rs.py`)
- ☕ **Java**: Java 21 Foreign Function & Memory API (Panama) entegrasyonu (`Jpef.java`)

---

## 🚀 CLI Kullanımı (Rust Binary)

### Derleme
```bash
cargo build --release
```
Oluşan dosyalar:
- `target/release/jpef.exe` (Rust CLI)
- `target/release/jpef.dll` (C-ABI Kütüphanesi)

### Komutlar

#### 1. JAR Analizi (`inspect`)
```bash
./target/release/jpef inspect MyApp.jar
```

#### 2. Tüm Platformlara Dönüştürme (`convert`)
```bash
./target/release/jpef convert MyApp.jar -t exe,elf,app -o dist/ --icon logo.png
```

#### 3. Konsol ve Bellek Ayarlarıyla Dönüştürme
```bash
./target/release/jpef convert MyApp.jar -t exe,elf,app -o dist/ \
  --name "SuperApp" \
  --console \
  --min-heap 256m \
  --max-heap 2048m \
  --jvm-arg "-Dfile.encoding=UTF-8"
```

---

## 🌐 Çoklu Dil Uyumluluk Kütüphaneleri (Language Bindings)

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
        printf("Başarılı! %zu dosya üretildi.\n", jpef_result_get_artifact_count(res));
    }
    jpef_result_free(res);
    jpef_config_free(cfg);
}
```
**Derleme:**
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
}
```
**Derleme:**
```bash
g++ -std=c++17 -static-libgcc -static-libstdc++ -Ibindings/c/include -Ibindings/cpp/include bindings/cpp/example.cpp target/release/jpef.dll -o example_cpp.exe
```

---

### 3. TypeScript & JavaScript (Node.js) (`bindings/typescript/`)
```typescript
import jpef from './bindings/typescript';

// JAR İnceleme
const info = jpef.inspect("MyApp.jar");
console.log(`Main-Class: ${info.mainClass}, Min Java: ${info.minJavaVersion}`);

// Dönüştürme
const result = jpef.convert({
  jarPath: "MyApp.jar",
  outputDir: "dist",
  appName: "SuperApp",
  targets: ["exe", "elf", "app"],
  isGui: true,
  minHeap: "256m",
  maxHeap: "1024m"
});

console.log(`Oluşturuldu: ${result.artifacts.length} dosya`);
```
**Çalıştırma:**
```bash
node bindings/typescript/example.js
```

---

### 4. C# / .NET (`bindings/csharp/`)
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
**Çalıştırma:**
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
**Çalıştırma:**
```bash
go run . (bindings/go dizininde)
```

---

### 6. Python (`bindings/python/`)
```python
import jpef_rs

# JAR Analizi
info = jpef_rs.inspect("MyApp.jar")
print(f"Main-Class: {info.main_class}, Java: {info.min_java_version}+")

# Dönüştürme
res = jpef_rs.convert(
    jar_path="MyApp.jar",
    output_dir="dist",
    app_name="PyApp",
    targets=["exe", "elf", "app"]
)
print(f"Başarılı: {len(res.artifacts)} dosya")
```
**Çalıştırma:**
```bash
python bindings/python/example.py
```

---

### 7. Java (`bindings/java/`)
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
        System.out.println("Başarılı: " + res.success());
    }
}
```
**Derleme ve Çalıştırma (Java 21+):**
```bash
javac --enable-preview --source 21 bindings/java/jpef/Jpef.java bindings/java/Example.java
java --enable-preview --enable-native-access=ALL-UNNAMED -cp bindings/java Example
```

---

## 🧪 Testleri Çalıştırma

### Rust Testleri
```bash
cargo test
```

### Çoklu Dil Entegrasyon Testleri
Her dildeki `example` uygulamaları derlenip çalıştırılarak test edilmiştir.
