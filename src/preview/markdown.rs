use crate::{
    common::*,
    preview::{markdown, viewer::{CUSTOM_SETTINGS, CustomViewer}},
};
use iced::{
    Element, Length, Padding, Task, Theme,
    border::Radius,
    mouse,
    widget::{
        column, container, image,
        markdown::{self as iced_markdown, Uri},
        mouse_area, row, rule, scrollable, space, text,
    },
    window,
};
use std::{collections::HashMap, path::PathBuf, result, sync::Arc};
use tracing::info;

#[derive(Debug, Default)]
pub struct Markdown {
    raw: Option<Arc<String>>,
    content: iced_markdown::Content,
    image: HashMap<iced_markdown::Uri, image::Handle>,
}

#[derive(Debug, Clone)]
pub enum MarkdownMessage {
    LoadRawText(Arc<String>),
    RenderMarkdown,
    HandleImageUrl(Vec<iced_markdown::Uri>),
    InsertImageToDict(Vec<(iced_markdown::Uri, image::Handle)>),
    LinkClicked(iced_markdown::Uri),
}

impl Markdown {
    pub fn update(&mut self, markdown_message: MarkdownMessage) -> Task<MarkdownMessage> {
        match markdown_message {
            MarkdownMessage::LoadRawText(raw) => {
                self.raw = Some(raw);
                Task::done(MarkdownMessage::RenderMarkdown)
            }
            MarkdownMessage::RenderMarkdown => {
                if let Some(ref content) = self.raw {
                    self.content = iced_markdown::Content::parse(content);
                    let url_vec = self
                        .content
                        .images()
                        .iter()
                        .filter(|url| {
                            (url.ends_with("jpg") || url.ends_with("png"))
                                && !self.image.contains_key(url.as_str())
                        })
                        .cloned()
                        .collect::<Vec<iced_markdown::Uri>>();
                    return Task::done(MarkdownMessage::HandleImageUrl(url_vec));
                }
                Task::none()
            }
            MarkdownMessage::HandleImageUrl(url_vec) => Task::future(async {
                tokio::task::spawn_blocking(|| {
                    url_vec
                        .into_iter()
                        .map(|url| {
                            let handle = image::Handle::from_path(&url);
                            (url, handle)
                        })
                        .collect::<Vec<(iced_markdown::Uri, image::Handle)>>()
                })
                .await
            })
            .then(|result| match result {
                Ok(images) => Task::done(MarkdownMessage::InsertImageToDict(images)),
                Err(error) => {
                    info!("{}", error.to_string());
                    Task::none()
                }
            }),
            MarkdownMessage::InsertImageToDict(images) => {
                for (url, handle) in images {
                    self.image.insert(url, handle);
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, MarkdownMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        container(column![
            row![
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
            container(
                scrollable(iced_markdown::view_with(
                    self.content.items(),
                    *CUSTOM_SETTINGS,
                    &CustomViewer { image: &self.image },
                ))
                .direction(scrollable::Direction::Vertical(hidden_scroller))
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        ])
        .into()
    }
}
