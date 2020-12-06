use rexif::ExifError;
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

    #[error("Could not copy file to target directory")]
    FileCopyFailed { source: std::io::Error },

    #[error("Failed to create target directory")]
    TargetDirNotCreated { source: std::io::Error },

    #[error("Source file could not be read")]
    SourceFileNotReadable { source: std::io::Error },

    #[error("The creation of a digest for file failed")]
    DigestNotCreated { source: Option<std::io::Error> },

    #[error("An unexpected IO error occurred")]
    UnexpectedIOErr(#[from] std::io::Error),
}
