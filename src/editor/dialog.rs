use iced::{Element, Task, mouse, widget::{Container, column, container, mouse_area, row, text, text_input}};


#[derive(Debug, Default, Clone)]
pub struct TableDialog {
    row: String,
    column: String
}

#[derive(Debug, Clone)]
pub enum TableDialogMessage {
    RowChanged(String),
    ColumnChanged(String),
    ConfirmInput,
    CancelInput,
    CloseDialog
}

impl TableDialog {
    pub fn update(&mut self, message: TableDialogMessage) -> Task<TableDialogMessage> {
        match message {
            TableDialogMessage::RowChanged(row) => {
                self.row = row;
                Task::none()
            }
            TableDialogMessage::ColumnChanged(column) => {
                self.column = column;
                Task::none()
            }
            TableDialogMessage::ConfirmInput => {
                Task::none()
            }
            TableDialogMessage::CancelInput => {
                self.row = "".to_string();
                self.column = "".to_string();
                Task::done(TableDialogMessage::CloseDialog)
            }
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Element<'_, TableDialogMessage> {
        container(column![
            text("请输入行和列"),
            row![
                text("行"),
                text_input("行", &self.row).on_input(TableDialogMessage::RowChanged)
            ],
            row![
                text("列"),
                text_input("列", &self.column).on_input(TableDialogMessage::ColumnChanged)
            ],
            row![
                text("确定"),
                mouse_area(text("取消")).interaction(mouse::Interaction::Pointer)
                    .on_press(TableDialogMessage::CancelInput)
            ]
        ]).into()
    }
}
