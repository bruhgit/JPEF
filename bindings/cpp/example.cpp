/**
 * Modern C++17 example using JPEF C++ library.
 */

#include <iostream>
#include "jpef.hpp"

int main(int argc, char *argv[]) {
    std::filesystem::path jar_path = (argc > 1) ? argv[1] : "sample.jar";

    std::cout << "========================================\n";
    std::cout << " JPEF C++ Binding - Version: " << jpef::version() << "\n";
    std::cout << "========================================\n\n";

    // 1. Inspect JAR
    std::cout << "[1] Inspecting JAR: " << jar_path << "\n";
    auto info = jpef::inspect(jar_path);
    if (info.is_valid()) {
        std::cout << "  Main-Class:   " << (info.main_class().empty() ? "(None)" : info.main_class()) << "\n";
        std::cout << "  Min Java:     Java " << info.min_java_version() << "+\n";
        std::cout << "  Runnable:     " << (info.is_runnable() ? "Yes" : "No") << "\n\n";
    }

    // 2. Build configuration with fluent API
    std::cout << "[2] Configuring conversion with C++ fluent API...\n";
    auto config = jpef::BuildConfig()
        .set_jar_path(jar_path)
        .set_output_dir("dist_cpp")
        .set_app_name("SampleAppCpp")
        .set_version("2.0.0.0")
        .set_company("JPEF C++ Team")
        .set_targets(jpef::Target::Exe | jpef::Target::Elf | jpef::Target::App)
        .set_gui_mode(false)
        .set_jvm_heap("256m", "1024m")
        .add_jvm_arg("-Dfile.encoding=UTF-8");

    // 3. Convert
    std::cout << "[3] Converting...\n";
    auto result = jpef::convert(config);

    if (result.is_success()) {
        std::cout << "\n[SUCCESS] Generated " << result.artifacts().size() 
                  << " artifact(s) in " << result.elapsed_seconds() << "s:\n";
        for (const auto &art : result.artifacts()) {
            std::cout << "  - [" << art.platform << "] " << art.path.string() 
                      << " (" << (static_cast<double>(art.size_bytes) / (1024.0 * 1024.0)) << " MB)\n";
        }
    } else {
        std::cerr << "\n[FAILED] Conversion error: " << result.errors() << "\n";
        return 1;
    }

    return 0;
}
