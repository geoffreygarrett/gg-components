use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Copies all `.rs` files from this crate’s `src/` into
/// `<target>/.my-component-crate/rs/`.
///
/// In your consuming app, you'd call this in **its** build.rs,
/// passing in its `target` dir as `install_files_to(&target_dir)`.
pub fn install_files_to(target_dir: &Path) {
    // We'll place everything under: target/.my-component-crate/rs
    let out_dir = target_dir.join(format!(".{}", env!("CARGO_CRATE_NAME"))).join("rs");
    fs::create_dir_all(&out_dir)
        .expect(format!("Failed to create .{}/rs directory", env!("CARGO_CRATE_NAME")).as_str());

    // We'll start scanning from *this* crate's source folder:
    // `CARGO_MANIFEST_DIR` = path to the crate that compiled this code
    let crate_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");

    // Use WalkDir to recursively walk `src/`
    for entry in WalkDir::new(&crate_src) {
        let entry = entry.expect("Failed to read entry in src/");
        // Only copy if it's a file ending with `.rs`
        if entry.file_type().is_file()
            && entry
            .path()
            .extension()
            .map(|ext| ext == "rs")
            .unwrap_or(false)
        {
            let source_path = entry.path();
            // Build the relative path from `crate_src`
            let rel_path = source_path.strip_prefix(&crate_src).unwrap();
            let dest_path = out_dir.join(rel_path);

            // Ensure subdirectories exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)
                    .expect("Failed to create parent dirs for a .rs file");
            }

            // Copy file
            fs::copy(source_path, &dest_path).unwrap_or_else(|e| {
                panic!(
                    "Failed to copy {} to {}: {}",
                    source_path.display(),
                    dest_path.display(),
                    e
                )
            });
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_install_files_to() {
        // Get the target directory from the environment
        let target_dir = ".";
        let target_path = Path::new(&target_dir);

        // Call the function to install files
        install_files_to(target_path);

        // Check if the files were copied correctly
        assert!(target_path.join(format!(".{}", env!("CARGO_CRATE_NAME")))
            .join("rs")
            .exists());
    }
}