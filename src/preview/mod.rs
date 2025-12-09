use iced::{widget::{container, text, Container}, Background, Color, Element, Task};

use crate::constants::PREVIEW_BG_COLOR;


#[derive(Debug, Clone)]
pub struct Preview {
    
}

#[derive(Debug, Clone)]
pub enum PreviewMessage {
    
}

impl Preview {
    pub fn new() -> Self {
        Self{}
    }
    
    pub fn update(&mut self, menu_bar_message: PreviewMessage) -> Task<PreviewMessage> {
        match menu_bar_message {
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, PreviewMessage> {
        container(
            text("aaa")
        )
        .style(|_| {
            container::Style {
                background: Some(Background::Color(PREVIEW_BG_COLOR)),
                ..container::Style::default()
            }
        })
    }
}