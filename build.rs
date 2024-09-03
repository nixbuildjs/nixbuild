use std::fs;
use std::path::Path;

fn main() {
    // Get the source directory.
    let src_dir: &Path = Path::new("src");

    // Iterate over all files in the src directory.
    if let Ok(entries) = fs::read_dir(src_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path: std::path::PathBuf = entry.path();
            if path.extension().and_then(|s: &std::ffi::OsStr| s.to_str()) == Some("rs") {
                // Print the path to the console to force a rebuild if the file changes.
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}