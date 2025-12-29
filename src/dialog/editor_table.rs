use iced::{
    Alignment, Background, Border, Element, Length, Padding, Task, Theme, mouse,
    widget::{column, container, mouse_area, row, space, text, text_input},
};

use crate::common::*;

#[derive(Debug, Default, Clone)]
pub struct EditorTable {
    row: String,
    column: String,
}

#[derive(Debug, Clone)]
pub enum EditorTableMessage {
    RowChanged(String),
    ColumnChanged(String),
    ConfirmInput,
    CancelInput,
    CloseDialog,
}

impl EditorTable {
    pub fn update(&mut self, message: EditorTableMessage) -> Task<EditorTableMessage> {
        match message {
            EditorTableMessage::RowChanged(row) => {
                self.row = row;
                Task::none()
            }
            EditorTableMessage::ColumnChanged(column) => {
                self.column = column;
                Task::none()
            }
            EditorTableMessage::ConfirmInput => Task::none(),
            EditorTableMessage::CancelInput => {
                self.row = "".to_string();
                self.column = "".to_string();
                Task::done(EditorTableMessage::CloseDialog)
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, EditorTableMessage> {
        container(column![
            space::vertical(),
            text("请输入行和列")
                .width(Length::Fill)
                .align_x(Alignment::Center),
            space::vertical(),
            column![
                text_input("行", &self.row)
                    .line_height(1.)
                    .on_input(EditorTableMessage::RowChanged),
                text_input("列", &self.column)
                    .line_height(1.)
                    .on_input(EditorTableMessage::ColumnChanged)
            ]
            .spacing(SPACING_SMALLER),
            space::vertical(),
            row![
                space::horizontal(),
                mouse_area(text("确定"))
                    .interaction(mouse::Interaction::Pointer)
                    .on_press(EditorTableMessage::CancelInput),
                space::horizontal(),
                mouse_area(text("取消"))
                    .interaction(mouse::Interaction::Pointer)
                    .on_press(EditorTableMessage::CancelInput),
                space::horizontal(),
            ],
            space::vertical()
        ])
        .width(TABLE_DIALOG_WIDTH)
        .height(TABLE_DIALOG_HEIGHT)
        .padding(Padding::from([0., PADDING_BIGGER]))
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weaker.color)),
                shadow: SHADOW_BASE_0_OFFSET,
                border: Border {
                    color: ex_palette.background.weaker.color,
                    ..DEFAULT_BORDER
                },
                ..Default::default()
            }
        })
        .into()
    }
}
