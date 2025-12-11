use iced::{widget::{container, markdown, scrollable, text, Container}, Background, Color, Element, Padding, Task, Theme};
use tracing::info;

use crate::common::*;

mod viewer;

#[derive(Debug, Clone)]
pub struct Preview {
    content: Option<String>,
    marddown: Vec<markdown::Item>
}

#[derive(Debug, Clone)]
pub enum PreviewMessage {
    GetInputContentFromEditor(String),
    RenderMarkdowm,
    None
}

impl Preview {
    pub fn new() -> Self {
        Self{
            content: None,
            marddown: vec![]
        }
    }
    
    pub fn update(&mut self, preview_message: PreviewMessage) -> Task<PreviewMessage> {
        match preview_message {
            PreviewMessage::GetInputContentFromEditor(content) => {
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
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, PreviewMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        container(
            scrollable(
                markdown::view(&self.marddown, DEFAULT_THEME.clone())
                    .map(|_|PreviewMessage::None)
        ).direction(scrollable::Direction::Vertical(hidden_scroller))
        ).padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weakest.color)),
                ..container::Style::default()
            }
        })
    }
}