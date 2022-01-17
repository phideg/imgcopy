use ffmpeg_next as ffmpeg;
use rexif::ExifError;
use std::array::TryFromSliceError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImgcpError {
    #[error("Operation canceled")]
    Canceled,

    #[error("Target directory {trg} is not empty")]
    TargetDirNotEmpty { trg: std::path::PathBuf },

    #[error("<source> = {src} is no valid directory")]
    SourcePathNoDir { src: std::path::PathBuf },

    #[error("<source> can not be used as <target> directory")]
    SrcNotAllowedAsTarget,

    #[error("No valid date found in exif metadata of image :(")]
    NoDateInExifFound { source: Option<ExifError> },

    #[error("Could not copy file '{file}' to target directory {trg}")]
    FileCopyFailed {
        source: std::io::Error,
        file: std::path::PathBuf,
        trg: std::path::PathBuf,
    },

    #[error("Failed to create target directory")]
    TargetDirNotCreated { source: std::io::Error },

    #[error("Source file could not be read")]
    SourceFileNotReadable { source: std::io::Error },

    #[error("The creation of a digest for file failed")]
    HasherNotCreated { source: std::io::Error },

    #[error("The creation of a digest for file failed")]
    DigestNotCreated { source: TryFromSliceError },

    #[error("Error in ffmpeg occurred")]
    FfmpegError { source: ffmpeg::Error },

    #[error("Could not find key 'creation_time' in '{path}'")]
    NoDateVideoFound { path: std::path::PathBuf },

    #[error("An unexpected IO error occurred")]
    UnexpectedIOErr(#[from] std::io::Error),
}
