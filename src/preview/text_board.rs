use std::collections::HashMap;

use crate::common::*;
use iced::{
    Background, Border, Color, Element, Length, Padding, Task, Theme,
    border::Radius,
    mouse,
    overlay::menu,
    widget::{column, container, mouse_area, pick_list, row, rule, space, text, text_editor},
};
use tracing::info;

#[derive(Debug, Default)]
pub struct TextBoard {
    all_text: HashMap<String, String>,
    current_text: text_editor::Content,
    current_text_id: Option<String>,
    id_counter: u64,
}

#[derive(Debug, Clone)]
pub enum TextBoardMessage {
    CreateNewText,
    LoadSelectedText,
    DeleteSelectedText,
    ChangeText(String),
    EditorAciton(text_editor::Action),
    HandError(AppError),
}

impl TextBoard {
    pub fn update(&mut self, message: TextBoardMessage) -> Task<TextBoardMessage> {
        match message {
            TextBoardMessage::CreateNewText => {
                self.id_counter += 1;
                let id = format!("文本 {}", self.id_counter);
                self.all_text.insert(id.clone(), String::default());
                self.current_text_id = Some(id);
                Task::done(TextBoardMessage::LoadSelectedText)
            }
            TextBoardMessage::EditorAciton(action) => {
                let is_edit = action.is_edit();
                self.current_text.perform(action);
                if is_edit {
                    if let Some(ref id) = self.current_text_id {
                        self.all_text.insert(id.clone(), self.current_text.text());
                    }
                }
                Task::none()
            }
            TextBoardMessage::ChangeText(id) => {
                self.current_text_id = Some(id);
                Task::done(TextBoardMessage::LoadSelectedText)
            }
            TextBoardMessage::LoadSelectedText => self
                .current_text_id
                .as_ref()
                .and_then(|id| self.all_text.get(id))
                .map(|text| {
                    self.current_text = text_editor::Content::with_text(text);
                    info!("[TextBoard-LoadSelectedText]:文本加载成功!");
                    Task::none()
                })
                .unwrap_or(Task::done(TextBoardMessage::HandError(
                    AppError::PreviewError("[TextBoard-LoadSelectedText]:文本加载失败!".to_string()),
                ))),
            TextBoardMessage::DeleteSelectedText => { self
                .current_text_id
                .as_ref()
                .and_then(|id| self.all_text.remove(id))
                .map(|_| {
                    info!("[TextBoard-DeleteSelectedText]:文本删除成功!");
                    if self.all_text.is_empty() {
                        Task::none()
                    } else {
                        let mut ids = self.all_text.keys().collect::<Vec<_>>();
                        ids.sort();
                        if let Some(id) = ids.first() {
                            self.current_text_id = Some(id.to_string());
                            Task::done(TextBoardMessage::LoadSelectedText)
                        } else {
                            Task::done(TextBoardMessage::HandError(
                                AppError::PreviewError("[TextBoard-DeleteSelectedText]:删除后新文本加载失败!".to_string()),
                            ))
                        }
                    }
                    
                }).unwrap_or(Task::done(TextBoardMessage::HandError(
                    AppError::PreviewError("[TextBoard-DeleteSelectedText]:文本删除失败!".to_string()),
                )))
            }
            TextBoardMessage::HandError(error) => {
                info!("{}", error);
                Task::none()
            }
        }
    }

    pub fn veiw(&self) -> Element<'_, TextBoardMessage> {
        let options: Element<'_, TextBoardMessage> = if !self.all_text.is_empty() {
            let mut options = self.all_text.keys().collect::<Vec<_>>();
            options.sort();
            pick_list(options, self.current_text_id.as_ref(), |key| {
                TextBoardMessage::ChangeText(key.to_string())
            })
            .text_size(FONT_SIZE_SMALLER)
            .text_line_height(1.)
            .style(|theme: &Theme, _| {
                let ex_palette = theme.extended_palette();
                let palette = theme.palette();
                pick_list::Style {
                    text_color: palette.text,
                    background: Background::Color(ex_palette.background.weaker.color),
                    border: Border::default(),
                    placeholder_color: palette.text,
                    handle_color: palette.text,
                }
            })
            .menu_style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                let palette = theme.palette();
                menu::Style {
                    background: Background::Color(ex_palette.background.weaker.color),
                    selected_background: Background::Color(ex_palette.background.base.color),
                    selected_text_color: palette.text,
                    text_color: palette.text,
                    border: Border::default(),
                    shadow: SHADOW_BASE,
                }
            })
            .into()
        } else {
            space().into()
        };
        let mut editor = text_editor(&self.current_text)
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
        if self.current_text_id.is_some() {
            editor = editor.on_action(TextBoardMessage::EditorAciton);
        }
        container(column![
            row![
                options,
                space::horizontal(),
                mouse_area(text("新建文本").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer)
                    .on_press(TextBoardMessage::CreateNewText),
                mouse_area(text("删除文本").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer)
                    .on_press(TextBoardMessage::DeleteSelectedText),
                mouse_area(text("复制全文").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer)
            ]
            .spacing(SPACING_BIGGER)
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
            editor
        ])
        .into()
    }
}
