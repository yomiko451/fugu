use iced::widget::text_editor;



pub fn get_reverse_edit(edit: text_editor::Edit) {
    match edit {
        text_editor::Edit::Insert(c) => {}
        text_editor::Edit::Delete => {}
        text_editor::Edit::Backspace => {}
        text_editor::Edit::Indent => {}
        text_editor::Edit::Unindent => {}
        text_editor::Edit::Paste(s) => {}
        text_editor::Edit::Enter => {}
    }
}