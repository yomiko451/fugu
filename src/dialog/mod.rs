use editor_table::EditorTableDialog;
use iced::{
    Color, Element, Task,
    widget::{center, container, opaque, space, stack},
};

use crate::dialog::{confirm::ConfirmDialog, editor_table::EditorTableDialogMessage};

mod editor_table;
mod confirm;

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
    Confirm
}

#[derive(Debug, Clone)]
pub enum DialogMessage {
    OpenEditorTableDialog,
    OpenConfirmDialog(String),
    EditorTableMessage(EditorTableDialogMessage),
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            current_dialog: DialogType::default(),
            editor_table: EditorTableDialog::default(),
            confirm: ConfirmDialog::default()
        }
    }

    pub fn update(&mut self, message: DialogMessage) -> Task<DialogMessage> {
        match message {
            DialogMessage::OpenEditorTableDialog => {
                self.current_dialog = DialogType::EditorTable;
                Task::none()
            }
            DialogMessage::EditorTableMessage(editor_table_message) => match editor_table_message {
                EditorTableDialogMessage::CloseDialog => {
                    self.current_dialog = DialogType::default();
                    Task::none()
                }
                _ => self
                    .editor_table
                    .update(editor_table_message)
                    .map(DialogMessage::EditorTableMessage),
            },
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, DialogMessage> {
        match self.current_dialog {
            DialogType::EditorTable => self
                .editor_table
                .view()
                .map(DialogMessage::EditorTableMessage),
            _ => space().into(),
        }
    }

    pub fn is_show(&self) -> bool {
        !(DialogType::NoDialog == self.current_dialog)
    }
}
