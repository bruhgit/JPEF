use std::time::Instant;

use crate::core::config::{BuildArtifact, BuildConfig, BuildResult, TargetPlatform};
use crate::core::manifest::inspect_jar;
use crate::platforms::linux::build_linux_elf;
use crate::platforms::macos::build_macos_app;
use crate::platforms::windows::build_windows_exe;

pub fn convert(
    config: &BuildConfig,
    progress_callback: Option<&dyn Fn(&str)>,
) -> BuildResult {
    let start_time = Instant::now();
    let mut artifacts = Vec::new();
    let mut errors = Vec::new();

    if let Some(cb) = progress_callback {
        cb(&format!("Inspecting JAR: {}", config.jar_path.display()));
    }

    let jar_info = match inspect_jar(&config.jar_path) {
        Ok(info) => Some(info),
        Err(e) => {
            return BuildResult {
                success: false,
                jar_info: None,
                artifacts: vec![],
                errors: vec![format!("Failed to inspect input JAR: {e}")],
                elapsed_seconds: start_time.elapsed().as_secs_f64(),
            };
        }
    };

    // Clone config to allow mutating inferred app name if default
    let mut effective_config = config.clone();

    if let Some(ref info) = jar_info {
        if effective_config.metadata.app_name == "JavaApp" || effective_config.metadata.app_name.is_empty() {
            if let Some(ref title) = info.implementation_title {
                let clean: String = title.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-').collect();
                if !clean.is_empty() {
                    effective_config.metadata.app_name = clean;
                }
            } else if let Some(stem) = config.jar_path.file_stem() {
                effective_config.metadata.app_name = stem.to_string_lossy().to_string();
            }
        }

        if effective_config.metadata.main_class.is_none() {
            effective_config.metadata.main_class = info.main_class.clone();
        }
    }

    // Run platform builds
    for target in &effective_config.targets {
        match target {
            TargetPlatform::Exe => match build_windows_exe(&effective_config, progress_callback) {
                Ok(path) => {
                    let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    artifacts.push(BuildArtifact {
                        platform: "Windows (.exe)".to_string(),
                        path,
                        size_bytes,
                        is_directory: false,
                    });
                }
                Err(e) => errors.push(format!("Windows build failed: {e}")),
            },
            TargetPlatform::Elf => match build_linux_elf(&effective_config, progress_callback) {
                Ok(path) => {
                    let size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    artifacts.push(BuildArtifact {
                        platform: "Linux (.elf)".to_string(),
                        path,
                        size_bytes,
                        is_directory: false,
                    });
                }
                Err(e) => errors.push(format!("Linux build failed: {e}")),
            },
            TargetPlatform::App => match build_macos_app(&effective_config, progress_callback) {
                Ok(path) => {
                    let size_bytes = compute_dir_size(&path);
                    artifacts.push(BuildArtifact {
                        platform: "macOS (.app)".to_string(),
                        path,
                        size_bytes,
                        is_directory: true,
                    });
                }
                Err(e) => errors.push(format!("macOS build failed: {e}")),
            },
        }
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let success = !artifacts.is_empty() && errors.is_empty();

    if let Some(cb) = progress_callback {
        cb(&format!("Finished build in {:.2}s. Generated {} artifact(s).", elapsed, artifacts.len()));
    }

    BuildResult {
        success,
        jar_info,
        artifacts,
        errors,
        elapsed_seconds: elapsed,
    }
}

fn compute_dir_size(dir: &std::path::Path) -> u64 {
    let mut total = 0;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                total += compute_dir_size(&path);
            } else if let Ok(m) = entry.metadata() {
                total += m.len();
            }
        }
    }
    total
}
