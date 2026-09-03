use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::PathBuf;

use crate::core::config::{BuildConfig, BuildResult, TargetPlatform};
use crate::core::converter::convert;
use crate::core::manifest::{inspect_jar, JarInfo};

pub const JPEF_TARGET_EXE: u32 = 1 << 0;
pub const JPEF_TARGET_ELF: u32 = 1 << 1;
pub const JPEF_TARGET_APP: u32 = 1 << 2;
pub const JPEF_TARGET_ALL: u32 = JPEF_TARGET_EXE | JPEF_TARGET_ELF | JPEF_TARGET_APP;

pub struct JpefConfig {
    pub inner: BuildConfig,
}

pub struct JpefResult {
    pub inner: BuildResult,
    // Cached CStrings for safe pointer delivery to caller
    cached_strings: Vec<CString>,
    error_summary: CString,
}

pub struct JpefJarInfo {
    pub inner: JarInfo,
    cached_main_class: Option<CString>,
    cached_error: Option<CString>,
}

unsafe fn c_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_string())
    }
}

// ------------------- Config API -------------------

#[no_mangle]
pub extern "C" fn jpef_config_new() -> *mut JpefConfig {
    Box::into_raw(Box::new(JpefConfig {
        inner: BuildConfig::new("", "dist"),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_free(config: *mut JpefConfig) {
    if !config.is_null() {
        drop(Box::from_raw(config));
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_jar_path(config: *mut JpefConfig, jar_path: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        if let Some(s) = c_to_string(jar_path) {
            cfg.inner.jar_path = PathBuf::from(s);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_output_dir(config: *mut JpefConfig, output_dir: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        if let Some(s) = c_to_string(output_dir) {
            cfg.inner.output_dir = PathBuf::from(s);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_app_name(config: *mut JpefConfig, app_name: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        if let Some(s) = c_to_string(app_name) {
            cfg.inner.metadata.app_name = s;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_version(config: *mut JpefConfig, version: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        if let Some(s) = c_to_string(version) {
            cfg.inner.metadata.version = s;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_company(config: *mut JpefConfig, company: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        if let Some(s) = c_to_string(company) {
            cfg.inner.metadata.company_name = s;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_targets(config: *mut JpefConfig, flags: u32) {
    if let Some(cfg) = config.as_mut() {
        let mut targets = Vec::new();
        if (flags & JPEF_TARGET_EXE) != 0 {
            targets.push(TargetPlatform::Exe);
        }
        if (flags & JPEF_TARGET_ELF) != 0 {
            targets.push(TargetPlatform::Elf);
        }
        if (flags & JPEF_TARGET_APP) != 0 {
            targets.push(TargetPlatform::App);
        }
        cfg.inner.targets = targets;
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_gui_mode(config: *mut JpefConfig, is_gui: bool) {
    if let Some(cfg) = config.as_mut() {
        cfg.inner.is_gui = is_gui;
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_icon_path(config: *mut JpefConfig, icon_path: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        cfg.inner.icon_path = c_to_string(icon_path).map(PathBuf::from);
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_set_jvm_heap(
    config: *mut JpefConfig,
    min_heap: *const c_char,
    max_heap: *const c_char,
) {
    if let Some(cfg) = config.as_mut() {
        cfg.inner.jvm.min_heap = c_to_string(min_heap);
        cfg.inner.jvm.max_heap = c_to_string(max_heap);
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_config_add_jvm_arg(config: *mut JpefConfig, arg: *const c_char) {
    if let Some(cfg) = config.as_mut() {
        if let Some(s) = c_to_string(arg) {
            cfg.inner.jvm.custom_args.push(s);
        }
    }
}

// ------------------- Convert & Result API -------------------

#[no_mangle]
pub unsafe extern "C" fn jpef_convert(config: *const JpefConfig) -> *mut JpefResult {
    if config.is_null() {
        return std::ptr::null_mut();
    }
    let cfg = &(*config).inner;
    let res = convert(cfg, None);

    let mut cached_strings = Vec::new();
    for art in &res.artifacts {
        cached_strings.push(CString::new(art.path.to_string_lossy().as_bytes()).unwrap_or_default());
        cached_strings.push(CString::new(art.platform.as_bytes()).unwrap_or_default());
    }

    let error_summary = CString::new(res.errors.join("\n")).unwrap_or_default();

    Box::into_raw(Box::new(JpefResult {
        inner: res,
        cached_strings,
        error_summary,
    }))
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_free(result: *mut JpefResult) {
    if !result.is_null() {
        drop(Box::from_raw(result));
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_is_success(result: *const JpefResult) -> bool {
    if let Some(res) = result.as_ref() {
        res.inner.success
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_get_artifact_count(result: *const JpefResult) -> usize {
    if let Some(res) = result.as_ref() {
        res.inner.artifacts.len()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_get_artifact_path(
    result: *const JpefResult,
    index: usize,
) -> *const c_char {
    if let Some(res) = result.as_ref() {
        if index < res.inner.artifacts.len() {
            let str_idx = index * 2;
            if str_idx < res.cached_strings.len() {
                return res.cached_strings[str_idx].as_ptr();
            }
        }
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_get_artifact_platform(
    result: *const JpefResult,
    index: usize,
) -> *const c_char {
    if let Some(res) = result.as_ref() {
        if index < res.inner.artifacts.len() {
            let str_idx = index * 2 + 1;
            if str_idx < res.cached_strings.len() {
                return res.cached_strings[str_idx].as_ptr();
            }
        }
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_get_artifact_size(
    result: *const JpefResult,
    index: usize,
) -> u64 {
    if let Some(res) = result.as_ref() {
        if let Some(art) = res.inner.artifacts.get(index) {
            return art.size_bytes;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_get_elapsed_seconds(result: *const JpefResult) -> f64 {
    if let Some(res) = result.as_ref() {
        res.inner.elapsed_seconds
    } else {
        0.0
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_result_get_errors(result: *const JpefResult) -> *const c_char {
    if let Some(res) = result.as_ref() {
        res.error_summary.as_ptr()
    } else {
        std::ptr::null()
    }
}

// ------------------- Inspect API -------------------

#[no_mangle]
pub unsafe extern "C" fn jpef_inspect(jar_path: *const c_char) -> *mut JpefJarInfo {
    let path_str = match c_to_string(jar_path) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };

    match inspect_jar(&PathBuf::from(path_str)) {
        Ok(info) => {
            let cached_main_class = info.main_class.as_ref().and_then(|s| CString::new(s.as_bytes()).ok());
            let cached_error = info.error_message.as_ref().and_then(|s| CString::new(s.as_bytes()).ok());
            Box::into_raw(Box::new(JpefJarInfo {
                inner: info,
                cached_main_class,
                cached_error,
            }))
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_jar_info_free(info: *mut JpefJarInfo) {
    if !info.is_null() {
        drop(Box::from_raw(info));
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_jar_info_get_main_class(info: *const JpefJarInfo) -> *const c_char {
    if let Some(ji) = info.as_ref() {
        if let Some(ref c_str) = ji.cached_main_class {
            return c_str.as_ptr();
        }
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn jpef_jar_info_get_error(info: *const JpefJarInfo) -> *const c_char {
    if let Some(ji) = info.as_ref() {
        if let Some(ref c_str) = ji.cached_error {
            return c_str.as_ptr();
        }
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn jpef_jar_info_get_min_java_version(info: *const JpefJarInfo) -> u32 {
    if let Some(ji) = info.as_ref() {
        ji.inner.min_java_version.unwrap_or(0)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn jpef_jar_info_is_runnable(info: *const JpefJarInfo) -> bool {
    if let Some(ji) = info.as_ref() {
        ji.inner.is_valid_runnable
    } else {
        false
    }
}

// ------------------- Utility API -------------------

#[no_mangle]
pub extern "C" fn jpef_version() -> *const c_char {
    static VERSION: &[u8] = b"1.0.0\0";
    VERSION.as_ptr() as *const c_char
}
