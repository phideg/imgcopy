use crate::error::AppError;
use crate::sync;
use anyhow::Result;
use rexif;
use std::path::{Path, PathBuf};

pub struct ExifHandler {
    date_time: String,
}

impl ExifHandler {
    pub fn new(path: &Path) -> Result<Self> {
        let exif = rexif::parse_file(path)?;
        if let Some(date_time) = exif
            .entries
            .iter()
            .find(|&s| s.tag == rexif::ExifTag::DateTimeOriginal)
        {
            return Ok(ExifHandler {
                date_time: date_time.value.to_string(),
            });
        }
        return Err(AppError::NoDateInExifFound)?;
    }

    pub fn date_to_path(&self) -> PathBuf {
        if self.date_time.len() >= 10 {
            let mut path = PathBuf::new();
            path.push(&self.date_time[0..4]);
            path.push(&self.date_time[5..7]);
            path.push(&self.date_time[8..10]);
            path
        } else {
            PathBuf::from(sync::UNKOWN_FOLDER)
        }
    }
}
