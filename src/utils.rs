use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

pub fn read_file_contents(path: &str) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("Failed to open file: {}", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read from file: {}", path))?;
    Ok(contents)
}
