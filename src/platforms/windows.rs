use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::core::config::BuildConfig;
use crate::core::icon::prepare_icons;

const STUB_WIN_GUI: &[u8] = include_bytes!("../stubs/stub_win_gui.exe");
const STUB_WIN_CLI: &[u8] = include_bytes!("../stubs/stub_win_cli.exe");

pub fn build_windows_exe(
    config: &BuildConfig,
    progress_callback: Option<&dyn Fn(&str)>,
) -> Result<PathBuf, String> {
    let output_dir = &config.output_dir;
    fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output dir: {e}"))?;

    let exe_name = format!("{}.exe", config.metadata.app_name);
    let target_exe = output_dir.join(&exe_name);

    if let Some(cb) = progress_callback {
        cb(&format!("Building Windows PE ({})", if config.is_gui { "GUI" } else { "Console" }));
    }

    // Try dynamic GCC + windres compilation if available
    let mut compiled_custom = false;
    let temp_dir = std::env::temp_dir().join(format!("jpef_win_{}", std::process::id()));

    if Command::new("gcc").arg("--version").output().is_ok() && Command::new("windres").arg("--version").output().is_ok() {
        if let Ok(_) = fs::create_dir_all(&temp_dir) {
            if let Ok((ico_path, _)) = prepare_icons(config.icon_path.as_deref(), &temp_dir) {
                let rc_file = temp_dir.join("res.rc");
                let res_o = temp_dir.join("res.o");
                let sub_exe = temp_dir.join("launcher.exe");

                let rc_content = format!(
                    "1 ICON \"{}\"\n1 VERSIONINFO\nFILEVERSION 1,0,0,0\nPRODUCTVERSION 1,0,0,0\nBEGIN\n  BLOCK \"StringFileInfo\"\n  BEGIN\n    BLOCK \"040904b0\"\n    BEGIN\n      VALUE \"CompanyName\", \"{}\"\n      VALUE \"FileDescription\", \"{}\"\n      VALUE \"FileVersion\", \"{}\"\n      VALUE \"ProductName\", \"{}\"\n    END\n  END\n  BLOCK \"VarFileInfo\"\n  BEGIN\n    VALUE \"Translation\", 0x409, 1200\n  END\nEND\n",
                    ico_path.to_str().unwrap_or("").replace('\\', "/"),
                    config.metadata.company_name,
                    config.metadata.file_description,
                    config.metadata.version,
                    config.metadata.app_name
                );

                if fs::write(&rc_file, rc_content).is_ok() {
                    let windres_ok = Command::new("windres")
                        .args(["-i", rc_file.to_str().unwrap(), "-o", res_o.to_str().unwrap()])
                        .status()
                        .map(|s| s.success())
                        .unwrap_or(false);

                    if windres_ok {
                        // Check if C source exists in stubs
                        let c_stub = Path::new("src/stubs/launcher_win.c");
                        if c_stub.is_file() {
                            let mut gcc_cmd = Command::new("gcc");
                            gcc_cmd.args(["-O2", "-s", "-municode"]);
                            if config.is_gui {
                                gcc_cmd.args(["-DJPEF_GUI_MODE", "-mwindows"]);
                            } else {
                                gcc_cmd.arg("-mconsole");
                            }
                            gcc_cmd.args([
                                &format!("-DAPP_NAME=L\"{}\"", config.metadata.app_name),
                                "-Wl,--gc-sections",
                                "-o", sub_exe.to_str().unwrap(),
                                c_stub.to_str().unwrap(),
                                res_o.to_str().unwrap(),
                                "-lshlwapi", "-lshell32"
                            ]);

                            if let Ok(status) = gcc_cmd.status() {
                                if status.success() && sub_exe.is_file() {
                                    if fs::copy(&sub_exe, &target_exe).is_ok() {
                                        compiled_custom = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            let _ = fs::remove_dir_all(&temp_dir);
        }
    }

    if !compiled_custom {
        let stub_bytes = if config.is_gui { STUB_WIN_GUI } else { STUB_WIN_CLI };
        fs::write(&target_exe, stub_bytes).map_err(|e| format!("Failed to write Windows PE stub: {e}"))?;
    }

    if let Some(cb) = progress_callback {
        cb("Appending JAR payload to Windows executable...");
    }

    // Append JAR payload
    let mut jar_file = File::open(&config.jar_path).map_err(|e| format!("Failed to open JAR file: {e}"))?;
    let mut exe_file = OpenOptions::new()
        .append(true)
        .open(&target_exe)
        .map_err(|e| format!("Failed to open target executable for appending: {e}"))?;

    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = jar_file.read(&mut buf).map_err(|e| format!("Read error: {e}"))?;
        if n == 0 {
            break;
        }
        exe_file.write_all(&buf[..n]).map_err(|e| format!("Write error: {e}"))?;
    }

    Ok(target_exe)
}
