use anyhow::{bail, Result};
use clap::{Clap, ValueHint};
use promptly::prompt_default;
use std::path::PathBuf;
use imgcopy::ImgcpError;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct Options {
    /// Move image files to target directory instead of copy
    #[clap(short, long)]
    move_files: bool,

    /// Suppress confirmation if target directory is not empty
    #[clap(short, long)]
    force: bool,

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
    match imgcopy::run(src, &opts.target, opts.move_files, opts.force) {
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
                imgcopy::run(src, &opts.target, opts.move_files, true)?;
            }
        }
        result => result?,
    }
    Ok(())
}
