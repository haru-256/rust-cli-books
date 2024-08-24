use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author = "haru256",
    version = "0.1.0",
    about = "grep by rust",
    long_about = None
)]
struct Cli {
    #[arg(value_name = "PATTERN", value_parser=validate_pattern)]
    pattern: String,
    #[arg(value_name = "PATH", value_parser=validate_path)]
    path: PathBuf,
}

fn validate_pattern(s: &str) -> Result<(), String> {
    if s.is_empty() {
        Err(String::from("pattern is empty"))
    } else {
        Ok(())
    }
}

fn validate_path(s: &str) -> Result<(), String> {
    if PathBuf::from(s).exists() {
        Ok(())
    } else {
        Err(String::from("path not found"))
    }
}

fn main() {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
