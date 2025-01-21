use std::path::Path;

use crate::{errors, template};

use super::{ImageLoader, Metadata};

const SUPPORTED_FILE_TYPES: [&str; 6] = ["JPG", "JPEG", "TIF", "TIFF", "JIF", "JFIF"];

pub struct RexifImage {}

impl ImageLoader for RexifImage {
    fn supports(extension: &str) -> bool {
        SUPPORTED_FILE_TYPES.contains(&extension)
    }

    fn render<P: AsRef<Path>>(
        ctx: &template::TemplateContext,
        path: P,
    ) -> Result<String, errors::Error> {
        let img = rexif::parse_file(path.as_ref()).map_err(|e| {
            errors::system_with_internal(
                &format!("Could not load image file '{}'.", path.as_ref().display()),
                "Make sure that you are attempting to load a valid image file format.",
                e,
            )
        })?;

        let mut metadata = Metadata::new(path.as_ref());

        for entry in img.entries {
            match entry.tag {
                rexif::ExifTag::Copyright => {
                    metadata.copyright = entry.value.to_string().into();
                }
                rexif::ExifTag::UserComment => {
                    metadata.artist = entry.value.to_string().into();
                }

                rexif::ExifTag::DateTimeOriginal => {
                    metadata.date_time = entry.value.to_string().into();
                }

                rexif::ExifTag::Make => {
                    metadata.camera_make = entry.value.to_string().into();
                }
                rexif::ExifTag::Model => {
                    metadata.camera_model = entry.value.to_string().into();
                }
                rexif::ExifTag::LensMake => {
                    metadata.lens_make = entry.value.to_string().into();
                }
                rexif::ExifTag::LensModel => {
                    metadata.lens_model = entry.value.to_string().into();
                }

                _ => {}
            }
        }

        metadata.validate()?;

        Ok(ctx.render(&metadata))
    }
}
