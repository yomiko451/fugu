use iced::{widget::{container, text, Container}, Background, Color, Element, Task, Theme};

use crate::common::*;


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
    
    pub fn update(&mut self, status_bar_message: StatusBarMessage) -> Task<StatusBarMessage> {
        match status_bar_message {
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, StatusBarMessage> {
        container(
            text("aaa")
        )
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weaker.color)),
                ..container::Style::default()
            }
        })
    }
}