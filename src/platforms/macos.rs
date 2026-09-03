use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::core::config::BuildConfig;
use crate::core::icon::prepare_icons;

const STUB_MACOS_MACHO: &[u8] = include_bytes!("../stubs/stub_darwin_amd64");

pub fn build_macos_app(
    config: &BuildConfig,
    progress_callback: Option<&dyn Fn(&str)>,
) -> Result<PathBuf, String> {
    let output_dir = &config.output_dir;
    fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output dir: {e}"))?;

    let app_name = &config.metadata.app_name;
    let bundle_dir = output_dir.join(format!("{}.app", app_name));

    if bundle_dir.exists() {
        let _ = fs::remove_dir_all(&bundle_dir);
    }

    let contents_dir = bundle_dir.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let resources_dir = contents_dir.join("Resources");

    fs::create_dir_all(&macos_dir).map_err(|e| format!("Failed to create MacOS dir: {e}"))?;
    fs::create_dir_all(&resources_dir).map_err(|e| format!("Failed to create Resources dir: {e}"))?;

    if let Some(cb) = progress_callback {
        cb(&format!("Building macOS Application Bundle: {}.app", app_name));
    }

    // 1. Info.plist XML
    let bundle_id = &config.metadata.bundle_id;
    let version = &config.metadata.version;
    let copyright = &config.metadata.copyright;

    let plist_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>{}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>{}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>{}</string>
    <key>CFBundleVersion</key>
    <string>{}</string>
    <key>NSHumanReadableCopyright</key>
    <string>{}</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
"#,
        app_name, bundle_id, app_name, version, version, copyright
    );

    fs::write(contents_dir.join("Info.plist"), plist_xml)
        .map_err(|e| format!("Failed to write Info.plist: {e}"))?;

    // 2. PkgInfo
    fs::write(contents_dir.join("PkgInfo"), b"APPL????")
        .map_err(|e| format!("Failed to write PkgInfo: {e}"))?;

    // 3. Resources/app.jar & <app_name>.jar
    fs::copy(&config.jar_path, resources_dir.join(format!("{}.jar", app_name)))
        .map_err(|e| format!("Failed to copy JAR to Resources: {e}"))?;
    let _ = fs::copy(&config.jar_path, resources_dir.join("app.jar"));

    // 4. Icon
    let temp_icon_dir = std::env::temp_dir().join(format!("jpef_mac_icon_{}", std::process::id()));
    if let Ok((_, icns_path)) = prepare_icons(config.icon_path.as_deref(), &temp_icon_dir) {
        let _ = fs::copy(icns_path, resources_dir.join("AppIcon.icns"));
        let _ = fs::remove_dir_all(&temp_icon_dir);
    }

    // 5. Native Mach-O launcher
    let launcher_path = macos_dir.join(app_name);
    fs::write(&launcher_path, STUB_MACOS_MACHO)
        .map_err(|e| format!("Failed to write Mach-O launcher: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(&launcher_path) {
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            let _ = fs::set_permissions(&launcher_path, permissions);
        }
    }

    // 6. Zip archive if enabled
    if config.create_zip_for_app {
        if let Some(cb) = progress_callback {
            cb(&format!("Packaging {}.app.zip distribution archive...", app_name));
        }
        let zip_path = output_dir.join(format!("{}.app.zip", app_name));
        if let Ok(zip_file) = File::create(&zip_path) {
            let mut zip = zip::ZipWriter::new(zip_file);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);

            let prefix = format!("{}.app", app_name);
            zip_dir_recursive(&bundle_dir, &prefix, &mut zip, options)?;
            let _ = zip.finish();
        }
    }

    Ok(bundle_dir)
}

fn zip_dir_recursive(
    dir: &Path,
    prefix: &str,
    zip: &mut zip::ZipWriter<File>,
    options: zip::write::SimpleFileOptions,
) -> Result<(), String> {
    for entry in fs::read_dir(dir).map_err(|e| format!("Read dir error: {e}"))? {
        let entry = entry.map_err(|e| format!("Entry error: {e}"))?;
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let arc_name = format!("{}/{}", prefix, file_name);

        if path.is_dir() {
            zip.add_directory(&arc_name, options)
                .map_err(|e| format!("Add dir error: {e}"))?;
            zip_dir_recursive(&path, &arc_name, zip, options)?;
        } else {
            zip.start_file(&arc_name, options)
                .map_err(|e| format!("Start file error: {e}"))?;
            let mut f = File::open(&path).map_err(|e| format!("Open error: {e}"))?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf).map_err(|e| format!("Read error: {e}"))?;
            zip.write_all(&buf).map_err(|e| format!("Write error: {e}"))?;
        }
    }
    Ok(())
}
