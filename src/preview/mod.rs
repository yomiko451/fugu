use std::{fmt::write, sync::Arc};

use iced::{
    alignment::{Horizontal, Vertical::Bottom}, border::Radius, mouse, overlay::menu, widget::{
        column, combo_box, container, markdown, mouse_area, pick_list, row, rule, scrollable, space, text, text_editor, Container
    }, Background, Border, Color, Element, Length, Padding, Shadow, Task, Theme
};
use tracing::info;

use crate::common::*;

mod viewer;

#[derive(Debug, Clone)]
pub struct Preview {
    current_page: PreviewPage,
    content: Option<Arc<String>>,
    marddown: Vec<markdown::Item>,
    snap_shot_index_state: Vec<String>,
    current_snap_shot_index: Option<String>,
    current_snapshot_content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum PreviewMessage {
    SyncContnetWithEditor(Arc<String>),
    RenderMarkdowm,
    EditorAction(text_editor::Action),
    ChangePageTo(PreviewPage),
    ChangeSnapShot(String),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PreviewPage {
    MarkDown,
    SnapShot,
    ClipBoard,
    Log,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            current_page: PreviewPage::SnapShot,
            current_snapshot_content: text_editor::Content::default(),
            content: None,
            marddown: vec![],
            current_snap_shot_index: None,
            snap_shot_index_state: vec![
                "快照1".to_string(),
                "快照2".to_string(),
                "快照3".to_string(),
            ],
        }
    }

    pub fn update(&mut self, preview_message: PreviewMessage) -> Task<PreviewMessage> {
        match preview_message {
            PreviewMessage::SyncContnetWithEditor(content) => {
                self.content = Some(content);
                Task::done(PreviewMessage::RenderMarkdowm)
            }
            PreviewMessage::RenderMarkdowm => {
                if let Some(ref content) = self.content {
                    self.marddown = markdown::parse(content).collect();
                    info!("文件内容渲染成功!");
                }
                Task::none()
            }
            PreviewMessage::EditorAction(action) => {
                if !action.is_edit() {
                    self.current_snapshot_content.perform(action);
                }
                Task::none()
            }
            PreviewMessage::ChangePageTo(page) => {
                self.current_page = page;
                Task::none()
            }
            PreviewMessage::ChangeSnapShot(snap_shot_index) => {
                self.current_snap_shot_index = Some(snap_shot_index);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, PreviewMessage> {
        let preview = match self.current_page {
            PreviewPage::MarkDown => self.generate_markdown_preview(),
            PreviewPage::SnapShot => self.generate_snapshot_component(),
            _ => self.generate_markdown_preview(),
        };

        container(column![
            container(
                row![
                    self.generate_page_change_button("预览", PreviewPage::MarkDown),
                    self.generate_page_change_button("快照", PreviewPage::SnapShot),
                    self.generate_page_change_button("日志", PreviewPage::Log),
                    self.generate_page_change_button("剪切板", PreviewPage::ClipBoard),
                ]
                .height(Length::Shrink)
            )
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weaker.color)),
                    ..container::Style::default()
                }
            }),
            preview,
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
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weakest.color)),
                ..container::Style::default()
            }
        })
    }

    pub fn generate_page_change_button(
        &self,
        label: &'static str,
        page: PreviewPage,
    ) -> Element<'_, PreviewMessage> {
        mouse_area(
            container(
                text(label)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .size(FONT_SIZE_BIGGER),
            )
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .style(move |theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(if self.current_page == page {
                        ex_palette.background.weakest.color
                    } else {
                        ex_palette.background.weaker.color
                    })),
                    ..container::Style::default()
                }
            }),
        )
        .interaction(mouse::Interaction::Pointer)
        .on_press(PreviewMessage::ChangePageTo(page))
        .into()
    }

    pub fn generate_markdown_preview(&self) -> Element<'_, PreviewMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        container(
            scrollable(
                markdown::view(&self.marddown, DEFAULT_THEME.clone()).map(|_| PreviewMessage::None),
            )
            .direction(scrollable::Direction::Vertical(hidden_scroller)),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        .into()
    }

    pub fn generate_snapshot_component(&self) -> Element<'_, PreviewMessage> {
        column![
            row![
                pick_list(
                    self.snap_shot_index_state.as_slice(),
                    self.current_snap_shot_index.as_ref(),
                    PreviewMessage::ChangeSnapShot
                )
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
                }).menu_style(|theme: &Theme| {
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
                }),
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
            text_editor(&self.current_snapshot_content)
                .on_action(PreviewMessage::EditorAction)
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
        ]
        .into()
    }
}
