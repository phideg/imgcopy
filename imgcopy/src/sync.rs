use crate::checks;
use crate::error::ImgcpError;
use crate::exif::ExifHandler;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub static UNKOWN_FOLDER: &str = "ToDo";

fn create_digest(mut file: &mut fs::File) -> Result<[u8; 20], ImgcpError> {
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher).map_err(|source| ImgcpError::DigestNotCreated {
        source: Some(source),
    })?;
    hasher
        .finalize()
        .as_slice()
        .try_into()
        .map_err(|_| ImgcpError::DigestNotCreated { source: None })
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
        file_map.insert(create_digest(&mut file)?, PathBuf::from(entry.path()));
    }
    Ok(file_map)
}

fn prepare_path(mut target_path: PathBuf, file_name: &str) -> Result<PathBuf, ImgcpError> {
    if !target_path.exists() {
        fs::create_dir_all(&target_path)
            .map_err(|source| ImgcpError::TargetDirNotCreated { source })?
    }
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
                println!("{:?} was not moved! Is it empty?", src.to_path_buf());
            }
        }
        Ok(())
    })?;
    Ok(())
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
        let mut file;
        if let Ok(open_file) = fs::File::open(entry.path()) {
            file = open_file;
        } else {
            continue;
        }

        let hash = create_digest(&mut file)?;

        // skip file:
        // a) if it already exists in target folder
        // b) if file metadata can not be read
        if let Some(existing_file) = file_map.get(&hash) {
            let existing_mdata = fs::metadata(existing_file.as_path());
            let entry_mdata = fs::metadata(entry.path());
            if existing_mdata.is_ok()
                && entry_mdata.is_ok()
                && existing_mdata.unwrap().len() == entry_mdata.unwrap().len()
            {
                // FIXME: write log instead of println
                println!(
                    "duplicate {:?} -> {:?}",
                    entry.path(),
                    existing_file.as_path()
                );
            } else {
                // FIXME: write log instead of println
                println!(
                    "skipping {:?} could not read file metadata :(",
                    entry.path()
                );
            }
            continue;
        }

        // read image metadata and generate and prepare target path
        let target_path = if let Ok(exif) = ExifHandler::new(entry.path()) {
            prepare_path(
                trg.join(exif.date_to_path()),
                &entry.path().file_name().unwrap().to_str().unwrap(),
            )?
        } else {
            // sadly no image metadata to read just put it into the "todo" folder
            prepare_path(
                trg.join(UNKOWN_FOLDER),
                &entry.path().file_name().unwrap().to_str().unwrap(),
            )?
        };

        // finally move file into target folder
        if verbose {
            println!("{:?} -> {:?}", entry.path(), &target_path);
        }
        copy_file(entry.path(), &target_path, do_move).map_err(|source| {
            ImgcpError::FileCopyFailed {
                source,
                file: entry.into_path(),
                trg: target_path.to_path_buf(),
            }
        })?;
        file_map.insert(hash, target_path.to_path_buf());
    }
    Ok(())
}
