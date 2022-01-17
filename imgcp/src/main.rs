use anyhow::{bail, Result};
use clap::{Parser, ValueHint};
use flexi_logger::{FileSpec, Logger};
use imgcopy::ImgcpError;
use promptly::prompt_default;
use std::path::PathBuf;

#[derive(clap::Parser)]
#[clap(author, about, version)]
struct Options {
    /// Move image files to target directory instead of copy
    #[clap(short, long)]
    move_files: bool,

    /// Suppress confirmation if target directory is not empty
    #[clap(short, long)]
    force: bool,

    /// Print info messages
    #[clap(short, long)]
    verbose: bool,

    /// Write a log file
    #[clap(short, long)]
    log: bool,

    /// Source directory
    #[clap(short = 's', short, long, parse(from_os_str), value_hint = ValueHint::AnyPath)]
    source: Option<PathBuf>,

    /// Target directory
    #[clap(parse(from_os_str), value_hint = ValueHint::AnyPath)]
    target: PathBuf,
}

fn main() -> Result<()> {
    let opts = Options::parse();
    let src = opts.source.as_deref();
    let logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().directory("log_files"))
        .print_message();
    if opts.log {
        logger.start()?;
    }
    match imgcopy::run(src, &opts.target, opts.move_files, opts.force, opts.verbose) {
        Err(ImgcpError::TargetDirNotEmpty { .. }) => {
            if !prompt_default(
                format!(
                    "Target dir {:?} is not empty, still continue?",
                    &opts.target
                ),
                true,
            )? {
                bail!("Operation aborted");
            } else {
                imgcopy::run(src, &opts.target, opts.move_files, true, opts.verbose)?;
            }
        }
        result => result?,
    }
    Ok(())
}
