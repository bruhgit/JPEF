use image::{ImageBuffer, Rgba};
use std::path::{Path, PathBuf};

pub fn generate_default_icon_image(size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::from_pixel(size, size, Rgba([0, 0, 0, 0]));
    let s = size as f32;
    let margin = s * 0.08;
    let radius = s * 0.18;
    let center = s / 2.0;

    for y in 0..size {
        for x in 0..size {
            let px = x as f32;
            let py = y as f32;

            // Check if inside rounded rectangle
            let dx = (px - center).abs() - (center - margin - radius);
            let dy = (py - center).abs() - (center - margin - radius);
            let inside_box = dx <= radius && dy <= radius && (dx <= 0.0 || dy <= 0.0 || (dx * dx + dy * dy) <= radius * radius);

            if inside_box {
                // Diagonal gradient background (blue to deep indigo)
                let t = (px + py) / (2.0 * s);
                let r = (20.0 * (1.0 - t) + 40.0 * t) as u8;
                let g = (80.0 * (1.0 - t) + 120.0 * t) as u8;
                let b = (220.0 * (1.0 - t) + 245.0 * t) as u8;

                // Central diamond emblem
                let diamond_dist = (px - center).abs() + (py - center).abs();
                if diamond_dist <= s * 0.22 {
                    if diamond_dist <= s * 0.10 {
                        // Orange inner accent
                        img.put_pixel(x, y, Rgba([255, 160, 40, 255]));
                    } else {
                        // White central diamond
                        img.put_pixel(x, y, Rgba([255, 255, 255, 245]));
                    }
                } else {
                    img.put_pixel(x, y, Rgba([r, g, b, 255]));
                }
            }
        }
    }

    img
}

pub fn prepare_icons(
    custom_icon: Option<&Path>,
    target_dir: &Path,
) -> Result<(PathBuf, PathBuf), String> {
    std::fs::create_dir_all(target_dir).map_err(|e| format!("Failed to create icon directory: {e}"))?;
    let ico_path = target_dir.join("app_icon.ico");
    let icns_path = target_dir.join("AppIcon.icns");

    let base_img = if let Some(p) = custom_icon {
        if p.is_file() {
            image::open(p).map(|img| img.to_rgba8()).unwrap_or_else(|_| generate_default_icon_image(256))
        } else {
            generate_default_icon_image(256)
        }
    } else {
        generate_default_icon_image(256)
    };

    // Save as ICO
    base_img
        .save_with_format(&ico_path, image::ImageFormat::Ico)
        .or_else(|_| base_img.save(&ico_path))
        .map_err(|e| format!("Failed to save ICO icon: {e}"))?;

    // Save as PNG/ICNS for macOS bundle
    base_img
        .save_with_format(&icns_path, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to save ICNS icon: {e}"))?;

    Ok((ico_path, icns_path))
}
