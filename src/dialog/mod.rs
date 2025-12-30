use editor_table::EditorTableDialog;
use iced::{
    Color, Element, Task,
    widget::{center, container, opaque, space, stack},
};

use crate::dialog::{
    confirm::{ConfirmDialog, ConfirmDialogMessage},
    editor_table::EditorTableDialogMessage,
};

mod confirm;
mod editor_table;

#[derive(Debug, Default, Clone)]
pub struct Dialog {
    current_dialog: DialogType,
    editor_table: EditorTableDialog,
    confirm: ConfirmDialog,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DialogType {
    #[default]
    NoDialog,
    EditorTable,
    Confirm,
}

#[derive(Debug, Clone)]
pub enum DialogMessage {
    OpenEditorTableDialog,
    OpenConfirmDialog(String),
    EditorTableDialogMessage(EditorTableDialogMessage),
    ConfirmDialogMessage(ConfirmDialogMessage),
    SendConfirmResult(bool)
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            current_dialog: DialogType::default(),
            editor_table: EditorTableDialog::default(),
            confirm: ConfirmDialog::default(),
        }
    }

    pub fn update(&mut self, message: DialogMessage) -> Task<DialogMessage> {
        match message {
            DialogMessage::OpenEditorTableDialog => {
                self.current_dialog = DialogType::EditorTable;
                Task::none()
            }
            DialogMessage::OpenConfirmDialog(text) => {
                self.current_dialog = DialogType::Confirm;
                Task::done(DialogMessage::ConfirmDialogMessage(
                    ConfirmDialogMessage::LoadConfirmText(text),
                ))
            }
            DialogMessage::EditorTableDialogMessage(editor_table_message) => {
                match editor_table_message {
                    EditorTableDialogMessage::CloseDialog => {
                        self.current_dialog = DialogType::default();
                        Task::none()
                    }
                    _ => self
                        .editor_table
                        .update(editor_table_message)
                        .map(DialogMessage::EditorTableDialogMessage),
                }
            }
            DialogMessage::ConfirmDialogMessage(confirm_dialog_message) => {
                match confirm_dialog_message {
                    ConfirmDialogMessage::SendConfirmResult(is_user_agreed) => {
                        self.current_dialog = DialogType::default();
                    Task::done(DialogMessage::SendConfirmResult(is_user_agreed))
                    }
                    _ => self
                        .confirm
                        .update(confirm_dialog_message)
                        .map(DialogMessage::ConfirmDialogMessage),
                }
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, DialogMessage> {
        match self.current_dialog {
            DialogType::EditorTable => self
                .editor_table
                .view()
                .map(DialogMessage::EditorTableDialogMessage),
            DialogType::Confirm => self.confirm.view().map(DialogMessage::ConfirmDialogMessage),
            _ => space().into(),
        }
    }

    pub fn is_show(&self) -> bool {
        !(DialogType::NoDialog == self.current_dialog)
    }
}
