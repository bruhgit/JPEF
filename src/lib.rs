pub mod core;
pub mod ffi;
pub mod platforms;

// Public Rust API re-exports
pub use core::config::{
    AppMetadata, BuildArtifact, BuildConfig, BuildResult, JvmOptions, TargetPlatform,
};
pub use core::converter::convert;
pub use core::manifest::{inspect_jar, JarInfo};
pub use ffi::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
