use log::{error, info, warn};
use std::{collections::HashSet, path::PathBuf};

use clap::Parser;

mod config;
mod conflict_manager;
mod errors;
mod image;
mod template;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    #[arg(short, long)]
    audit: bool,
}

fn main() {
    colog::init();

    let args = Args::parse();

    match run(args) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<(), errors::Error> {
    let config = config::Config::load(args.config)?;
    let template = template::TemplateContext::new(&config.template)
        .with_transform("lowercase", template::transform(|s| s.to_lowercase()))
        .with_transform("uppercase", template::transform(|s| s.to_uppercase()))
        .with_transform(
            "path_safe",
            template::transform(|s| s.replace(['/', '\\', ':', ';', '#'], "")),
        )
        .with_transform("trim", template::transform(|s| s.trim().to_owned()));

    let mut written_files = HashSet::new();

    for entry in walkdir::WalkDir::new(&config.source)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        if written_files.contains(entry.path()) {
            continue;
        }

        match image::render(&template, entry.path()) {
            Some(Ok(target)) => {
                let mut target = config.target.join(target);

                if let Some(ext) = entry.path().extension() {
                    target = target.with_extension(ext);
                }

                if target == entry.path() {
                    continue;
                }

                info!("mv '{}' '{}'", entry.path().display(), target.display());

                if !args.audit {
                    std::fs::create_dir_all(target.parent().unwrap()).map_err(|e| errors::user_with_internal(
                        &format!("Unable to create directory '{}'.", target.parent().unwrap().display()),
                        "Make sure that you've got permission to create this directory and try again.",
                        e))?;

                    let written_path = conflict_manager::rename_no_conflict(entry.path(), &target)
                        .map_err(|e| {
                            errors::user_with_internal(
                            &format!(
                                "Failed to move '{}' to '{}'",
                                entry.path().display(),
                                target.display()
                            ),
                            "Make sure that you have permission to move the image and try again.",
                            e,
                        )
                        })?;

                    written_files.insert(written_path);
                }
            }
            Some(Err(e)) => warn!("Error: {}", e),
            None => {}
        }
    }

    Ok(())
}
