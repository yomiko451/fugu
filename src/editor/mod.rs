use std::sync::Arc;

use crate::common::*;
use iced::{
    alignment::Vertical, border::Radius, mouse, widget::{
        column, container, mouse_area, row, rule, space, stack, text, text_editor, text_input, Container, Row
    }, Background, Border, Color, Element, Length, Padding, Subscription, Task, Theme
};
use iced_aw::style::colors::PALE_GREEN;
use tracing::info;

mod operation;
mod snapshot;

#[derive(Debug, Clone)]
pub struct Editor {
    selected_file: Option<FileData>,
    is_content_changed: bool,
    last_edited_at: Option<iced::time::Instant>,
    editor_content: text_editor::Content,
    input_content: String,
    input_flag: bool,
    auto_save: bool,
    snap_shot: Vec<FileData>, //history: VecDeque<text_editor::Action>,
                              //undo_stack: Vec<text_editor::Action>,                               //redo_stack: Vec<text_editor::Action>,
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    EditorAction(text_editor::Action),
    InputAction(String),
    DebounceTick(iced::time::Instant),
    AddSnapShot(u32),
    ShowRenameInput,

    SendNewContentToPreview(Arc<String>),
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
            input_content: "".to_string(),
            snap_shot: vec![],
            is_content_changed: false,
            input_flag: false,
            auto_save: true,
            //history: VecDeque::with_capacity(100),
            //undo_stack: vec![],
            //redo_stack: vec![],
        }
    }

    pub fn update(&mut self, editor_message: EditorMessage) -> Task<EditorMessage> {
        match editor_message {
            EditorMessage::ShowRenameInput => {
                self.input_flag = !self.input_flag;
                Task::none()
            }
            EditorMessage::EditorAction(action) => {
                //self.history.push_back(action.clone());
                let is_edit = action.is_edit();
                self.editor_content.perform(action);
                if is_edit {
                    self.is_content_changed = true;
                    let new_input = Arc::new(self.editor_content.text());
                    self.last_edited_at = Some(iced::time::Instant::now());
                    if let Some(file_node) = &mut self.selected_file {
                        file_node.content = Arc::clone(&new_input);
                    }
                    return Task::done(EditorMessage::SendNewContentToPreview(new_input));
                }
                Task::none()
            }
            EditorMessage::LoadFileDataFromFilePanel(file_data) => {
                self.selected_file = Some(file_data.clone());
                self.editor_content = text_editor::Content::with_text(&file_data.content);
                info!("文件内容载入成功!");
                Task::done(EditorMessage::SendNewContentToPreview(Arc::clone(
                    &file_data.content,
                )))
            }
            EditorMessage::DebounceTick(instant) => {
                if let Some(last_edited_at) = self.last_edited_at {
                    if instant.duration_since(last_edited_at) > iced::time::Duration::from_secs(2) {
                        if let Some(ref file_data) = self.selected_file {
                            return Task::done(EditorMessage::SendSaveRequestToSaveFileData(
                                file_data.clone(),
                            ));
                        }
                    }
                }
                Task::none()
            }
            EditorMessage::GetSaveSuccessFromFilePanel => {
                info!("文件保存成功!");
                self.last_edited_at = None;
                self.is_content_changed = false;
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

    pub fn subscription(&self) -> Subscription<EditorMessage> {
        iced::time::every(iced::time::Duration::from_secs(1))
            .map(|_| EditorMessage::DebounceTick(iced::time::Instant::now()))
    }

    pub fn generate_editor_component(&self) -> Element<'_, EditorMessage> {
        let mut file_name_bar = Row::new()
        .width(Length::Fill)
        .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        .height(Length::Shrink);
        if let Some(file_data) = &self.selected_file {
            if self.auto_save {
                file_name_bar = file_name_bar.push(text("自动保存 ")
                    .size(FONT_SIZE_BIGGER)
                    .style(|theme: &Theme| {
                        let palette = theme.palette();
                        text::Style {
                            color: Some(palette.success)
                        }
                    }))
            }
            file_name_bar = file_name_bar.push(text(&file_data.name).size(FONT_SIZE_BIGGER));
        }
        if self.is_content_changed {
            file_name_bar = file_name_bar.push(text!(" *").size(FONT_SIZE_BIGGER));
        } 
        
        let file_name_input: Element<EditorMessage> = text_input("", &self.input_content)
            .on_input(EditorMessage::InputAction)
            .on_submit(EditorMessage::ShowRenameInput)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .size(FONT_SIZE_BIGGER)
            .style(|theme: &Theme, _| {
                let palette = theme.palette();
                text_input::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    placeholder: palette.text,
                    value: palette.text,
                    border: Border::default(),
                    selection: palette.background,
                    icon: palette.text,
                }
            })
            .into();
        column![
            if !self.input_flag {
                mouse_area(
                    file_name_bar
                )
                .interaction(mouse::Interaction::Pointer)
                .on_press(EditorMessage::ShowRenameInput)
                .into()
            } else {
                file_name_input
            },
            mouse_area(rule::horizontal(1).style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                rule::Style {
                    color: ex_palette.background.weaker.color,
                    radius: Radius::default(),
                    snap: true,
                    fill_mode: rule::FillMode::Full,
                }
            })).interaction(mouse::Interaction::Text),
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
