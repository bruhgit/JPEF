use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::core::config::BuildConfig;

const STUB_LINUX_ELF: &[u8] = include_bytes!("../stubs/stub_linux_amd64.elf");

pub fn build_linux_elf(
    config: &BuildConfig,
    progress_callback: Option<&dyn Fn(&str)>,
) -> Result<PathBuf, String> {
    let output_dir = &config.output_dir;
    fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output dir: {e}"))?;

    let elf_name = format!("{}.elf", config.metadata.app_name);
    let target_elf = output_dir.join(&elf_name);

    if let Some(cb) = progress_callback {
        cb(&format!("Writing Linux 64-bit ELF launcher for {}", config.metadata.app_name));
    }

    fs::write(&target_elf, STUB_LINUX_ELF).map_err(|e| format!("Failed to write Linux ELF stub: {e}"))?;

    if let Some(cb) = progress_callback {
        cb("Appending JAR payload to Linux ELF executable...");
    }

    // Append JAR payload
    let mut jar_file = File::open(&config.jar_path).map_err(|e| format!("Failed to open JAR file: {e}"))?;
    let mut elf_file = OpenOptions::new()
        .append(true)
        .open(&target_elf)
        .map_err(|e| format!("Failed to open target ELF for appending: {e}"))?;

    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = jar_file.read(&mut buf).map_err(|e| format!("Read error: {e}"))?;
        if n == 0 {
            break;
        }
        elf_file.write_all(&buf[..n]).map_err(|e| format!("Write error: {e}"))?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&target_elf) {
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            let _ = fs::set_permissions(&target_elf, permissions);
        }
    }

    Ok(target_elf)
}
