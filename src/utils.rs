use std::path::Path;
use std::io::{Result};
use std::fs::{self};

pub fn create_dir() -> Result<()> {
    let path = Path::new(".todos");
    fs::create_dir_all(path)?;
    Ok(())
}
