use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

/// Return a [Path] to a `test-data` directory inside the crate or workspace `target/` directory
///
/// This function implicitly assumes the current executable resides within the `target/` directory
/// which is the case for test binaries. If it cannot find the `target/` directory it will panic.
///
/// It prints diagnostic information to `stderr`.
///
/// The first time this is called in a process, it will remove any pre-existing `test-data`
/// directory, then create a new directory. This design aims to leave test data available
/// for inspection after a test run, while also ensuring all of the contents come from the same
/// test process run. This function is thread-safe via [OnceCell], which supports the primary use
/// case of being used for multiple `#[test]` functions which may be invoked concurrently.
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
