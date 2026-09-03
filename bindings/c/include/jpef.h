/**
 * @file jpef.h
 * @brief JPEF (Java Portable Executable Format) - C API
 *
 * Provides functions to inspect JAR files and convert them into native
 * Windows (.exe), Linux (.elf), and macOS (.app) executables.
 */

#ifndef JPEF_H
#define JPEF_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Target platform bitflags */
#define JPEF_TARGET_EXE (1 << 0)
#define JPEF_TARGET_ELF (1 << 1)
#define JPEF_TARGET_APP (1 << 2)
#define JPEF_TARGET_ALL (JPEF_TARGET_EXE | JPEF_TARGET_ELF | JPEF_TARGET_APP)

/* Opaque types */
typedef struct JpefConfig JpefConfig;
typedef struct JpefResult JpefResult;
typedef struct JpefJarInfo JpefJarInfo;

/* Configuration functions */
JpefConfig *jpef_config_new(void);
void jpef_config_free(JpefConfig *config);
void jpef_config_set_jar_path(JpefConfig *config, const char *jar_path);
void jpef_config_set_output_dir(JpefConfig *config, const char *output_dir);
void jpef_config_set_app_name(JpefConfig *config, const char *app_name);
void jpef_config_set_version(JpefConfig *config, const char *version);
void jpef_config_set_company(JpefConfig *config, const char *company);
void jpef_config_set_targets(JpefConfig *config, uint32_t flags);
void jpef_config_set_gui_mode(JpefConfig *config, bool is_gui);
void jpef_config_set_icon_path(JpefConfig *config, const char *icon_path);
void jpef_config_set_jvm_heap(JpefConfig *config, const char *min_heap, const char *max_heap);
void jpef_config_add_jvm_arg(JpefConfig *config, const char *arg);

/* Conversion functions */
JpefResult *jpef_convert(const JpefConfig *config);
void jpef_result_free(JpefResult *result);
bool jpef_result_is_success(const JpefResult *result);
size_t jpef_result_get_artifact_count(const JpefResult *result);
const char *jpef_result_get_artifact_path(const JpefResult *result, size_t index);
const char *jpef_result_get_artifact_platform(const JpefResult *result, size_t index);
uint64_t jpef_result_get_artifact_size(const JpefResult *result, size_t index);
double jpef_result_get_elapsed_seconds(const JpefResult *result);
const char *jpef_result_get_errors(const JpefResult *result);

/* JAR Inspection functions */
JpefJarInfo *jpef_inspect(const char *jar_path);
void jpef_jar_info_free(JpefJarInfo *info);
const char *jpef_jar_info_get_main_class(const JpefJarInfo *info);
const char *jpef_jar_info_get_error(const JpefJarInfo *info);
uint32_t jpef_jar_info_get_min_java_version(const JpefJarInfo *info);
bool jpef_jar_info_is_runnable(const JpefJarInfo *info);

/* Version info */
const char *jpef_version(void);

#ifdef __cplusplus
}
#endif

#endif /* JPEF_H */
