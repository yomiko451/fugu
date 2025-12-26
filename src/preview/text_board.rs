use std::collections::HashMap;

use crate::common::*;
use iced::{
    Background, Border, Color, Element, Length, Padding, Task, Theme,
    border::Radius,
    mouse,
    overlay::menu,
    widget::{column, container, mouse_area, pick_list, row, rule, space, text, text_editor},
};

#[derive(Debug, Default)]
pub struct TextBoard {
    all_text: HashMap<String, String>,
    current_text: text_editor::Content,
    current_text_index: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TextBoardMessage {
    LoadText(String),
    ChangeText(String),
    EditorAciton(text_editor::Action),
}

impl TextBoard {
    pub fn update(&mut self, message: TextBoardMessage) -> Task<TextBoardMessage> {
        match message {
            _ => Task::none(),
        }
    }

    pub fn veiw(&self) -> Element<'_, TextBoardMessage> {
        let options: Element<'_, TextBoardMessage> = if !self.all_text.is_empty() {
            let mut options = self.all_text.keys().collect::<Vec<_>>();
            options.sort();
            pick_list(options, self.current_text_index.as_ref(), |key| {
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
        container(column![
            row![
                options,
                space::horizontal(),
                mouse_area(text("恢复").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer),
                mouse_area(text("删除").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer),
                mouse_area(text("另存为").size(FONT_SIZE_BIGGER))
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
            text_editor(&self.current_text)
                .on_action(TextBoardMessage::EditorAciton)
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
                })
        ])
        .into()
    }
}
