use anyhow::{bail, Result};
use imgcopy::ImgcpError;
use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() -> Result<()> {
    // introduce next steps of the program
    if MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("How To")
        .set_text(
            "After clicking OK you will need to choose the \
                   source directory and afterwards the target directory \
                   in which the images should be copied.",
        )
        .show_alert()
        .is_err()
    {
        return Ok(());
    };

    // get and check source directory
    let source;
    if let Ok(Some(result)) = FileDialog::new().show_open_single_dir() {
        source = result;
        if !source.is_dir() || source.read_dir()?.next().is_none() {
            dbg!("source dir is empty");
            return Err(imgcopy::ImgcpError::Canceled.into());
        }
    } else {
        return Err(imgcopy::ImgcpError::Canceled.into());
    }

    // get and check target directory
    if let Ok(Some(target)) = FileDialog::new().show_open_single_dir() {
        let error;
        match imgcopy::run(Some(source.as_path()), &target, false, false) {
            Err(ImgcpError::TargetDirNotEmpty { .. }) => {
                let result = MessageDialog::new()
                    .set_title("Confirmation")
                    .set_text(&format!(
                        "Target dir {:?} is not empty, still continue?",
                        &target
                    ))
                    .set_type(MessageType::Info)
                    .show_confirm();
                if result.is_ok() && result.unwrap() {
                    error = imgcopy::run(Some(source.as_path()), &target, false, true);
                } else {
                    bail!("Operation aborted");
                }
            }
            result => error = result,
        }

        if error.is_err() {
            let _ = MessageDialog::new()
                .set_title("Error Occurred")
                .set_text(&format!("{:?}", &error))
                .set_type(MessageType::Info)
                .show_alert();
        }
    }

    Ok(())
}
