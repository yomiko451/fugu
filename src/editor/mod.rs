use iced::{widget::{container, text, Container}, Background, Task, Theme};

use crate::constants::*;



#[derive(Debug, Clone)]
pub struct Editor {
    
}


#[derive(Debug, Clone)]
pub enum EditorMessage {
    
}

impl Editor {
    pub fn new() -> Self {
        Self{}
    }
    
    pub fn update(&mut self, menu_bar_message: EditorMessage) -> Task<EditorMessage> {
        match menu_bar_message {
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, EditorMessage> {
        container(
            text("aaa")
        )
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                background: Some(Background::Color(palette.background)),
                ..container::Style::default()
            }
        })
    }
}