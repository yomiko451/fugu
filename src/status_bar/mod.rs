use iced::{widget::{container, text, Container}, Background, Color, Element, Task};

use crate::constants::MENU_BAR_AND_STATUS_BAR_BG_COLOR;


#[derive(Debug, Clone)]
pub struct StatusBar {
    
}

#[derive(Debug, Clone)]
pub enum StatusBarMessage {
    
}

impl StatusBar {
    pub fn new() -> Self {
        Self{}
    }
    
    pub fn update(&mut self, menu_bar_message: StatusBarMessage) -> Task<StatusBarMessage> {
        match menu_bar_message {
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, StatusBarMessage> {
        container(
            text("aaa")
        )
        .style(|_| {
            container::Style {
                background: Some(Background::Color(MENU_BAR_AND_STATUS_BAR_BG_COLOR)),
                ..container::Style::default()
            }
        })
    }
}