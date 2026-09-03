/**
 * Example C program using JPEF C-ABI library.
 */

#include <stdio.h>
#include <stdlib.h>
#include "jpef.h"

int main(int argc, char *argv[]) {
    const char *jar_path = (argc > 1) ? argv[1] : "sample.jar";

    printf("========================================\n");
    printf(" JPEF C Binding - Version: %s\n", jpef_version());
    printf("========================================\n\n");

    // 1. Inspect JAR
    printf("[1] Inspecting JAR: %s\n", jar_path);
    JpefJarInfo *info = jpef_inspect(jar_path);
    if (info) {
        const char *main_class = jpef_jar_info_get_main_class(info);
        uint32_t min_java = jpef_jar_info_get_min_java_version(info);
        bool runnable = jpef_jar_info_is_runnable(info);

        printf("  Main-Class:   %s\n", main_class ? main_class : "(None)");
        printf("  Min Java:     Java %u+\n", min_java);
        printf("  Runnable:     %s\n\n", runnable ? "Yes" : "No");
        jpef_jar_info_free(info);
    } else {
        printf("  (Could not inspect %s, proceeding to convert test)\n\n", jar_path);
    }

    // 2. Configure conversion
    printf("[2] Configuring conversion...\n");
    JpefConfig *cfg = jpef_config_new();
    jpef_config_set_jar_path(cfg, jar_path);
    jpef_config_set_output_dir(cfg, "dist_c");
    jpef_config_set_app_name(cfg, "SampleAppC");
    jpef_config_set_version(cfg, "1.0.0.0");
    jpef_config_set_company(cfg, "JPEF-C");
    jpef_config_set_targets(cfg, JPEF_TARGET_EXE | JPEF_TARGET_ELF | JPEF_TARGET_APP);
    jpef_config_set_gui_mode(cfg, false); // Console mode
    jpef_config_set_jvm_heap(cfg, "128m", "512m");
    jpef_config_add_jvm_arg(cfg, "-Dfile.encoding=UTF-8");

    // 3. Run conversion
    printf("[3] Converting to .exe, .elf, and .app...\n");
    JpefResult *res = jpef_convert(cfg);
    jpef_config_free(cfg);

    if (!res) {
        fprintf(stderr, "Error: jpef_convert returned NULL!\n");
        return 1;
    }

    bool success = jpef_result_is_success(res);
    double elapsed = jpef_result_get_elapsed_seconds(res);
    size_t count = jpef_result_get_artifact_count(res);

    if (success) {
        printf("\n[SUCCESS] Generated %zu artifact(s) in %.2fs:\n", count, elapsed);
        for (size_t i = 0; i < count; i++) {
            const char *platform = jpef_result_get_artifact_platform(res, i);
            const char *path = jpef_result_get_artifact_path(res, i);
            uint64_t size = jpef_result_get_artifact_size(res, i);
            printf("  - [%s] %s (%.2f MB)\n", platform, path, (double)size / (1024.0 * 1024.0));
        }
    } else {
        printf("\n[FAILED] Conversion failed: %s\n", jpef_result_get_errors(res));
    }

    jpef_result_free(res);
    return success ? 0 : 1;
}
