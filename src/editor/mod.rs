use std::sync::Arc;

use crate::{
    common::*,
    editor::dialog::{TableDialog, TableDialogMessage},
};
use iced::{
    Background, Border, Color, Element, Length, Padding, Subscription, Task, Theme,
    border::Radius,
    mouse,
    widget::{
        Container, Row, center, column, container, mouse_area, opaque, row, rule, space, stack,
        text, text_editor,
    },
};
use tracing::{error, info};

mod dialog;
mod operation;

#[derive(Debug)]
pub struct Editor {
    selected_file: Option<FileData>,
    original_version: Option<u64>,
    editor_content: text_editor::Content,
    snap_shot: Vec<FileData>,
    // 各种模态窗口
    current_dialog: DialogPage,
    table_dialog: TableDialog, //history: VecDeque<text_editor::Action>,
                               //undo_stack: Vec<text_editor::Action>,                               //redo_stack: Vec<text_editor::Action>,
}

#[derive(Debug, Clone, Copy)]
pub enum DialogPage {
    None,
    TableDialog,
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    EditorAction(text_editor::Action),

    AddSnapShot(u32),

    SendNewContentToPreview(Arc<String>),

    SaveRequested,
    SaveAsRequested,
    AutoSaveCheck(u64),
    AutoSaveToFile(FileData),
    FileSaveAs(FileData),
    SaveToFile(FileData),
    HandleSaveResult(Result<(), AppError>),

    LoadFileDataFromFilePanel(FileData),

