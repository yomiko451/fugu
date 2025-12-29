use std::sync::Arc;

use crate::{
    common::{AppSetting, DEFAULT_SETTING},
    editor::{Editor, EditorMessage},
    file_panel::{FilePanel, FilePanelMessage},
    menu_bar::{MenuBar, MenuBarMessage},
    preview::{Preview, PreviewMessage},
};
use iced::{
    Element, Length, Subscription, Task,
    widget::{column, row},
};
use tracing::{error, info, warn};
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
pub struct App {
    editor: Editor,
    preview: Preview,
    file_panel: FilePanel,
    menu_bar: MenuBar,
    setting: AppSetting,
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
        let app = Self {
            editor: Editor::new(),
            preview: Preview::new(),
            file_panel: FilePanel::new(),
            menu_bar: MenuBar::new(),
            setting: DEFAULT_SETTING,
        };
        let task = Task::batch([Task::none()]);
        (app, task)
    }

    pub fn update(&mut self, app_message: AppMessage) -> Task<AppMessage> {
        match app_message {
            AppMessage::FilePanel(file_panel_message) => match file_panel_message {
                FilePanelMessage::SendFileDataToEditor(file_data) => Task::done(
                    AppMessage::Editor(EditorMessage::LoadFileDataFromFilePanel(file_data)),
                ),
                FilePanelMessage::ReturnSaveResult(operation_result) => Task::done(
                    AppMessage::Editor(EditorMessage::HandleSaveResult(operation_result)),
                ),
                FilePanelMessage::SendImgDataToPreview(image_data) => Task::done(
                    AppMessage::Preview(PreviewMessage::GetImgPathFromFilePanel(image_data)),
                ),
                FilePanelMessage::SendImgCodeToEditor(code) => {
                    Task::done(AppMessage::Editor(EditorMessage::GetImgCodeFromFilePanel(code)))
                }
                FilePanelMessage::SendImgBasePathToPreview(path) => {
                    Task::done(AppMessage::Preview(PreviewMessage::GetImgBasePathFromFilePanel(path)))
                }
                _ => self
                    .file_panel
                    .update(file_panel_message, &self.setting)
                    .map(AppMessage::FilePanel),
            },
            AppMessage::MenuBar(menu_bar_message) => match menu_bar_message {
                MenuBarMessage::CommandOpenFolder => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::OpenMdFolder))
                }
                MenuBarMessage::CommandOpenFile => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::OpenFile))
                }
                MenuBarMessage::CommandImportImg => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::ImportImg))
                }
                MenuBarMessage::CommandImportImgFolder => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::ImportImgFolder))
                }
                MenuBarMessage::CommandCreateNewFile => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::CreateNewFile))
                }
                MenuBarMessage::CommandSaveFile => {
                    Task::done(AppMessage::Editor(EditorMessage::SaveRequested))
                }
                MenuBarMessage::CommandSaveAs => {
                    Task::done(AppMessage::Editor(EditorMessage::SaveAsRequested))
                }
                MenuBarMessage::SettingAutoSave(auto_save) => {
                    self.setting.auto_save = auto_save;
                    Task::none()
                }
                _ => self
                    .menu_bar
                    .update(menu_bar_message)
                    .map(AppMessage::MenuBar),
            },
            AppMessage::Editor(editor_message) => match editor_message {
                EditorMessage::SendNewContentToPreview(new_content) => Task::done(
                    AppMessage::Preview(PreviewMessage::SyncContnetWithEditor(new_content)),
                ),
                EditorMessage::AutoSaveToFile(file_data) => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::AutoSave(file_data)))
                }
                EditorMessage::FileSaveAs(file_data) => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::SaveAs(file_data)))
                }
                EditorMessage::SaveToFile(file_data) => {
                    Task::done(AppMessage::FilePanel(FilePanelMessage::Save(file_data)))
                }
                _ => self
                    .editor
                    .update(editor_message, &self.setting)
                    .map(AppMessage::Editor),
            },
            AppMessage::Preview(preview_message) => match preview_message {
                PreviewMessage::SendImgIdToFilePanel(id) => Task::done(AppMessage::FilePanel(
                    FilePanelMessage::GetImgIdFromPreview(id),
                )),
                _ => self
                    .preview
                    .update(preview_message, &self.setting)
                    .map(AppMessage::Preview),
            },
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let menu_bar: Element<'_, MenuBarMessage> = self
            .menu_bar
            .view(&self.setting)
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
        Subscription::batch([self.preview.subscription().map(AppMessage::Preview)])
    }
}
