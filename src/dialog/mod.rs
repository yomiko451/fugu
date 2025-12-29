use editor_table::EditorTable;
use iced::{
    Color, Element, Task,
    widget::{center, container, opaque, space, stack},
};

use crate::dialog::editor_table::EditorTableMessage;

mod editor_table;

#[derive(Debug, Default, Clone)]
pub struct Dialog {
    current_dialog: DialogType,
    editor_table: EditorTable,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum DialogType {
    #[default]
    NoDialog,
    EditorTable,
}

#[derive(Debug, Clone)]
pub enum DialogMessage {
    OpenEditorTableDialog,
    EditorTableMessage(EditorTableMessage),
}

impl Dialog {
    pub fn new() -> Self {
        Self {
            current_dialog: DialogType::default(),
            editor_table: EditorTable::default(),
        }
    }

    pub fn update(&mut self, message: DialogMessage) -> Task<DialogMessage> {
        match message {
            DialogMessage::OpenEditorTableDialog => {
                self.current_dialog = DialogType::EditorTable;
                Task::none()
            }
            DialogMessage::EditorTableMessage(editor_table_message) => match editor_table_message {
                EditorTableMessage::CloseDialog => {
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
