use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

/// The first time this is called in a process, it will remove any pre-existing directory. This
/// design aims to leave test data available for inspection after a test run, while also ensuring
/// all of the contents come from the same test process run.
pub fn get_base_test_dir() -> &'static Path {
    static DIR: OnceCell<PathBuf> = OnceCell::new();

    DIR.get_or_init(|| init_base_test_dir().expect("could not initialize base test data directory"))
        .as_path()
}

fn init_base_test_dir() -> std::io::Result<PathBuf> {
    let pb = get_target_dir()?.join("test-data");
    if pb.is_dir() {
        eprintln!("Removing {:?} from previous test run...", pb.display());
        std::fs::remove_dir_all(&pb)?;
    }
    eprintln!("Creating {:?}...", pb.display());
    std::fs::create_dir(&pb)?;
    Ok(pb)
}

/// Attempt to return the crate or workspace `target/` directory
///
/// Precondition: the executable path resides within the `target/` directory. This is the case for
/// standard `cargo test` runs, AFAIK.
fn get_target_dir() -> std::io::Result<PathBuf> {
    for candidate in std::env::current_exe()?.ancestors() {
        if candidate.is_dir() && candidate.file_name().and_then(|os| os.to_str()) == Some("target")
        {
            return Ok(candidate.to_path_buf());
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Cargo 'target/' directory not found.",
    ))
}

#[cfg(test)]
mod tests;

