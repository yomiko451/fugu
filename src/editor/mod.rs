use iced::{
    alignment::Horizontal, border::Radius, theme::palette, widget::{column, container, markdown::rule, row, rule, text, text_editor, Container}, Background, Border, Color, Length, Padding, Task, Theme
};
use tracing::info;

use crate::common::*;

#[derive(Debug, Clone)]
pub struct Editor {
    selected_file: Option<FileData>,
    editor_content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    EditorAction(text_editor::Action),
    GetFileDataFromFilePanel(FileData),
    // 模块间通信
    SendInputContentToPreview(String),
}

impl Editor {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            editor_content: text_editor::Content::default(),
        }
    }

    pub fn update(&mut self, editor_message: EditorMessage) -> Task<EditorMessage> {
        match editor_message {
            EditorMessage::EditorAction(action) => {
                let is_edit = action.is_edit();
                self.editor_content.perform(action);
                if is_edit {
                    if let Some(file_data) = &mut self.selected_file {
                        file_data.is_saved = false;
                    }
                    let input = self.editor_content.text();
                    return Task::done(EditorMessage::SendInputContentToPreview(input));
                }
                Task::none()
            }
            EditorMessage::GetFileDataFromFilePanel(file_data) => {
                self.selected_file = Some(file_data.clone());
                self.editor_content = text_editor::Content::with_text(&file_data.content);
                info!("文件内容载入成功!");
                Task::done(EditorMessage::SendInputContentToPreview(file_data.content))
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, EditorMessage> {
        let name = if let Some(file_data) = &self.selected_file {
            if !file_data.is_saved {
                format!("{} *", file_data.name)
            } else {
                file_data.name.clone()
            }
        } else {
            "".to_string()
        };

        container(column![
            row![
                text(name)
                    .width(Length::Fill)
                    
                    .size(FONT_SIZE_BIGGER)
            ]
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink),
            rule::horizontal(1).style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                rule::Style {
                    color: ex_palette.background.weaker.color,
                    radius: Radius::default(),
                    snap: true,
                    fill_mode: rule::FillMode::Full
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
        ])
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(Background::Color(palette.background)),
                ..container::Style::default()
            }
        })
    }
}
