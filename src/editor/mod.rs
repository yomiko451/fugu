use iced::{widget::{container, text, Container}, Background, Color, Element, Task};

use crate::constants::EDITOR_BG_COLOR;



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
        .style(|_| {
            container::Style {
                background: Some(Background::Color(EDITOR_BG_COLOR)),
                ..container::Style::default()
            }
        })
    }
}