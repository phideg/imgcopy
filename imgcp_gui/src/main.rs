use anyhow::{bail, Result};
use imgcopy::ImgcpError;
use native_dialog::*;

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
        return Err(imgcopy::ImgcpError::Canceled.into());
    }
    // If target dir is not empty ask for confirmation to continue
    if !source.is_dir() || source.read_dir()?.next().is_none() {
        dbg!("dir empty");
        return Err(imgcopy::ImgcpError::Canceled.into());
    }

    // get and check target directory
    let target;
    let dialog = OpenSingleDir { dir: None };
    let result = dialog.show()?;
    if let Some(result) = result {
        target = result;
    } else {
        return Err(imgcopy::ImgcpError::Canceled.into());
    }

    let error;
    match imgcopy::run(Some(source.as_path()), &target, false, false) {
        Err(ImgcpError::TargetDirNotEmpty { .. }) => {
            let dialog = MessageConfirm {
                title: "Confirmation",
                text: &format!("Target dir {:?} is not empty, still continue?", &target),
                typ: MessageType::Info,
            };
            let result = dialog.show();
            if result.is_ok() && result.unwrap() {
                error = imgcopy::run(Some(source.as_path()), &target, false, true);
            } else {
                bail!("Operation aborted");
            }
        }
        result => error = result,
    }

    if error.is_err() {
        let dialog = MessageAlert {
            title: "Error Occurred",
            text: &format!("{:?}", &error),
            typ: MessageType::Info,
        };
        let _ = dialog.show();
    }

    Ok(())
}
