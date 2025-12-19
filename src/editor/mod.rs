use std::sync::Arc;

use crate::common::*;
use iced::{
    Background, Border, Color, Element, Length, Padding, Subscription, Task, Theme,
    border::Radius,
    widget::{Container, Row, column, container, row, rule, space, text, text_editor},
};
use tracing::{error, info};

mod operation;
mod snapshot;

#[derive(Debug, Clone)]
pub struct Editor {
    selected_file: Option<FileData>,
    original_version: Option<u64>,
    editor_content: text_editor::Content,
    snap_shot: Vec<FileData>, //history: VecDeque<text_editor::Action>,
                              //undo_stack: Vec<text_editor::Action>,                               //redo_stack: Vec<text_editor::Action>,
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
    HandleSaveResult(OperationResult),

    LoadFileDataFromFilePanel(FileData),
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            editor_content: text_editor::Content::default(),
            snap_shot: vec![],
            original_version: None,
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
                        Task::perform(Editor::set_auto_save_delay_timer(file_node.version), EditorMessage::AutoSaveCheck)
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
                self.editor_content = text_editor::Content::with_text(&file_data.content);
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
                OperationResult::Ok(ref info) => {
                    info!(info);
                    if let Some(ref selected_file) = self.selected_file {
                        self.original_version = Some(selected_file.version);
                    }
                    Task::none()
                }
                OperationResult::Err(ref error) => {
                    error!(error);
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
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, EditorMessage> {
        container(column![
            self.generate_editor_component(),
            container(
                row![
                    text!("行数  {}", self.editor_content.line_count()).size(FONT_SIZE_BASE),
                    text!("字符数  {}", self.editor_content.text().chars().count())
                        .size(FONT_SIZE_BASE),
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
        let mut file_name_bar = Row::new()
            .width(Length::Fill)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink);
        let mut file_content_editor = text_editor(&self.editor_content)
            .height(Length::Fill)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
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
            if self.original_version != Some(file_data.version) {
                file_name_bar = file_name_bar.push(text!(" *").size(FONT_SIZE_BIGGER));
            }
            file_name_bar = file_name_bar.push(text(file_data.version).size(FONT_SIZE_BIGGER));
        } else {
            file_name_bar = file_name_bar.push(text("").size(FONT_SIZE_BIGGER));
        }

        column![
            file_name_bar,
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
    
}

