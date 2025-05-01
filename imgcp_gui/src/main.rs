use anyhow::{Result, bail};
use imgcp_lib::ImgcpError;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

fn main() -> Result<()> {
    // introduce next steps of the program
    if MessageDialog::new()
        .set_level(MessageLevel::Info)
        .set_title("How To")
        .set_description(
            "After clicking OK you will need to choose the \
             source directory and afterwards the target directory \
             in which the images should be copied.",
        )
        .set_buttons(MessageButtons::OkCancel)
        .show()
        == MessageDialogResult::Cancel
    {
        return Ok(());
    };

    // get and check source directory
    let source;
    if let Some(result) = FileDialog::new()
        .set_title("Pick source directory")
        .pick_folder()
    {
        source = result;
        if !source.is_dir() || source.read_dir()?.next().is_none() {
            dbg!("source dir is empty");
            return Err(ImgcpError::Canceled.into());
        }
    } else {
        return Err(ImgcpError::Canceled.into());
    }

    // get and check target directory
    if let Some(target) = FileDialog::new()
        .set_title("Pick target directory")
        .pick_folder()
    {
        let error;
        match imgcp_lib::run(Some(source.as_path()), &target, false, false, false) {
            Err(ImgcpError::TargetDirNotEmpty { .. }) => {
                let result = MessageDialog::new()
                    .set_title("Confirmation")
                    .set_description(format!(
                        "Target dir {:?} is not empty, still continue?",
                        &target
                    ))
                    .set_level(MessageLevel::Info)
                    .set_buttons(MessageButtons::OkCancel)
                    .show();
                if result == MessageDialogResult::Cancel {
                    bail!("Operation aborted");
                } else {
                    error = imgcp_lib::run(Some(source.as_path()), &target, false, true, false);
                }
            }
            result => error = result,
        }

        if error.is_err() {
            let _ = MessageDialog::new()
                .set_title("Error Occurred")
                .set_description(format!("{:?}", &error))
                .set_level(MessageLevel::Error)
                .set_buttons(MessageButtons::Ok)
                .show();
        }
    }

    Ok(())
}
