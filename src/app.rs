use crate::{
    editor::{Editor, EditorMessage},
    file_panel::{FilePanel, FilePanelMessage},
    menu_bar::{MenuBar, MenuBarMessage},
    preview::{self, Preview, PreviewMessage},
    status_bar::{StatusBar, StatusBarMessage},
};
use iced::{
    widget::{canvas::path::lyon_path::geom::euclid::approxeq::ApproxEq, column, row}, Element, Length, Task
};
use tracing::{error, info, warn};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone)]
pub struct App {
    editor: Editor,
    preview: Preview,
    file_panel: FilePanel,
    menu_bar: MenuBar,
    status_bar: StatusBar,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    //模块消息
    Editor(EditorMessage),
    Preview(PreviewMessage),
    FilePanel(FilePanelMessage),
    MenuBar(MenuBarMessage),
    StatusBar(StatusBarMessage),
    // 顶层消息
    None,
}

impl App {
    pub fn new() -> Self {
        // 初始化日志
        // let file_appender = rolling::daily("logs", "app.log");
        // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // tracing_subscriber::fmt()
        //     .with_env_filter(EnvFilter::new("fugu=info"))
        //     .with_ansi(false)
        //     .with_writer(non_blocking)
        //     .init();
        // 

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("fugu=info"))
            .init();
        info!("应用启动！");
        Self {
            editor: Editor::new(),
            preview: Preview::new(),
            file_panel: FilePanel::new(),
            menu_bar: MenuBar::new(),
            status_bar: StatusBar {},
        }
    }

    pub fn update(&mut self, app_message: AppMessage) -> Task<AppMessage> {
        match app_message {
            AppMessage::FilePanel(file_panel_message) => match file_panel_message {
                FilePanelMessage::SendFileDataToEditor(file_data) => {
                    self.editor.update(EditorMessage::GetFileDataFromFilePanel(file_data)).map(AppMessage::Editor)
                }
                _ => self
                    .file_panel
                    .update(file_panel_message)
                    .map(AppMessage::FilePanel)
            },
            AppMessage::MenuBar(menu_bar_message) => match menu_bar_message {
                MenuBarMessage::CommandOpenFolder => self
                    .file_panel
                    .update(FilePanelMessage::OperationOpenFolder)
                    .map(AppMessage::FilePanel),
                MenuBarMessage::CommandOpenFile => self
                    .file_panel
                    .update(FilePanelMessage::OperationOpenFile)
                    .map(AppMessage::FilePanel),
                _ => self
                    .menu_bar
                    .update(menu_bar_message)
                    .map(AppMessage::MenuBar),
            },
            AppMessage::Editor(editor_message) => {
                match editor_message {
                    EditorMessage::SendInputContentToPreview(input_content) => {
                        self.preview.update(PreviewMessage::GetInputContentFromEditor(input_content))
                            .map(AppMessage::Preview)
                    }
                    _ => self.editor.update(editor_message).map(AppMessage::Editor)
                }
            }
            AppMessage::Preview(preview_message) => {
                match preview_message {
                    _ => self.preview.update(preview_message).map(AppMessage::Preview)
                }
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let menu_bar: Element<'_, MenuBarMessage> = self
            .menu_bar
            .view()
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();
        let file_panel: Element<'_, FilePanelMessage> = self
            .file_panel
            .view()
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .into();
        let editor: Element<'_, EditorMessage> = self
            .editor
            .view()
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .into();
        let preview: Element<'_, PreviewMessage> = self
            .preview
            .view()
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .into();
        let status_bar: Element<'_, StatusBarMessage> = self
            .status_bar
            .view()
            .width(Length::Fill)
            .height(Length::Shrink)
            .into();

        column![
            menu_bar.map(AppMessage::MenuBar),
            row![
                file_panel.map(AppMessage::FilePanel),
                editor.map(AppMessage::Editor),
                preview.map(AppMessage::Preview),
            ]
            .height(Length::Fill)
            .width(Length::Fill),
            status_bar.map(AppMessage::StatusBar)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}


