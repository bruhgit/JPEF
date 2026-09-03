/**
 * @file jpef.hpp
 * @brief Modern C++17 RAII wrapper for JPEF (Java Portable Executable Format)
 */

#ifndef JPEF_HPP
#define JPEF_HPP

#include <cstdint>
#include <filesystem>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "jpef.h"

namespace jpef {

namespace fs = std::filesystem;

enum class Target : uint32_t {
    Exe = JPEF_TARGET_EXE,
    Elf = JPEF_TARGET_ELF,
    App = JPEF_TARGET_APP,
    All = JPEF_TARGET_ALL,
};

inline Target operator|(Target a, Target b) {
    return static_cast<Target>(static_cast<uint32_t>(a) | static_cast<uint32_t>(b));
}

inline bool operator&(Target a, Target b) {
    return (static_cast<uint32_t>(a) & static_cast<uint32_t>(b)) != 0;
}

struct Artifact {
    std::string platform;
    fs::path path;
    uint64_t size_bytes{0};
};

class JarInfo {
public:
    explicit JarInfo(JpefJarInfo *raw) : raw_(raw, jpef_jar_info_free) {}

    [[nodiscard]] bool is_valid() const noexcept { return raw_ != nullptr; }

    [[nodiscard]] std::string main_class() const {
        if (!raw_) return "";
        const char *s = jpef_jar_info_get_main_class(raw_.get());
        return s ? std::string(s) : "";
    }

    [[nodiscard]] uint32_t min_java_version() const noexcept {
        return raw_ ? jpef_jar_info_get_min_java_version(raw_.get()) : 0;
    }

    [[nodiscard]] bool is_runnable() const noexcept {
        return raw_ ? jpef_jar_info_is_runnable(raw_.get()) : false;
    }

    [[nodiscard]] std::string error() const {
        if (!raw_) return "";
        const char *s = jpef_jar_info_get_error(raw_.get());
        return s ? std::string(s) : "";
    }

private:
    std::shared_ptr<JpefJarInfo> raw_;
};

class Result {
public:
    explicit Result(JpefResult *raw) : raw_(raw, jpef_result_free) {}

    [[nodiscard]] bool is_success() const noexcept {
        return raw_ ? jpef_result_is_success(raw_.get()) : false;
    }

    [[nodiscard]] double elapsed_seconds() const noexcept {
        return raw_ ? jpef_result_get_elapsed_seconds(raw_.get()) : 0.0;
    }

    [[nodiscard]] std::string errors() const {
        if (!raw_) return "";
        const char *s = jpef_result_get_errors(raw_.get());
        return s ? std::string(s) : "";
    }

    [[nodiscard]] std::vector<Artifact> artifacts() const {
        std::vector<Artifact> list;
        if (!raw_) return list;
        size_t count = jpef_result_get_artifact_count(raw_.get());
        list.reserve(count);
        for (size_t i = 0; i < count; ++i) {
            const char *p_plat = jpef_result_get_artifact_platform(raw_.get(), i);
            const char *p_path = jpef_result_get_artifact_path(raw_.get(), i);
            uint64_t sz = jpef_result_get_artifact_size(raw_.get(), i);
            list.push_back({
                p_plat ? p_plat : "",
                p_path ? fs::path(p_path) : fs::path(),
                sz
            });
        }
        return list;
    }

private:
    std::shared_ptr<JpefResult> raw_;
};

class BuildConfig {
public:
    BuildConfig() : raw_(jpef_config_new(), jpef_config_free) {}

    BuildConfig &set_jar_path(const fs::path &path) {
        jpef_config_set_jar_path(raw_.get(), path.string().c_str());
        return *this;
    }

    BuildConfig &set_output_dir(const fs::path &dir) {
        jpef_config_set_output_dir(raw_.get(), dir.string().c_str());
        return *this;
    }

    BuildConfig &set_app_name(const std::string &name) {
        jpef_config_set_app_name(raw_.get(), name.c_str());
        return *this;
    }

    BuildConfig &set_version(const std::string &version) {
        jpef_config_set_version(raw_.get(), version.c_str());
        return *this;
    }

    BuildConfig &set_company(const std::string &company) {
        jpef_config_set_company(raw_.get(), company.c_str());
        return *this;
    }

    BuildConfig &set_targets(Target targets) {
        jpef_config_set_targets(raw_.get(), static_cast<uint32_t>(targets));
        return *this;
    }

    BuildConfig &set_gui_mode(bool is_gui) {
        jpef_config_set_gui_mode(raw_.get(), is_gui);
        return *this;
    }

    BuildConfig &set_icon_path(const fs::path &icon) {
        jpef_config_set_icon_path(raw_.get(), icon.string().c_str());
        return *this;
    }

    BuildConfig &set_jvm_heap(const std::string &min_heap, const std::string &max_heap) {
        jpef_config_set_jvm_heap(raw_.get(), min_heap.c_str(), max_heap.c_str());
        return *this;
    }

    BuildConfig &add_jvm_arg(const std::string &arg) {
        jpef_config_add_jvm_arg(raw_.get(), arg.c_str());
        return *this;
    }

    [[nodiscard]] const JpefConfig *raw() const noexcept { return raw_.get(); }

private:
    std::shared_ptr<JpefConfig> raw_;
};

inline std::string version() {
    const char *v = jpef_version();
    return v ? std::string(v) : "1.0.0";
}

inline JarInfo inspect(const fs::path &jar_path) {
    return JarInfo(jpef_inspect(jar_path.string().c_str()));
}

inline Result convert(const BuildConfig &config) {
    return Result(jpef_convert(config.raw()));
}

} // namespace jpef

#endif // JPEF_HPP
