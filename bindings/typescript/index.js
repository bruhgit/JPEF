/**
 * JPEF JavaScript / TypeScript Node.js Binding
 * Powered by Koffi FFI and Rust Native Core
 */

const path = require('path');
const fs = require('fs');
const koffi = require('koffi');

// Find jpef.dll / libjpef.so / libjpef.dylib
function findLibrary() {
  const candidates = [
    path.resolve(__dirname, '../../target/release/jpef.dll'),
    path.resolve(__dirname, '../../target/release/libjpef.dll'),
    path.resolve(__dirname, '../../target/release/libjpef.so'),
    path.resolve(__dirname, '../../target/release/libjpef.dylib'),
    path.resolve(__dirname, './jpef.dll'),
    path.resolve(__dirname, './libjpef.so'),
  ];

  for (const p of candidates) {
    if (fs.existsSync(p)) {
      return p;
    }
  }

  // Fallback to library name in system path
  return process.platform === 'win32' ? 'jpef.dll' : 'libjpef.so';
}

const libPath = findLibrary();
const lib = koffi.load(libPath);

// Define opaque pointers
const JpefConfigPtr = koffi.pointer('JpefConfig', koffi.opaque());
const JpefResultPtr = koffi.pointer('JpefResult', koffi.opaque());
const JpefJarInfoPtr = koffi.pointer('JpefJarInfo', koffi.opaque());

// Function bindings
const jpef_version = lib.func('const char *jpef_version()');

// Config
const jpef_config_new = lib.func('JpefConfig *jpef_config_new()');
const jpef_config_free = lib.func('void jpef_config_free(JpefConfig *config)');
const jpef_config_set_jar_path = lib.func('void jpef_config_set_jar_path(JpefConfig *config, const char *jar_path)');
const jpef_config_set_output_dir = lib.func('void jpef_config_set_output_dir(JpefConfig *config, const char *output_dir)');
const jpef_config_set_app_name = lib.func('void jpef_config_set_app_name(JpefConfig *config, const char *app_name)');
const jpef_config_set_version = lib.func('void jpef_config_set_version(JpefConfig *config, const char *version)');
const jpef_config_set_company = lib.func('void jpef_config_set_company(JpefConfig *config, const char *company)');
const jpef_config_set_targets = lib.func('void jpef_config_set_targets(JpefConfig *config, uint32_t flags)');
const jpef_config_set_gui_mode = lib.func('void jpef_config_set_gui_mode(JpefConfig *config, bool is_gui)');
const jpef_config_set_icon_path = lib.func('void jpef_config_set_icon_path(JpefConfig *config, const char *icon_path)');
const jpef_config_set_jvm_heap = lib.func('void jpef_config_set_jvm_heap(JpefConfig *config, const char *min_heap, const char *max_heap)');
const jpef_config_add_jvm_arg = lib.func('void jpef_config_add_jvm_arg(JpefConfig *config, const char *arg)');

// Convert
const jpef_convert = lib.func('JpefResult *jpef_convert(const JpefConfig *config)');
const jpef_result_free = lib.func('void jpef_result_free(JpefResult *result)');
const jpef_result_is_success = lib.func('bool jpef_result_is_success(const JpefResult *result)');
const jpef_result_get_artifact_count = lib.func('size_t jpef_result_get_artifact_count(const JpefResult *result)');
const jpef_result_get_artifact_path = lib.func('const char *jpef_result_get_artifact_path(const JpefResult *result, size_t index)');
const jpef_result_get_artifact_platform = lib.func('const char *jpef_result_get_artifact_platform(const JpefResult *result, size_t index)');
const jpef_result_get_artifact_size = lib.func('uint64_t jpef_result_get_artifact_size(const JpefResult *result, size_t index)');
const jpef_result_get_elapsed_seconds = lib.func('double jpef_result_get_elapsed_seconds(const JpefResult *result)');
const jpef_result_get_errors = lib.func('const char *jpef_result_get_errors(const JpefResult *result)');

