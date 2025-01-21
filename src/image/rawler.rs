use std::path::Path;

use rawler::analyze::AnalyzerData;

use crate::{errors, template};

use super::{ImageLoader, Metadata};

const SUPPORTED_FILE_TYPES: [&str; 27] = [
    "ARI", "ARW", "CR2", "CR3", "CRM", "CRW", "DCR", "DCS", "DNG", "ERF", "IIQ", "KDC", "MEF",
    "MOS", "MRW", "NEF", "NRW", "ORF", "PEF", "RAF", "RAW", "RW2", "RWL", "SRW", "3FR", "FFF",
    "X3F",
];

pub struct RawlerImage {}

impl ImageLoader for RawlerImage {
    fn supports(extension: &str) -> bool {
        SUPPORTED_FILE_TYPES.contains(&extension)
    }

    fn render<P: AsRef<Path>>(
        ctx: &template::TemplateContext,
        path: P,
    ) -> Result<String, errors::Error> {
        let img = rawler::analyze::analyze_metadata(path.as_ref()).map_err(|e| {
            errors::system_with_internal(
                &format!("Could not load image file '{}'.", path.as_ref().display()),
                "Make sure that you are attempting to load a valid image file format.",
                e,
            )
        })?;

        match img.data {
            Some(AnalyzerData::Metadata(m)) => {
                let mut metadata = Metadata::new(path.as_ref());

                metadata.artist = m.raw_metadata.exif.artist;
                metadata.copyright = m.raw_metadata.exif.copyright;

                metadata.date_time = m.raw_metadata.exif.date_time_original;
                metadata.image_number = m.raw_metadata.exif.image_number.or(metadata.image_number);

                metadata.owner_name = m.raw_metadata.exif.owner_name;
                metadata.camera_make = m.raw_metadata.make.into();
                metadata.camera_model = m.raw_metadata.model.into();
                metadata.lens_make = m.raw_metadata.exif.lens_make;
                metadata.lens_model = m.raw_metadata.exif.lens_model;

                metadata.validate()?;

                Ok(ctx.render(&metadata))
            }
            _ => Err(errors::user(
                &format!(
                    "Could not load image metadata from '{}'.",
                    path.as_ref().display()
                ),
                "Make sure that the image file contains the necessary metadata.",
            )),
        }
    }
}
