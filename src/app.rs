use std::sync::Arc;

use crate::{
    common::{AppSetting, DEFAULT_SETTING}, editor::{Editor, EditorMessage}, file_panel::{FilePanel, FilePanelMessage}, menu_bar::{MenuBar, MenuBarMessage}, preview::{Preview, PreviewMessage}
};
use iced::{
    Element, Length, Subscription, Task,
    widget::{column, row},
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
    setting: AppSetting
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    //模块消息
    Editor(EditorMessage),
    Preview(PreviewMessage),
    FilePanel(FilePanelMessage),
    MenuBar(MenuBarMessage),
    // 顶层消息
    None,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
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
        let app = Self {
            editor: Editor::new(),
            preview: Preview::new(),
            file_panel: FilePanel::new(),
            menu_bar: MenuBar::new(),
            setting: DEFAULT_SETTING,
        };
        let task = Task::batch([
            Task::none()
        ]);
        (app, task)
    }

    pub fn update(&mut self, app_message: AppMessage) -> Task<AppMessage> {
        match app_message {
            AppMessage::FilePanel(file_panel_message) => match file_panel_message {
                FilePanelMessage::SendSelectedFileDataToEditor(file_data) => self
                    .editor
                    .update(EditorMessage::LoadFileDataFromFilePanel(file_data))
                    .map(AppMessage::Editor),
                FilePanelMessage::SendSaveSuccessToEditor => self
                    .editor
                    .update(EditorMessage::GetSaveSuccessFromFilePanel)
                    .map(AppMessage::Editor),
                _ => self
                    .file_panel
                    .update(file_panel_message, &self.setting)
                    .map(AppMessage::FilePanel),
            },
            AppMessage::MenuBar(menu_bar_message) => match menu_bar_message {
                MenuBarMessage::CommandOpenFolder => self
                    .file_panel
                    .update(FilePanelMessage::OperationOpenFolder, &self.setting)
                    .map(AppMessage::FilePanel),
                MenuBarMessage::CommandOpenFile => self
                    .file_panel
                    .update(FilePanelMessage::OperationOpenFile, &self.setting)
                    .map(AppMessage::FilePanel),
                MenuBarMessage::CommandCreateNewFile => self
                    .file_panel
                    .update(FilePanelMessage::OperationCreateNewFile, &self.setting)
                    .map(AppMessage::FilePanel),
                MenuBarMessage::CommandSaveFile=> self
                    .editor
                    .update(EditorMessage::ManualSave)
                    .map(AppMessage::Editor),
                _ => self
                    .menu_bar
                    .update(menu_bar_message)
                    .map(AppMessage::MenuBar),
            },
            AppMessage::Editor(editor_message) => match editor_message {
                EditorMessage::SendNewContentToPreview(new_content) => self
                    .preview
                    .update(PreviewMessage::SyncContnetWithEditor(new_content))
                    .map(AppMessage::Preview),
                EditorMessage::SendSaveRequestToFileData(file_data) => self
                    .file_panel
                    .update(FilePanelMessage::SaveFileDataFromEditor(file_data), &self.setting)
                    .map(AppMessage::FilePanel),
                _ => self.editor.update(editor_message).map(AppMessage::Editor),
            },
            AppMessage::Preview(preview_message) => match preview_message {
                _ => self
                    .preview
                    .update(preview_message)
                    .map(AppMessage::Preview),
            },
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

        row![
            column![
                menu_bar.map(AppMessage::MenuBar),
                row![
                    file_panel.map(AppMessage::FilePanel),
                    editor.map(AppMessage::Editor),
                ]
                .height(Length::Fill)
                .width(Length::Fill)
            ]
            .height(Length::Fill)
            .width(Length::FillPortion(7)),
            preview.map(AppMessage::Preview),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        Subscription::batch([self.editor.subscription().map(AppMessage::Editor)])
    }
}