// Inspect
const jpef_inspect = lib.func('JpefJarInfo *jpef_inspect(const char *jar_path)');
const jpef_jar_info_free = lib.func('void jpef_jar_info_free(JpefJarInfo *info)');
const jpef_jar_info_get_main_class = lib.func('const char *jpef_jar_info_get_main_class(const JpefJarInfo *info)');
const jpef_jar_info_get_min_java_version = lib.func('uint32_t jpef_jar_info_get_min_java_version(const JpefJarInfo *info)');
const jpef_jar_info_is_runnable = lib.func('bool jpef_jar_info_is_runnable(const JpefJarInfo *info)');
const jpef_jar_info_get_error = lib.func('const char *jpef_jar_info_get_error(const JpefJarInfo *info)');

const TARGET_EXE = 1 << 0;
const TARGET_ELF = 1 << 1;
const TARGET_APP = 1 << 2;

function version() {
  return jpef_version();
}

function inspect(jarPath) {
  const infoPtr = jpef_inspect(jarPath);
  if (!infoPtr) {
    return {
      mainClass: null,
      minJavaVersion: 0,
      isRunnable: false,
      error: 'Failed to inspect JAR archive',
    };
  }

  try {
    return {
      mainClass: jpef_jar_info_get_main_class(infoPtr),
      minJavaVersion: jpef_jar_info_get_min_java_version(infoPtr),
      isRunnable: jpef_jar_info_is_runnable(infoPtr),
      error: jpef_jar_info_get_error(infoPtr),
    };
  } finally {
    jpef_jar_info_free(infoPtr);
  }
}

function convert(options) {
  if (!options || !options.jarPath) {
    throw new Error('options.jarPath is required');
  }

  const cfg = jpef_config_new();
  try {
    jpef_config_set_jar_path(cfg, options.jarPath);
    if (options.outputDir) jpef_config_set_output_dir(cfg, options.outputDir);
    if (options.appName) jpef_config_set_app_name(cfg, options.appName);
    if (options.version) jpef_config_set_version(cfg, options.version);
    if (options.companyName) jpef_config_set_company(cfg, options.companyName);
    if (options.iconPath) jpef_config_set_icon_path(cfg, options.iconPath);

    let flags = 0;
    const targets = options.targets || ['exe', 'elf', 'app'];
    for (const t of targets) {
      const s = String(t).toLowerCase();
      if (s === 'exe' || s === 'windows') flags |= TARGET_EXE;
      if (s === 'elf' || s === 'linux') flags |= TARGET_ELF;
      if (s === 'app' || s === 'macos') flags |= TARGET_APP;
    }
    jpef_config_set_targets(cfg, flags);

    jpef_config_set_gui_mode(cfg, options.isGui !== false);

    if (options.minHeap || options.maxHeap) {
      jpef_config_set_jvm_heap(cfg, options.minHeap || '', options.maxHeap || '');
    }

    if (Array.isArray(options.jvmArgs)) {
      for (const arg of options.jvmArgs) {
        jpef_config_add_jvm_arg(cfg, arg);
      }
    }

    const res = jpef_convert(cfg);
    if (!res) {
      throw new Error('jpef_convert returned NULL');
    }

    try {
      const success = jpef_result_is_success(res);
      const elapsed = jpef_result_get_elapsed_seconds(res);
      const errors = jpef_result_get_errors(res) || '';
      const count = jpef_result_get_artifact_count(res);

      const artifacts = [];
      for (let i = 0; i < count; i++) {
        artifacts.push({
          platform: jpef_result_get_artifact_platform(res, i),
          path: jpef_result_get_artifact_path(res, i),
          sizeBytes: Number(jpef_result_get_artifact_size(res, i)),
        });
      }

      return {
        success,
        elapsedSeconds: elapsed,
        artifacts,
        errors,
      };
    } finally {
      jpef_result_free(res);
    }
  } finally {
    jpef_config_free(cfg);
  }
}

module.exports = {
  version,
  inspect,
  convert,
};
