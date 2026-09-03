use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetPlatform {
    Exe,
    Elf,
    App,
}

impl TargetPlatform {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "exe" | "windows" => Some(Self::Exe),
            "elf" | "linux" => Some(Self::Elf),
            "app" | "macos" | "darwin" => Some(Self::App),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Exe => "exe",
            Self::Elf => "elf",
            Self::App => "app",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JvmOptions {
    pub min_heap: Option<String>,
    pub max_heap: Option<String>,
    pub custom_args: Vec<String>,
    pub bundled_jre_path: Option<String>,
    pub min_java_version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub app_name: String,
    pub version: String,
    pub company_name: String,
    pub file_description: String,
    pub copyright: String,
    pub bundle_id: String,
    pub main_class: Option<String>,
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self {
            app_name: "JavaApp".to_string(),
            version: "1.0.0.0".to_string(),
            company_name: "JPEF".to_string(),
            file_description: "Java Portable Executable".to_string(),
            copyright: "Copyright (C) 2026".to_string(),
            bundle_id: "com.jpef.app".to_string(),
            main_class: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub jar_path: PathBuf,
    pub output_dir: PathBuf,
    pub targets: Vec<TargetPlatform>,
    pub is_gui: bool,
    pub icon_path: Option<PathBuf>,
    pub metadata: AppMetadata,
    pub jvm: JvmOptions,
    pub create_zip_for_app: bool,
}

impl BuildConfig {
    pub fn new(jar_path: impl Into<PathBuf>, output_dir: impl Into<PathBuf>) -> Self {
        Self {
            jar_path: jar_path.into(),
            output_dir: output_dir.into(),
            targets: vec![TargetPlatform::Exe, TargetPlatform::Elf, TargetPlatform::App],
            is_gui: true,
            icon_path: None,
            metadata: AppMetadata::default(),
            jvm: JvmOptions::default(),
            create_zip_for_app: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildArtifact {
    pub platform: String,
    pub path: PathBuf,
    pub size_bytes: u64,
    pub is_directory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub success: bool,
    pub jar_info: Option<crate::core::manifest::JarInfo>,
    pub artifacts: Vec<BuildArtifact>,
    pub errors: Vec<String>,
    pub elapsed_seconds: f64,
}
