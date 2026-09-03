use std::fs::File;
use std::io::Write;

use jpef::core::config::{BuildConfig, TargetPlatform};
use jpef::core::converter::convert;
use jpef::core::manifest::inspect_jar;

fn create_test_jar(path: &std::path::Path) {
    let file = File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    zip.start_file("META-INF/MANIFEST.MF", options).unwrap();
    zip.write_all(b"Manifest-Version: 1.0\r\nMain-Class: org.jpef.TestMain\r\n\r\n").unwrap();

    zip.start_file("org/jpef/TestMain.class", options).unwrap();
    zip.write_all(b"\xca\xfe\xba\xbe\x00\x00\x00\x41").unwrap(); // bytecode 65 = Java 21

    zip.finish().unwrap();
}

#[test]
fn test_inspect_jar() {
    let temp_dir = std::env::temp_dir().join(format!("jpef_test_insp_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let jar_path = temp_dir.join("test.jar");
    create_test_jar(&jar_path);

    let info = inspect_jar(&jar_path).expect("inspect failed");
    assert_eq!(info.main_class.as_deref(), Some("org.jpef.TestMain"));
    assert_eq!(info.bytecode_major_version, Some(65));
    assert_eq!(info.min_java_version, Some(21));
    assert!(info.is_valid_runnable);

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_multi_target_convert() {
    let temp_dir = std::env::temp_dir().join(format!("jpef_test_conv_{}", std::process::id()));
    let out_dir = temp_dir.join("out");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let jar_path = temp_dir.join("test.jar");
    create_test_jar(&jar_path);

    let mut config = BuildConfig::new(&jar_path, &out_dir);
    config.targets = vec![TargetPlatform::Exe, TargetPlatform::Elf, TargetPlatform::App];
    config.metadata.app_name = "RustSample".to_string();

    let res = convert(&config, None);
    assert!(res.success, "Conversion failed: {:?}", res.errors);
    assert_eq!(res.artifacts.len(), 3);

    assert!(out_dir.join("RustSample.exe").is_file());
    assert!(out_dir.join("RustSample.elf").is_file());
    assert!(out_dir.join("RustSample.app").is_dir());

    let _ = std::fs::remove_dir_all(&temp_dir);
}
