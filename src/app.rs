use iced::{
    widget::{column, row, rule}, Element, Length, Task
};

use crate::{
    editor::{Editor, EditorMessage},
    file_panel::{FilePanel, FilePanelMessage},
    menu_bar::{MenuBar, MenuBarMessage},
    preview::{Preview, PreviewMessage},
    status_bar::{StatusBar, StatusBarMessage},
};

#[derive(Debug, Clone)]
pub struct App {
    editor: Editor,
    preview: Preview,
    file_panel: FilePanel,
    menu_bar: MenuBar,
    status_bar: StatusBar
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
        Self {
            editor: Editor {},
            preview: Preview {},
            file_panel: FilePanel::new(),
            menu_bar: MenuBar {},
            status_bar: StatusBar {},
        }
    }

    pub fn update(&mut self, app_message: AppMessage) -> Task<AppMessage> {
        match app_message {
            AppMessage::FilePanel(file_panel_message) => {
                self.file_panel.update(file_panel_message).map(AppMessage::FilePanel)
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        let menu_bar: Element<'_, MenuBarMessage> = self.menu_bar.view()
            .width(Length::Fill).height(Length::Shrink).into();
        let file_panel: Element<'_, FilePanelMessage> = self.file_panel.view()
            .width(Length::FillPortion(2)).height(Length::Fill).into();
        let editor: Element<'_, EditorMessage> = self.editor.view()
            .width(Length::FillPortion(5)).height(Length::Fill).into();
        let preview: Element<'_, PreviewMessage> = self.preview.view()
            .width(Length::FillPortion(5)).height(Length::Fill).into();
        let status_bar: Element<'_, StatusBarMessage> = self.status_bar.view()
            .width(Length::Fill).height(Length::Shrink).into();

        column![
            menu_bar.map(|_| AppMessage::None),
            row![
                file_panel.map(AppMessage::FilePanel),
                editor.map(|_| AppMessage::None),
                preview.map(|_| AppMessage::None),
            ].height(Length::Fill)
            .width(Length::Fill),
            status_bar.map(|_| AppMessage::None)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
