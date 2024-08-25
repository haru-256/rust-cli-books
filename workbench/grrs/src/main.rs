use anyhow::{self, Context};
use clap::Parser;
use log::{error, info};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

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
    env_logger::init();

    let args = Cli::parse();

    let f =
        File::open(&args.path).with_context(|| format!("failed to open file: {:?}", &args.path))?;
    let br = BufReader::new(f);

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    info!(
        "searching for pattern: {} from file: {:?}",
        args.pattern, args.path
    );
    let pb = indicatif::ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    for line in br.lines() {
        let content = match line {
            Ok(l) => l,
            Err(e) => {
                error!("failed to read line: {:?}", e);
                return Err(e.into());
            }
        };

        if content.contains(&args.pattern) {
            writeln!(handle, "{}", content)?;
        }
        thread::sleep(Duration::from_secs(1));
    }
    pb.finish_and_clear();

    handle.flush()?;

    Ok(())
}
