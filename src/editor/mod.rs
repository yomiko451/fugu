use crate::common::*;
use iced::{
    alignment::Horizontal, border::Radius, mouse, theme::palette, widget::{
        column, container, markdown::rule, mouse_area, row, rule, space, text, text_editor, text_input, Container, TextEditor
    }, Background, Border, Color, Element, Length, Padding, Subscription, Task, Theme
};
use std::{collections::VecDeque, string};
use tracing::info;

mod clipboard;
mod operation;
mod snapshot;

#[derive(Debug, Clone)]
pub struct Editor {
    selected_file: Option<FileData>,
    last_edited_at: Option<iced::time::Instant>,
    editor_content: text_editor::Content,
    snap_shot: Vec<SnapShot>
    //history: VecDeque<text_editor::Action>,
                                         //undo_stack: Vec<text_editor::Action>,                               //redo_stack: Vec<text_editor::Action>,
}

#[derive(Debug, Clone)]
struct SnapShot {
    created_at: String,
    name: String,
    content: String
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    EditorAction(text_editor::Action),
    DebounceTick(iced::time::Instant),
    TimeToSaveFileData,
    AddSnapShot(String),

    SendInputContentToPreview(String),
    SendSaveRequestToSaveFileData(FileData),
    LoadFileDataFromFilePanel(FileData),
    GetSaveSuccessFromFilePanel,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            //auto_save: bool, // 开了会不会出现手动和自动保存竞态?
            last_edited_at: None,
            editor_content: text_editor::Content::default(),
            snap_shot: vec![]
             //history: VecDeque::with_capacity(100),
                              //undo_stack: vec![],
                              //redo_stack: vec![],
        }
    }

    pub fn update(&mut self, editor_message: EditorMessage) -> Task<EditorMessage> {
        match editor_message {
            EditorMessage::EditorAction(action) => {
                //self.history.push_back(action.clone());
                let is_edit = action.is_edit();
                self.editor_content.perform(action);
                if is_edit {
                    if let Some(file_data) = &mut self.selected_file {
                        file_data.is_saved = false;
                    }
                    let input = self.editor_content.text();
                    self.last_edited_at = Some(iced::time::Instant::now());
                    return Task::done(EditorMessage::SendInputContentToPreview(input));
                }
                Task::none()
            }
            EditorMessage::LoadFileDataFromFilePanel(file_data) => {
                self.selected_file = Some(file_data.clone());
                self.editor_content = text_editor::Content::with_text(&file_data.content);
                info!("文件内容载入成功!");
                Task::done(EditorMessage::SendInputContentToPreview(file_data.content))
            }
            EditorMessage::DebounceTick(instant) => {
                if let Some(last_edited_at) = self.last_edited_at {
                    if instant.duration_since(last_edited_at) > iced::time::Duration::from_secs(2) {
                        return Task::done(EditorMessage::TimeToSaveFileData);
                    }
                }
                Task::none()
            }
            EditorMessage::TimeToSaveFileData => {
                if let Some(file_data) = &mut self.selected_file {
                    file_data.content = self.editor_content.text();
                    return Task::done(EditorMessage::SendSaveRequestToSaveFileData(
                        file_data.clone(),
                    ));
                }
                Task::none()
            }
            EditorMessage::GetSaveSuccessFromFilePanel => {
                if let Some(file_data) = &mut self.selected_file {
                    info!("文件保存成功!");
                    self.last_edited_at = None;
                    file_data.is_saved = true;
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
                    text("恢复").size(FONT_SIZE_BASE),
                    space::horizontal(),
                    text("恢复").size(FONT_SIZE_BASE)
                ]
                .width(Length::Fill)
                .spacing(SPACING)
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

    pub fn subscription(&self) -> Subscription<EditorMessage> {
        iced::time::every(iced::time::Duration::from_secs(1))
            .map(|_| EditorMessage::DebounceTick(iced::time::Instant::now()))
    }

    pub fn generate_editor_component(&self) -> Element<EditorMessage> {
        let name = if let Some(file_data) = &self.selected_file {
            if !file_data.is_saved {
                format!("{} *", file_data.name)
            } else {
                file_data.name.clone()
            }
        } else {
            "".to_string()
        };
        column![
            row![text(name).width(Length::Fill).size(FONT_SIZE_BIGGER)]
                .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
                .height(Length::Shrink),
            rule::horizontal(1).style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                rule::Style {
                    color: ex_palette.background.weaker.color,
                    radius: Radius::default(),
                    snap: true,
                    fill_mode: rule::FillMode::Full,
                }
            }),
            text_editor(&self.editor_content)
                .on_action(EditorMessage::EditorAction)
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
                }),
        ]
        .into()
    }
}
