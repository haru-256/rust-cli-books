use anyhow::{self, Context};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
struct Cli {
    // #[arg(value_name = "PATTERN", value_parser=validate_pattern)]
    #[arg(value_name = "PATTERN", help="Search this pattern", value_parser=clap::builder::NonEmptyStringValueParser::new())]
    pattern: String,
    #[arg(value_name = "PATH", help="File path for search", value_parser=validate_path)]
    path: PathBuf,
}

fn validate_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        Ok(path)
    } else {
        Err(String::from("path not found"))
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let f =
        File::open(&args.path).with_context(|| format!("failed to open file: {:?}", &args.path))?;
    let br = BufReader::new(f);

    for line in br.lines() {
        let content = match line {
            Ok(l) => l,
            Err(e) => {
                return Err(e.into());
            }
        };

        if content.contains(&args.pattern) {
            println!("{}", content);
        }
    }

    Ok(())
}
