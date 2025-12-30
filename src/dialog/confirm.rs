use crate::common::*;
use iced::{
    Background, Border, Color, Element, Length, Padding, Task, Theme, border::Radius, mouse, padding, widget::{column, container, mouse_area, row, space, text::Alignment}
};
use iced::widget::text;
#[derive(Debug, Default, Clone)]
pub struct ConfirmDialog {
    content: String,
}

#[derive(Debug, Clone)]
pub enum ConfirmDialogMessage {
    LoadConfirmText(String),
    SendConfirmResult(bool)
}

impl ConfirmDialog {
    pub fn update(&mut self, message: ConfirmDialogMessage) -> Task<ConfirmDialogMessage> {
        match message {
            ConfirmDialogMessage::LoadConfirmText(text) => {
                self.content = text;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ConfirmDialogMessage> {
        container(
            column![
                container(text("提醒!"))
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .padding(Padding::from([PADDING_SMALLER, PADDING_BASE]))
                    .style(|theme: &Theme| {
                        let ex_palette = theme.extended_palette();
                        container::Style {
                            background: Some(Background::Color(
                                ex_palette.background.strong.color.scale_alpha(0.75),
                            )),
                            border: Border {
                                color: Color::TRANSPARENT,
                                radius: Radius::default().top(DEFAULT_BORDER.radius.top_left),
                                ..DEFAULT_BORDER
                            },
                            ..Default::default()
                        }
                    }),
                text(&self.content).width(Length::Fill).align_x(Alignment::Center),
                row![
                    space::horizontal(),
                    mouse_area(text("确定"))
                        .interaction(mouse::Interaction::Pointer)
                        .on_press(ConfirmDialogMessage::SendConfirmResult(true)),
                    space::horizontal(),
                    mouse_area(text("取消"))
                        .interaction(mouse::Interaction::Pointer)
                        .on_press(ConfirmDialogMessage::SendConfirmResult(false)),
                    space::horizontal(),
                ].padding(padding::bottom(PADDING_BASE))
            ]
            .spacing(SPACING_BIGGER)
            .height(Length::Shrink)
            .width(CONFIRM_DIALOG_WIDTH),
        )
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weaker.color)),
                shadow: SHADOW_BASE_0_OFFSET,
                border: Border {
                    color: Color::TRANSPARENT,
                    ..DEFAULT_BORDER
                },
                ..Default::default()
            }
        })
        .into()
    }
}
