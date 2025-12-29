use iced::{Element, Task, widget::{space, row, text, column, container}};


#[derive(Debug, Default, Clone)]
pub struct ConfirmDialog {
    content: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ConfirmDialogMessage {
    
}

impl ConfirmDialog {
    pub fn update(&mut self, message: ConfirmDialogMessage) -> Task<ConfirmDialogMessage> {
        match message {
            
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Element<'_, ConfirmDialogMessage> {
        container(
            column![
                text("提醒"),
                text(&self.content),
                row![
                    space::horizontal(),
                    text("确定"),
                    space::horizontal(),
                    text("取消"),
                    space::horizontal()
                ]
            ]
        ).into()
    }
}