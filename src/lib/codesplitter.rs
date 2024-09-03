use std::fs;
use std::io::{self, Write};
use regex::Regex;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct SplitMetadata {
    filename: String,
    start_index: usize,
    end_index: usize,
}

fn main() -> io::Result<()> {
    let input_file = "input.ts";
    let output_dir = "output_chunks/";
    let metadata_file = "metadata.json";

    // Read the TypeScript file
    let content = fs::read_to_string(input_file)?;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Define regex patterns for different code structures
    let re_function = Regex::new(r"function\s+(\w+)\s*\(").unwrap();
    let re_class = Regex::new(r"class\s+(\w+)").unwrap();
    let re_import = Regex::new(r"import\s+.*").unwrap();

    let mut last_match_end = 0;
    let mut chunk_index = 0;
    let mut metadata = Vec::new();

    // Split by functions or classes, keeping imports together
    for mat in re_function.find_iter(&content).chain(re_class.find_iter(&content)) {
        let start = mat.start();
        if start > last_match_end {
            let chunk = &content[last_match_end..start];
            if !chunk.trim().is_empty() {
                let chunk_filename = format!("{}/chunk_{}.ts", output_dir, chunk_index);
                fs::write(&chunk_filename, chunk)?;
                metadata.push(SplitMetadata {
                    filename: chunk_filename,
                    start_index: last_match_end,
                    end_index: start,
                });
                chunk_index += 1;
            }
        }
        last_match_end = mat.end();
    }

    // Write remaining content after the last match
    if last_match_end < content.len() {
        let chunk = &content[last_match_end..];
        if !chunk.trim().is_empty() {
            let chunk_filename = format!("{}/chunk_{}.ts", output_dir, chunk_index);
            fs::write(&chunk_filename, chunk)?;
            metadata.push(SplitMetadata {
                filename: chunk_filename,
                start_index: last_match_end,
                end_index: content.len(),
            });
        }
    }

    // Write metadata to a JSON file
    let metadata_json = json!(metadata);
    fs::write(metadata_file, serde_json::to_string_pretty(&metadata_json)?)?;

    println!("Splitting done. Check the '{}' directory and '{}' for metadata.", output_dir, metadata_file);
    Ok(())
}