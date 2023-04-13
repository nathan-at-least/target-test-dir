use anyhow_std::PathAnyhow;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

/// Return a [Path] to a `test-data` directory inside the crate or workspace `target/` directory
///
/// This function relies on the `$CARGO_MANIFEST_DIR` environment variable, which is set in unit
/// tests, integration tests, and doc tests. If it cannot find the `target/` directory it
/// will panic.
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

    DIR.get_or_init(|| match init_base_test_dir() {
        Ok(pathbuf) => pathbuf,
        Err(e) => panic!("Failed to initialize test data directory:\n{e:?}"),
    })
    .as_path()
}

fn init_base_test_dir() -> anyhow::Result<PathBuf> {
    let pb = get_target_dir()?.join("test-data");
    if pb.is_dir() {
        eprintln!("Removing {:?} from previous test run...", pb.display());
        pb.remove_dir_all_anyhow()?;
    }
    eprintln!("Creating {:?}...", pb.display());
    pb.create_dir_anyhow()?;
    Ok(pb)
}

/// Attempt to return the crate or workspace `target/` directory
///
/// Precondition: the executable path resides within the `target/` directory. This is the case for
/// standard `cargo test` runs, AFAIK.
fn get_target_dir() -> anyhow::Result<PathBuf> {
    let mut not_found = vec![];
    for candidate_parent in Path::new(&std::env::var("CARGO_MANIFEST_DIR")?).ancestors() {
        let candidate = candidate_parent.join("target");
        if candidate.is_dir() {
            return Ok(candidate);
        } else {
            not_found.push(candidate);
        }
    }

    let mut e = anyhow::anyhow!("Cargo 'target/' directory not found.");
    for candidate in not_found {
        e = e.context(format!("candidate: {:?}", candidate.display()));
    }
    Err(e)
}

#[cfg(test)]
mod tests;
