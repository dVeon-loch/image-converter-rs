use std::path::{Path, PathBuf};

use image::ImageFormat;
use walkdir::WalkDir;

pub fn convert_image(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    image_format: ImageFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    img.save_with_format(output_path, image_format)?;
    Ok(())
}

static MAX_FILES: usize = 5000;

// Define image extensions (case-insensitive)
static IMAGE_EXTS: [&'static str; 8] = ["png", "jpg", "jpeg", "bmp", "gif", "webp", "tiff", "svg"];

pub fn collect_image_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .follow_links(true) // Follow symbolic links
        .into_iter()
        .take(MAX_FILES)
        .filter_map(|e| e.ok()) // Ignore unreadable entries
        .map(|e| e.path().to_path_buf())
        .filter(|e| e.is_file())
        .filter(|e| {
            let exts: Vec<String> = [IMAGE_EXTS.clone()]
                .iter()
                .flatten()
                .map(|ext| ext.to_string())
                .collect();

            let extension = e.extension();
            extension.is_some()
                && exts.contains(
                    &extension
                        .unwrap()
                        .to_ascii_lowercase()
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
        })
        .collect()
}
