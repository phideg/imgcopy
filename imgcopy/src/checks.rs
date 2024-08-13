use crate::error::ImgcpError;
use same_file::is_same_file;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

// If source dir was not provided we use the current directory
fn check_source_path(src: Option<&Path>) -> Result<PathBuf, ImgcpError> {
    if let Some(src) = src {
        if src.is_dir() {
            Ok(src.to_path_buf())
        } else {
            Err(ImgcpError::SourcePathNoDir {
                src: src.to_path_buf(),
            })
        }
    } else {
        Ok(env::current_dir()?)
    }
}

fn check_target_path(trg: &Path, ignore_non_empty_target: bool) -> Result<PathBuf, ImgcpError> {
    if trg.is_dir() {
        let is_target_empty = trg.read_dir()?.next().is_none();
        if !is_target_empty && !(ignore_non_empty_target) {
            return Err(ImgcpError::TargetDirNotEmpty {
                trg: trg.to_path_buf(),
            });
        }
    } else {
        // target directory does not exist try to create it
        fs::create_dir(trg).map_err(|source| ImgcpError::TargetDirNotCreated { source })?;
    }
    Ok(trg.to_path_buf())
}

pub fn check_paths(
    src: Option<&Path>,
    trg: &Path,
    ignore_non_empty_target: bool,
) -> Result<(PathBuf, PathBuf), ImgcpError> {
    let src = check_source_path(src)?;
    let trg = check_target_path(trg, ignore_non_empty_target)?;
    // check that source and target directory are not the same!
    if is_same_file(&trg, &src)? {
        return Err(ImgcpError::SrcNotAllowedAsTarget);
    }
    Ok((src, trg))
}
