use deunicode::AsciiChars;
use std::path::Path;

use crate::template::DataSource;

#[derive(Debug)]
pub struct Metadata<'a> {
    pub path: &'a Path,

    pub artist: Option<String>,
    pub copyright: Option<String>,

    pub date_time: Option<String>,
    pub image_number: Option<u32>,

    pub owner_name: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_make: Option<String>,
    pub lens_model: Option<String>,
}

impl<'a> Metadata<'a> {
    pub fn new(path: &'a Path) -> Self {
        Metadata {
            path,

            artist: None,
            copyright: None,

            date_time: None,
            image_number: Self::parse_image_number_from_path(path),

            owner_name: None,
            camera_make: None,
            camera_model: None,
            lens_make: None,
            lens_model: None,
        }
    }

    pub fn validate(&self) -> Result<(), crate::errors::Error> {
        if !matches!(self.get("year"), Some(y) if y.as_str().trim().len() == 4) {
            Err(human_errors::user(
                format!(
                    "The image '{}' does not have a valid date/time set.",
                    self.path.display()
                ),
                &["Make sure that the image has a valid date/time set in its metadata."],
            ))
        } else {
            Ok(())
        }
    }

    fn parse_image_number_from_path(path: &'a Path) -> Option<u32> {
        if let Some(stem) = path.file_stem().map(|s| s.to_string_lossy()) {
            // Takes any trailing numbers and parses them as a single contiguous value
            stem.chars()
                .rev()
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
                .parse()
                .ok()
        } else {
            None
        }
    }
}

impl DataSource for Metadata<'_> {
    fn get(&self, key: &str) -> Option<crate::template::Value> {
        match key {
            "name" => self
                .path
                .file_stem()
                .and_then(|name| name.to_str())
                .map(|v| v.into()),

            "artist" => self.artist.as_ref().map(|s| cleanup_string(s).into()),
            "copyright" => self.copyright.as_ref().map(|s| cleanup_string(s).into()),

            "number" => self.image_number.map(|n| n.to_string().into()),

            "owner.name" => self.owner_name.as_ref().map(|s| cleanup_string(s).into()),

            "camera.make" => self.camera_make.as_ref().map(|s| cleanup_string(s).into()),
            "camera.model" => self.camera_model.as_ref().map(|s| cleanup_string(s).into()),

            "lens.make" => self.lens_make.as_ref().map(|s| cleanup_string(s).into()),
            "lens.model" => self.lens_model.as_ref().map(|s| cleanup_string(s).into()),

            "year" => self
                .date_time
                .as_deref()
                .map(|date| &date[..4])
                .map(|v| v.into()),
            "month" => self
                .date_time
                .as_deref()
                .map(|date| &date[5..7])
                .map(|v| v.into()),
            "day" => self
                .date_time
                .as_deref()
                .map(|date| &date[8..10])
                .map(|v| v.into()),
            "date_time" => self
                .date_time
                .as_deref()
                .map(|v| v.replace(':', "").replace(' ', "T").into()),
            "date" => self
                .date_time
                .as_deref()
                .map(|v| &v[..10])
                .map(|v| v.replace(":", "-").into()),
            "time" => self
                .date_time
                .as_deref()
                .map(|v| &v[11..19])
                .map(|v| v.replace(':', "").into()),
            _ => None,
        }
    }
}

fn cleanup_string(s: &str) -> String {
    s.trim()
        .ascii_chars()
        .filter_map(|c| c)
        .filter(|c| !c.contains('\0'))
        .collect()
}
