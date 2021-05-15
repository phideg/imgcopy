use druid::widget::{Button, CrossAxisAlignment, Flex, TextBox};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, FileDialogOptions,
    Handled, Lens, LocalizedString, Target, UnitPoint, Widget, WidgetExt, WindowDesc,
};
//use std::path::PathBuf;
// use anyhow::{bail, Result};
// use imgcopy;
// use imgcopy::ImgcpError;

struct Delegate;

#[derive(Clone, Lens, Data, Debug)]
struct PathData {
    src_path: String,
    trg_path: String,
}

pub fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .window_size((500., 200.))
        .with_min_size((500., 200.))
        .title(LocalizedString::new("title").with_placeholder("Bilder-Aufräumer"));
    let data = PathData {
        src_path: String::new(),
        trg_path: String::new(),
    };
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<PathData> {
    let src_dialog_options = FileDialogOptions::new()
        .select_directories()
        .name_label("Quelle")
        .title("Woher kommen die Bilder?")
        .button_text("Quellordner");

    let trg_dialog_options = src_dialog_options
        .clone()
        // .name_label("Ziel")
        .title("Wohin sollen die Bilder?")
        .button_text("Zielordner");

    let input_row = Flex::row()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Button::new("QuellOrdner auswählen")
                        .on_click(move |ctx, _, _| {
                            ctx.submit_command(Command::new(
                                druid::commands::SHOW_OPEN_PANEL,
                                src_dialog_options.clone(),
                                Target::Auto,
                            ))
                        })
                        .padding(5.0),
                )
                .with_child(
                    Button::new("Zielordner auswählen")
                        .on_click(move |ctx, _, _| {
                            ctx.submit_command(Command::new(
                                druid::commands::SHOW_OPEN_PANEL,
                                trg_dialog_options.clone(),
                                Target::Auto,
                            ))
                        })
                        .padding(5.0),
                )
                .align_left()
                .padding(10.0),
        )
        .with_flex_child(
            Flex::column()
                .with_child(
                    TextBox::new()
                        .lens(PathData::src_path)
                        .expand_width()
                        .padding(5.0),
                )
                .with_child(
                    TextBox::new()
                        .lens(PathData::trg_path)
                        .expand_width()
                        .padding(5.0),
                )
                .expand_width()
                .padding(1.0),
            1.0,
        )
        .padding(10.0)
        .align_left();
    Flex::column().with_child(input_row).with_child(
        Flex::row()
            .with_child(Button::new("Start"))
            .with_spacer(20.0)
            .align_horizontal(UnitPoint::RIGHT),
    )
    // .debug_paint_layout()
}

impl AppDelegate<PathData> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut PathData,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            dbg!(&file_info);
            data.trg_path = file_info.path().to_str().unwrap_or("").to_string();
            return Handled::Yes;
        }
        Handled::No
    }
}
