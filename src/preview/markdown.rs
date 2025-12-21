use std::{collections::HashMap, path::PathBuf, result, sync::Arc};
use tracing::info;
use iced::{Element, Length, Padding, Task, widget::{container, image, markdown as iced_markdown, scrollable}};
use crate::{common::*, preview::viewer::CustomViewer};

#[derive(Debug, Default)]
pub struct Markdown {
    raw: Option<Arc<String>>,
    content: iced_markdown::Content,
    image: HashMap<iced_markdown::Uri, image::Handle>
}

#[derive(Debug, Clone)]
pub enum MarkdownMessage {
    LoadRawText(Arc<String>),
    RenderMarkdown,
    HandleImageUrl(iced_markdown::Uri),
    InsertImageToDict(iced_markdown::Uri, image::Handle),
    LinkClicked(iced_markdown::Uri)
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
                    info!("文件内容渲染成功!");
                }
                Task::none()
            }
            MarkdownMessage::HandleImageUrl(url) => {
                if url.ends_with("jpg") || url.ends_with("png") {
                    return Task::future(
                        async {
                            tokio::task::spawn_blocking(|| {
                                let handle = image::Handle::from_path(&url);
                                (url, handle)
                            }).await
                        }
                    ).then(|result| {
                        match result {
                            Ok((url, handle)) => Task::done(MarkdownMessage::InsertImageToDict(url, handle)),
                            Err(error) => {
                                info!("{}", error.to_string());
                                Task::none()
                            }
                        }
                    })
                }
                Task::none()
            }
            MarkdownMessage::InsertImageToDict(url, handle) => {
                if !self.image.contains_key(&url) {
                    self.image.insert(url, handle);
                    //return Task::done(MarkdownMessage::RenderMarkdown);
                }
                Task::none()
            }
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Element<'_, MarkdownMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        container(
            scrollable(
                iced_markdown::view_with(self.content.items(), DEFAULT_THEME.clone(), &CustomViewer {
                    image: &self.image
                })
            )
            .direction(scrollable::Direction::Vertical(hidden_scroller)),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        .into()
    }
}