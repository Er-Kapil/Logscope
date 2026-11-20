use clap::Parser;
use std::path::PathBuf;

/// Program to summarize the log files
#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the app
    pub app: String,
    /// Path to the file to process
    pub filepath: PathBuf,
}
pub fn take_input() -> Result<Args, String> {
    let args = Args::parse();
    if args.filepath.is_empty() {
        return Err("Empty file path".to_string());
    }
    let extension = args.filepath.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("txt") | Some("log") => {}
        _ => return Err("Invalid file format".to_string()),
    }
    return Ok(args);
}
