use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Operation canceled")]
    Canceled,
    #[error("<source> = {src} is no valid directory")]
    SourceNoDir { src: std::path::PathBuf },
    #[error("<source> can not be used as <target> directory")]
    SrcNotAllowedAsTarget,
    #[error("No valid date found in exif metadata of image :(")]
    NoDateInExifFound,
}