    // 各种模态窗口消息
    ChangeDialogPage(DialogPage),
    TableDialog(TableDialogMessage),
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            editor_content: text_editor::Content::default(),
            snap_shot: vec![],
            original_version: None,
            current_dialog: DialogPage::None,
            table_dialog: TableDialog::default(),
            //history: VecDeque::with_capacity(100),
            //undo_stack: vec![],
            //redo_stack: vec![],
        }
    }

    pub fn update(
        &mut self,
        editor_message: EditorMessage,
        setting: &AppSetting,
    ) -> Task<EditorMessage> {
        match editor_message {
            EditorMessage::EditorAction(action) => {
                //self.history.push_back(action.clone());
                let is_edit = action.is_edit();
                self.editor_content.perform(action);
                if is_edit {
                    let new_input = Arc::new(self.editor_content.text());
                    let task = if let Some(file_node) = &mut self.selected_file {
                        file_node.version += 1;
                        file_node.content = Arc::clone(&new_input);
                        Task::perform(
                            Editor::set_auto_save_delay_timer(file_node.version),
                            EditorMessage::AutoSaveCheck,
                        )
                    } else {
                        Task::none()
                    };
                    return Task::done(EditorMessage::SendNewContentToPreview(new_input))
                        .chain(task);
                }
                Task::none()
            }
            EditorMessage::LoadFileDataFromFilePanel(file_data) => {
                self.selected_file = Some(file_data.clone());
                self.editor_content = text_editor::Content::new();
                self.editor_content.perform(text_editor::Action::Edit(text_editor::Edit::Paste(Arc::clone(&file_data.content))));
                self.original_version = Some(file_data.version);
                info!("文件内容载入成功!");
                Task::done(EditorMessage::SendNewContentToPreview(Arc::clone(
                    &file_data.content,
                )))
            }
            EditorMessage::AutoSaveCheck(version) => {
                if let Some(ref selected_file) = self.selected_file {
                    if setting.auto_save && version == selected_file.version {
                        return Task::done(EditorMessage::AutoSaveToFile(selected_file.clone()));
                    }
                }
                Task::none()
            }
            EditorMessage::HandleSaveResult(operation_result) => match operation_result {
                Ok(_) => {
                    if let Some(ref selected_file) = self.selected_file {
                        self.original_version = Some(selected_file.version);
                    }
                    Task::none()
                }
                Err(ref error) => {
                    error!("{}", error);
                    Task::none()
                }
            },
            EditorMessage::SaveRequested => {
                if let Some(file_data) = &self.selected_file {
                    return Task::done(EditorMessage::SaveToFile(file_data.clone()));
                }
                Task::none()
            }
            EditorMessage::SaveAsRequested => {
                if let Some(file_data) = &self.selected_file {
                    return Task::done(EditorMessage::FileSaveAs(file_data.clone()));
                }
                Task::none()
            }
            // 处理各种子模块模态窗口消息
            EditorMessage::ChangeDialogPage(dialog) => {
                self.current_dialog = dialog;
                Task::none()
            }
            EditorMessage::TableDialog(table_dialog_message) => match table_dialog_message {
                TableDialogMessage::CloseDialog => {
                    self.current_dialog = DialogPage::None;
                    Task::none()
                }
                _ => self
                    .table_dialog
                    .update(table_dialog_message)
                    .map(EditorMessage::TableDialog),
            },
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, EditorMessage> {
        let (line_count, word_count) = if self.selected_file.is_some() {
            (
                self.editor_content.line_count(),
                self.editor_content.text().chars().count(),
            )
        } else {
            (0, 0)
        };
        let base: Element<'_, EditorMessage> = self.generate_editor_component().into();
        let editor_view = match self.current_dialog {
            DialogPage::None => base,
            DialogPage::TableDialog => Editor::set_modal(
                base.into(),
                self.table_dialog.view().map(EditorMessage::TableDialog),
            ),
            _ => base,
        };
        container(column![
            editor_view,
            container(
                row![
                    text!("行数  {}", line_count).size(FONT_SIZE_BASE),
                    text!("字符数  {}", word_count).size(FONT_SIZE_BASE),
                    space::horizontal(),
                ]
                .width(Length::Fill)
                .spacing(SPACING_BIGGER)
            )
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink)
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weaker.color)),
                    ..container::Style::default()
                }
            })
        ])
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(Background::Color(palette.background)),
                ..container::Style::default()
            }
        })
    }

    pub fn generate_editor_component(&self) -> Element<'_, EditorMessage> {
        let mut file_name_bar = row![
            mouse_area(text("图片").size(FONT_SIZE_BIGGER))
                .interaction(mouse::Interaction::Pointer),
            mouse_area(text("表格").size(FONT_SIZE_BIGGER))
                .interaction(mouse::Interaction::Pointer)
                .on_press(EditorMessage::ChangeDialogPage(DialogPage::TableDialog)),
            mouse_area(text("注释").size(FONT_SIZE_BIGGER))
                .interaction(mouse::Interaction::Pointer),
            mouse_area(text("链接").size(FONT_SIZE_BIGGER))
                .interaction(mouse::Interaction::Pointer),
            mouse_area(text("代码").size(FONT_SIZE_BIGGER))
                .interaction(mouse::Interaction::Pointer),
            space::horizontal()
        ]
        .width(Length::Fill)
        .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        .height(Length::Shrink);
        let mut file_content_editor = text_editor(&self.editor_content)
            .height(Length::Fill)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .highlight("markdown", iced::highlighter::Theme::Base16Ocean)
            .style(|theme: &Theme, _| {
                let palette = theme.palette();
                text_editor::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border::default(),
                    placeholder: palette.text,
                    value: palette.text,
                    selection: palette.primary,
                }
            });
        if let Some(file_data) = &self.selected_file {
            file_content_editor = file_content_editor.on_action(EditorMessage::EditorAction);
            if self.original_version == Some(file_data.version) {
                file_name_bar = file_name_bar.push(text!("已保存").size(FONT_SIZE_BIGGER).style(
                    |theme: &Theme| {
                        let palette = theme.palette();
                        text::Style {
                            color: Some(palette.success),
                        }
                    },
                ));
            } else if self.original_version.is_some()
                && self.original_version != Some(file_data.version)
            {
                file_name_bar = file_name_bar.push(text!("未保存").size(FONT_SIZE_BIGGER).style(
                    |theme: &Theme| {
                        let palette = theme.palette();
                        text::Style {
                            color: Some(palette.warning),
                        }
                    },
                ));
            }
        }

        column![
            file_name_bar.spacing(SPACING_BIGGER),
            rule::horizontal(1).style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                rule::Style {
                    color: ex_palette.background.weaker.color,
                    radius: Radius::default(),
                    snap: true,
                    fill_mode: rule::FillMode::Full,
                }
            }),
            file_content_editor
        ]
        .into()
    }

    pub async fn set_auto_save_delay_timer(version: u64) -> u64 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        version
    }

    pub fn set_modal<'a>(
        base: Element<'a, EditorMessage>,
        content: Element<'a, EditorMessage>,
    ) -> Element<'a, EditorMessage> {
        stack![
            base,
            opaque(center(content).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.2,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
        ]
        .into()
    }
}
