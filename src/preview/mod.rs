use iced::{widget::{container, text, Container}, Background, Color, Element, Task, Theme};

use crate::constants::*;


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
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weakest.color)),
                ..container::Style::default()
            }
        })
    }
}