use anyhow::Result;
use clap::{Clap, ValueHint};
use promptly::prompt_default;
use same_file::is_same_file;
use std::env;
use std::fs;
use std::path::PathBuf;

use imgcopy;

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
    let mut opts = Options::parse();

    // If source dir was not provided we use the current directory
    if opts.source.is_none() {
        opts.source = Some(env::current_dir()?);
    } else if !(opts.source.as_ref().unwrap().is_dir()) {
        return Err(imgcopy::AppError::SourceNoDir {
            src: opts.source.unwrap(),
        })?;
    }

    // If target dir is not empty ask for confirmation to continue
    if opts.target.is_dir() {
        let is_target_empty = opts.target.read_dir()?.next().is_none();
        if !is_target_empty
            && !(opts.force
                || prompt_default(
                    format!(
                        "Target dir {:?} is not empty, still continue?",
                        &opts.target
                    ),
                    true,
                )?)
        {
            return Err(imgcopy::AppError::Canceled)?;
        }
    } else {
        // target directory does not exist try to create it
        fs::create_dir(&opts.target)?;
    }

    // check that source and target directory are not the same!
    if is_same_file(&opts.target, opts.source.as_deref().unwrap())? {
        return Err(imgcopy::AppError::SrcNotAllowedAsTarget)?;
    }

    // execute the sync operation
    imgcopy::run(&opts.source.unwrap(), &opts.target, opts.move_files)
}
