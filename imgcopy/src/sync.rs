use crate::checks;
use crate::error::ImgcpError;
use crate::exif::ExifHandler;
use log::{debug, error, info, warn};
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::DirEntry;
use walkdir::WalkDir;

pub static UNKOWN_FOLDER: &str = "ToDo";

fn create_digest(mut file: &mut fs::File) -> Result<[u8; 20], ImgcpError> {
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher).map_err(|source| ImgcpError::HasherNotCreated { source })?;
    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|source| ImgcpError::DigestNotCreated { source })
}

fn init_file_map(trg: &Path) -> Result<HashMap<[u8; 20], PathBuf>, ImgcpError> {
    let mut file_map: HashMap<[u8; 20], PathBuf> = HashMap::new();
    for entry in WalkDir::new(trg)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let mut file = fs::File::open(entry.path())
            .map_err(|source| ImgcpError::SourceFileNotReadable { source })?;
        let digest = create_digest(&mut file)?;
        debug!("Inserting {digest:?} for {entry:?}");
        file_map.insert(digest, entry.path().to_path_buf());
    }
    Ok(file_map)
}

fn prepare_path(src: &DirEntry, trg: &Path) -> Result<PathBuf, ImgcpError> {
    let mut target_path = if let Ok(exif) = ExifHandler::new(src.path()) {
        trg.join(exif.date_to_path())
    } else {
        // sadly no image metadata to read just put it into the "todo" folder
        trg.join(UNKOWN_FOLDER)
    };
    if !target_path.exists() {
        fs::create_dir_all(&target_path)
            .map_err(|source| ImgcpError::TargetDirNotCreated { source })?
    }
    let file_name = src.path().file_name().unwrap().to_str().unwrap();
    target_path.push(file_name);
    let mut increment = 0;
    while target_path.exists() {
        target_path.pop();
        target_path.push(format!("{}_{}", file_name, increment));
        increment += 1;
    }
    Ok(target_path)
}

fn copy_file(src: &Path, trg: &Path, do_move: bool) -> Result<(), std::io::Error> {
    let mut source = fs::File::open(src)?;
    let mut target = fs::File::create(trg)?;
    io::copy(&mut source, &mut target).and_then(|r| {
        if do_move {
            if r > 0 {
                fs::remove_file(src)?
            } else {
                warn!("{:?} was not moved! Is it empty?", src.to_path_buf());
            }
        }
        Ok(())
    })?;
    Ok(())
}

fn target_exists(
    existing_targets: &HashMap<[u8; 20], PathBuf>,
    hash: &[u8; 20],
    entry: &DirEntry,
) -> bool {
    if let Some(existing_file) = existing_targets.get(hash) {
        let existing_mdata = fs::metadata(existing_file.as_path());
        let entry_mdata = fs::metadata(entry.path());
        if let (Ok(existing_mdata), Ok(entry_mdata)) = (existing_mdata, entry_mdata) {
            if existing_mdata.len() == entry_mdata.len() {
                info!(
                    "skipping {0:?}: duplicate {0:?} -> {1:?}",
                    entry.path(),
                    existing_file.as_path()
                );
            } else {
                warn!(
                    "skipping {:?}: same hash but files have different lenghts -> hash collision!",
                    entry.path()
                );
            }
        } else {
            error!(
                "skipping {:?}: file metadata could not be read",
                entry.path()
            );
        }
        return true;
    }
    false
}

pub fn run(
    src: Option<&Path>,
    trg: &Path,
    do_move: bool,
    ignore_non_empty_target: bool,
    verbose: bool,
) -> Result<(), ImgcpError> {
    let (src, trg) = checks::check_paths(src, trg, ignore_non_empty_target)?;
    let mut file_map = init_file_map(&trg)?;

    for entry in WalkDir::new(src)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        // open file and crate sha1 hash to keep track of copied files
        let mut file =
        if let Ok(open_file) = fs::File::open(entry.path()) {
            open_file
        } else {
            warn!("skipping {:?}: file could not be read", entry.path());
            continue;
        };

        let hash = create_digest(&mut file)?;

        // skip file:
        // a) if it already exists in target folder
        // b) if file metadata can not be read
        if target_exists(&file_map, &hash, &entry) {
            continue;
        }

        // read image metadata and prepare target path
        let target_path = prepare_path(&entry, &trg)?;

        // finally move file into target folder
        if verbose {
            println!("{:?} -> {:?}", entry.path(), &target_path);
        }

        info!("{:?} -> {:?}", entry.path(), &target_path);
        if let Err(err) = copy_file(entry.path(), &target_path, do_move).map_err(|source| {
            ImgcpError::FileCopyFailed {
                source,
                file: entry.into_path(),
                trg: target_path.to_path_buf(),
            }
        }) {
            error!("error during file copy {err}");
        }
        file_map.insert(hash, target_path.to_path_buf());
    }
    Ok(())
}
