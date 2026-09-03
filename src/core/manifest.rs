use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JarInfo {
    pub path: PathBuf,
    pub file_size_bytes: u64,
    pub entry_count: usize,
    pub main_class: Option<String>,
    pub classpath: Vec<String>,
    pub implementation_title: Option<String>,
    pub implementation_version: Option<String>,
    pub bytecode_major_version: Option<u16>,
    pub min_java_version: Option<u32>,
    pub is_valid_runnable: bool,
    pub error_message: Option<String>,
}

pub fn bytecode_to_java_version(major: u16) -> u32 {
    match major {
        45..=48 => major as u32 - 44, // 1..4
        49 => 5,
        50 => 6,
        51 => 7,
        52 => 8,
        53 => 9,
        54 => 10,
        55 => 11,
        56 => 12,
        57 => 13,
        58 => 14,
        59 => 15,
        60 => 16,
        61 => 17,
        62 => 18,
        63 => 19,
        64 => 20,
        65 => 21,
        66 => 22,
        67 => 23,
        68 => 24,
        _ if major > 68 => major as u32 - 44,
        _ => 8,
    }
}

pub fn parse_manifest(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut current_key: Option<String> = None;
    let mut current_value = String::new();

    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with(' ') && current_key.is_some() {
            current_value.push_str(&trimmed[1..]);
        } else if let Some((k, v)) = trimmed.split_once(':') {
            if let Some(prev_key) = current_key.take() {
                map.insert(prev_key, current_value.trim().to_string());
                current_value.clear();
            }
            current_key = Some(k.trim().to_string());
            current_value = v.trim().to_string();
        }
    }

    if let Some(last_key) = current_key {
        map.insert(last_key, current_value.trim().to_string());
    }

    map
}

pub fn inspect_jar(path: &Path) -> Result<JarInfo, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open JAR file: {e}"))?;
    let metadata = file.metadata().map_err(|e| format!("Failed to read file metadata: {e}"))?;
    let file_size_bytes = metadata.len();

    let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Invalid ZIP/JAR archive: {e}"))?;
    let entry_count = archive.len();

    let mut manifest_content: Option<String> = None;
    let mut max_bytecode_major: Option<u16> = None;

    // Scan for manifest and inspect class files
    let mut class_checks = 0;
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let name = file.name().to_string();
        if name.eq_ignore_ascii_case("META-INF/MANIFEST.MF") && manifest_content.is_none() {
            let mut buf = String::new();
            if file.read_to_string(&mut buf).is_ok() {
                manifest_content = Some(buf);
            }
        } else if name.ends_with(".class") && class_checks < 15 {
            let mut header = [0u8; 8];
            if file.read_exact(&mut header).is_ok() && &header[0..4] == b"\xca\xfe\xba\xbe" {
                let major = u16::from_be_bytes([header[6], header[7]]);
                if max_bytecode_major.is_none() || Some(major) > max_bytecode_major {
                    max_bytecode_major = Some(major);
                }
                class_checks += 1;
            }
        }
    }

    let mut main_class = None;
    let mut classpath = Vec::new();
    let mut impl_title = None;
    let mut impl_version = None;

    if let Some(content) = manifest_content {
        let headers = parse_manifest(&content);
        main_class = headers.get("Main-Class").cloned();
        if let Some(cp) = headers.get("Class-Path") {
            classpath = cp.split_whitespace().map(|s| s.to_string()).collect();
        }
        impl_title = headers.get("Implementation-Title").cloned();
        impl_version = headers.get("Implementation-Version").cloned();
    }

    let min_java_version = max_bytecode_major.map(bytecode_to_java_version);
    let is_valid_runnable = main_class.is_some();
    let error_message = if is_valid_runnable {
        None
    } else {
        Some("Warning: Main-Class attribute was not found in META-INF/MANIFEST.MF".to_string())
    };

    Ok(JarInfo {
        path: path.to_path_buf(),
        file_size_bytes,
        entry_count,
        main_class,
        classpath,
        implementation_title: impl_title,
        implementation_version: impl_version,
        bytecode_major_version: max_bytecode_major,
        min_java_version,
        is_valid_runnable,
        error_message,
    })
}
