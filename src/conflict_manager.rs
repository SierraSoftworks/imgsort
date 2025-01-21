use std::path::{Path, PathBuf};

use sha2::Digest;

pub fn rename_no_conflict<S: AsRef<Path>, D: AsRef<Path>>(
    source: S,
    dest: D,
) -> std::io::Result<PathBuf> {
    let mut dest = dest.as_ref().to_path_buf();
    let base_filename = dest.file_stem().unwrap().to_string_lossy().to_string();
    let extension = dest.extension().unwrap().to_string_lossy().to_string();
    let mut conflict_suffix = None;

    while dest.exists() && !are_files_equal(source.as_ref(), dest.as_ref()) {
        conflict_suffix = match conflict_suffix {
            Some(i) => Some(i + 1),
            None => Some(1),
        };

        dest.set_file_name(format!(
            "{} ({}).{}",
            base_filename,
            conflict_suffix.unwrap(),
            extension
        ));
    }

    if conflict_suffix.is_some() && are_files_equal(source.as_ref(), dest.as_ref()) {
        std::fs::remove_file(source.as_ref())?;
    } else {
        std::fs::rename(source, dest.as_path())?;
    }

    Ok(dest)
}

fn are_files_equal<P: AsRef<Path>>(a: P, b: P) -> bool {
    match (file_hash(a), file_hash(b)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn file_hash<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut hasher = sha2::Sha256::new();
    let mut file = std::fs::File::open(path)?;
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(base16ct::lower::encode_string(&hash))
}
