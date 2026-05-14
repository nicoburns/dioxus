use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use imagesize::ImageSize;
use imagesize::ImageType;
use imagesize::{reader_size, reader_type};

pub struct RasterIcon {
    pub path: PathBuf,
    pub size: ImageSize,
    pub format: ImageType,
}

pub struct NonRasterIcon {
    pub path: PathBuf,
    pub format: NonRasterIconKind,
}

pub enum NonRasterIconKind {
    /// Windows .ico file
    Ico,
    /// macOS .icns file
    Icns,
    /// SVG file
    Svg,
    /// Android VectorDrawable file
    VectorDrawable,
}

#[derive(Default)]
pub struct Icons {
    raster: Vec<RasterIcon>,
    other: Vec<NonRasterIcon>,
}

impl Icons {
    pub fn new(base_path: &Path, icon_paths: &[String]) -> Result<Self> {
        let mut raster_icons = Vec::new();
        let mut non_raster_icons = Vec::new();

        for icon in icon_paths {
            let icon_path = base_path
                .join(icon)
                .canonicalize()
                .with_context(|| format!("Failed to canonicalize icon path: {icon}"))?;

            if let Some(ext) = icon_path.extension() {
                match ext.to_string_lossy().as_ref() {
                    // Handle vector and compound formats
                    ext @ ("ico" | "icns" | "svg" | "xml") => {
                        let format = match ext {
                            "ico" => NonRasterIconKind::Ico,
                            "icns" => NonRasterIconKind::Icns,
                            "svg" => NonRasterIconKind::Svg,
                            "xml" => NonRasterIconKind::VectorDrawable,
                            _ => unreachable!(),
                        };
                        non_raster_icons.push(NonRasterIcon {
                            path: icon_path,
                            format,
                        });
                    }

                    // Handle raster formats
                    _ => {
                        let mut file = BufReader::new(File::open(&icon_path)?);
                        let format = reader_type(&mut file)?;
                        let size = reader_size(&mut file)?;
                        raster_icons.push(RasterIcon {
                            path: icon_path,
                            size,
                            format,
                        });
                    }
                }
            }
        }

        // Sort raster icons in descending order of size
        raster_icons.sort_by_key(|icon| icon.size.width + icon.size.height);
        raster_icons.reverse();

        Ok(Self {
            raster: raster_icons,
            other: non_raster_icons,
        })
    }
}
