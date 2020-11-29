use crate::exif::ExifHandler;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub static UNKOWN_FOLDER: &str = "ToDo";

fn create_digest(mut file: &mut fs::File) -> Result<[u8; 20]> {
    let mut hasher = Sha1::new();
    io::copy(&mut file, &mut hasher)?;
    let digest = hasher.finalize().as_slice().try_into()?;
    Ok(digest)
}

fn init_file_map(trg: &Path) -> Result<HashMap<[u8; 20], PathBuf>> {
    let mut file_map: HashMap<[u8; 20], PathBuf> = HashMap::new();
    for entry in WalkDir::new(trg)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let mut file = fs::File::open(entry.path())?;
        file_map.insert(create_digest(&mut file)?, PathBuf::from(entry.path()));
    }
    Ok(file_map)
}

fn prepare_path(mut target_path: PathBuf, file_name: &str) -> Result<PathBuf> {
    if !target_path.exists() {
        fs::create_dir_all(&target_path)?
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

fn copy_file(src: &Path, trg: &Path, do_move: bool) -> Result<()> {
    fs::copy(src, trg).and_then(|r| {
        if do_move {
            fs::remove_file(src)?
        }
        Ok(r)
    })?;
    Ok(())
}

pub fn run(src: &Path, trg: &Path, do_move: bool) -> Result<()> {
    let mut file_map = init_file_map(trg)?;

    for entry in WalkDir::new(src)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        // open file and crate sha1 hash to keep track of copied files
        let mut file = fs::File::open(entry.path())?;
        let hash = create_digest(&mut file)?;

        // skip file if it already exists in target folder
        if let Some(existing_file) = file_map.get(&hash) {
            if fs::metadata(existing_file.as_path())?.len() == fs::metadata(entry.path())?.len() {
                println!(
                    "duplicate {:?} -> {:?}",
                    entry.path(),
                    existing_file.as_path()
                );
                continue;
            }
        }

        // read image metadata and generate and prepare target path
        let target_path = if let Ok(exif) = ExifHandler::new(entry.path()) {
            prepare_path(
                PathBuf::from(trg).join(exif.date_to_path()),
                &entry.path().file_name().unwrap().to_str().unwrap(),
            )?
        } else {
            // sadly no image metadata to read just put it into the "todo" folder
            prepare_path(
                PathBuf::from(trg).join(UNKOWN_FOLDER),
                &entry.path().file_name().unwrap().to_str().unwrap(),
            )?
        };

        // finally move file into target folder
        copy_file(entry.path(), &target_path, do_move)?;
        println!("{:?} -> {:?}", entry.path(), &target_path);
        file_map.insert(hash, target_path);
    }
    Ok(())
}
