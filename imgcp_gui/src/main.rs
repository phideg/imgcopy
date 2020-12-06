use anyhow::{bail, Result};
use druid::widget::{Align, Button, Flex, Label};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, FileDialogOptions,
    FileSpec, Handled, LocalizedString, Target, Widget, WindowDesc,
};
use imgcopy;
use imgcopy::ImgcpError;
use std::path::PathBuf;

#[derive(Clone, Data)]
struct DialogState {
    src: String,
    trg: String,
}

struct Delegate;

pub fn main() {
    let main_window = WindowDesc::new(ui_builder).title(
        LocalizedString::new("imgcopy-title")
            .with_placeholder("ImgCopy - Bilder organisieren leicht gemacht"),
    );

    let initial_state = DialogState {
        src: ".".to_string(),
        trg: ".".to_string(),
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .use_simple_logger()
        .launch(initial_state)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<DialogState> {
    let src_dialog_options = FileDialogOptions::new().select_directories();

    let src_label = Label::new(|data: &DialogState, _env: &_| format!("{}", data.src));

    let open =
        Button::new(LocalizedString::new("button-src-folder").with_placeholder("Quellordner"))
            .on_click(move |ctx, _, _| {
                ctx.submit_command(Command::new(
                    druid::commands::SHOW_OPEN_PANEL,
                    src_dialog_options.clone(),
                    Target::Auto,
                ))
            });

    let mut row = Flex::row();
    row.add_child(src_label);
    row.add_spacer(8.0);
    row.add_child(open);
    Align::centered(row)
}

impl AppDelegate<DialogState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut DialogState,
        _env: &Env,
    ) -> Handled {
        if let Some(Some(file_info)) = cmd.get(commands::SAVE_FILE) {
            if let Err(e) = std::fs::write(file_info.path(), &data[..]) {
                println!("Error writing file: {}", e);
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    let first_line = s.lines().next().unwrap_or("");
                    *data = first_line.to_owned();
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }
}

// use anyhow::{bail, Result};
// use native_dialog::*;
// use imgcopy;
// use imgcopy::ImgcpError;

// fn main() -> Result<()> {
//     // introduce next steps of the program
//     let dialog = MessageAlert {
//         title: "How To",
//         text: "After clicking OK you will need to choose first the source directory and afterwards the target directory in which the images should be copied.",
//         typ: MessageType::Info,
//     };
//     if dialog.show().is_err() {
//         return Ok(());
//     };

//     // get and check source directory
//     let dialog = OpenSingleDir { dir: None };
//     let source;
//     let result = dialog.show()?;
//     if let Some(result) = result {
//         source = result;
//     } else {
//         return Err(imgcopy::ImgcpError::Canceled)?;
//     }
//     // If target dir is not empty ask for confirmation to continue
//     if !source.is_dir() || source.read_dir()?.next().is_none() {
//         dbg!("dir empty");
//         return Err(imgcopy::ImgcpError::Canceled)?;
//     }

//     // get and check target directory
//     let target;
//     let dialog = OpenSingleDir { dir: None };
//     let result = dialog.show()?;
//     if let Some(result) = result {
//         target = result;
//     } else {
//         return Err(imgcopy::ImgcpError::Canceled)?;
//     }

//     let error;
//     match imgcopy::run(Some(source.as_path()), &target, false, false) {
//         Err(ImgcpError::TargetDirNotEmpty { .. }) => {
//             let dialog = MessageConfirm {
//                 title: "Confirmation",
//                 text: &format!("Target dir {:?} is not empty, still continue?", &target),
//                 typ: MessageType::Info,
//             };
//             let result = dialog.show();
//             if result.is_ok() && result.unwrap() == true {
//                 error = imgcopy::run(Some(source.as_path()), &target, false, true);
//             } else {
//                 bail!("Operation aborted");
//             }
//         }
//         result => error = result,
//     }

//     if error.is_err() {
//         let dialog = MessageAlert {
//             title: "Error Occurred",
//             text: &format!("{:?}", &error),
//             typ: MessageType::Info,
//         };
//         let _ = dialog.show();
//     }

//     Ok(())
// }
