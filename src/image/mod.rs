use std::path::Path;

use crate::{errors, template::TemplateContext};

mod metadata;
mod rawler;
mod rexif;

pub use metadata::Metadata;

pub trait ImageLoader {
    fn supports(extension: &str) -> bool;
    fn render<P: AsRef<Path>>(ctx: &TemplateContext, path: P) -> Result<String, errors::Error>;
}

pub fn render<P: AsRef<Path>>(
    ctx: &TemplateContext,
    path: P,
) -> Option<Result<String, errors::Error>> {
    let extension = path
        .as_ref()
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default();

    if rawler::RawlerImage::supports(&extension.to_uppercase()) {
        return Some(rawler::RawlerImage::render(ctx, path));
    }

    if rexif::RexifImage::supports(&extension.to_uppercase()) {
        return Some(rexif::RexifImage::render(ctx, path));
    }

    None
}
