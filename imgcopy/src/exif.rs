use crate::error::ImgcpError;
use crate::sync;
use rexif;
use std::path::{Path, PathBuf};

pub struct ExifHandler {
    date_time: String,
}

impl ExifHandler {
    pub fn new(path: &Path) -> Result<Self, ImgcpError> {
        let exif = rexif::parse_file(path).map_err(|source| ImgcpError::NoDateInExifFound {
            source: Some(source),
        })?;
        if let Some(date_time) = exif
            .entries
            .iter()
            .find(|&s| s.tag == rexif::ExifTag::DateTimeOriginal)
        {
            return Ok(ExifHandler {
                date_time: date_time.value.to_string(),
            });
        }
        Err(ImgcpError::NoDateInExifFound { source: None })
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
