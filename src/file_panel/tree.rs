use iced::{widget::{mouse_area, text, Button, Column, Row}, Padding};
use crate::file_panel::{FilePanel, FilePanelMessage};

impl FilePanel {
    pub fn view_node(&self, id: usize, depth: usize) -> Column<'_, FilePanelMessage> {
        let node = &self.file_tree.nodes[id];
    
        let mut col = Column::new()
            .push(
                Row::new()
                    .padding(Padding::default().left(depth as u32 * 20))
                    .push(
                        mouse_area(
                            text(node.node.file_name().unwrap().to_string_lossy()).size(10).wrapping(text::Wrapping::None)
                        )
                            .on_press(FilePanelMessage::Toggle(id))
                    )
            );
    
        if !node.children.is_empty() {
            for &child in &node.children {
                col = col.push(self.view_node(child, depth + 1));
            }
        }
    
        col.into()
    }
}