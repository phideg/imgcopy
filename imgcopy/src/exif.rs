use crate::error::ImgcpError;
use crate::sync;
use ffmpeg_next as ffmpeg;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub struct ExifHandler {
    date_time: String,
}

impl ExifHandler {
    pub fn new(path: &Path) -> Result<Self, ImgcpError> {
        match path.extension().and_then(OsStr::to_str) {
            Some("mp4" | "MP4" | "mov" | "MOV") => Ok(Self::new_from_video(path)?),
            _ => Ok(Self::new_from_image(path)?),
        }
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

    fn new_from_image(path: &Path) -> Result<Self, ImgcpError> {
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

    fn new_from_video(path: &Path) -> Result<Self, ImgcpError> {
        match ffmpeg::init() {
            Ok(_) => {
                if let Ok(context) = ffmpeg::format::input(&path) {
                    for (key, value) in context.metadata().iter() {
                        if key == "creation_time" {
                            return Ok(ExifHandler {
                                date_time: value.to_string(),
                            });
                        }
                    }
                }
                Err(ImgcpError::NoDateVideoFound {
                    path: path.to_path_buf(),
                })
            }
            Err(err) => Err(ImgcpError::FfmpegError { source: err }),
        }
    }
}
