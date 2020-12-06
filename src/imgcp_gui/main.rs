use anyhow::Result;
use native_dialog::*;
use same_file::is_same_file;
use std::fs;

use imgcopy;

fn main() -> Result<()> {
    // introduce next steps of the program
    let dialog = MessageAlert {
        title: "How To",
        text: "After clicking OK you will need to choose first the source directory and afterwards the target directory in which the images should be copied.",
        typ: MessageType::Info,
    };
    if dialog.show().is_err() {
        return Ok(());
    };

    // get and check source directory
    let dialog = OpenSingleDir { dir: None };
    let source;
    let result = dialog.show()?;
    if let Some(result) = result {
        source = result;
    } else {
        return Err(imgcopy::AppError::Canceled)?;
    }
    // If target dir is not empty ask for confirmation to continue
    if !source.is_dir() || source.read_dir()?.next().is_none() {
        dbg!("dir empty");
        return Err(imgcopy::AppError::Canceled)?;
    }

    // get and check target directory
    let target;
    let dialog = OpenSingleDir { dir: None };
    let result = dialog.show()?;
    if let Some(result) = result {
        target = result;
    } else {
        return Err(imgcopy::AppError::Canceled)?;
    }
    // If target dir is not empty ask for confirmation to continue
    if target.is_dir() {
        if !target.read_dir()?.next().is_none() {
            let dialog = MessageConfirm {
                title: "Confirmation",
                text: &format!("Target dir {:?} is not empty, still continue?", &target),
                typ: MessageType::Info,
            };
            let result = dialog.show();
            if result.is_err() || result.unwrap() != true {
                return Err(imgcopy::AppError::Canceled)?;
            };
        }
    } else {
        // target directory does not exist try to create it
        fs::create_dir(&target)?;
    }

    // check that source and target directory are not the same!
    if is_same_file(&target, &source)? {
        let dialog = MessageAlert {
            title: "Error",
            text: "Target directory must be different from source directory",
            typ: MessageType::Error,
        };
        if dialog.show().is_err() {
            return Err(imgcopy::AppError::SrcNotAllowedAsTarget)?;
        };
    }

    // execute the sync operation
    imgcopy::run(&source, &target, false)
}
