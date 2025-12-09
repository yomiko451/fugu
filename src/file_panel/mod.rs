use iced::{widget::{button, column, container, scrollable, text, Column, Container, Scrollable}, Background, Color, Element, Task};
use iced::widget::{scrollable::Scrollbar};
use crate::{constants::FILE_PANEL_BG_COLOR, file_panel::loader::{load_file_tree, FileTree}};
mod tree;  // 展示文件内容
mod search;  // 过滤，搜索
mod operation; // 各种文件操作，新建、删除、重命名、移动等
mod loader; // 异步加载文件内容
mod context_menu; // 右键菜单

#[derive(Debug, Clone)]
pub struct FilePanel {
    pub file_tree: FileTree
}

#[derive(Debug, Clone)]
pub enum FilePanelMessage {
    FetchFileTree,
    LoadFileTree(FileTree),
    Toggle(usize)
}

impl FilePanel {
    pub fn new() -> Self {
        let file_panel= Self{
            file_tree: FileTree {
                nodes: vec![]
            }
        };
        
        file_panel
    }
    
    pub fn update(&mut self, file_panel_message: FilePanelMessage) -> Task<FilePanelMessage> {
        match file_panel_message {
            FilePanelMessage::LoadFileTree(tree) => {
                self.file_tree = tree;
                Task::none()
            }
            FilePanelMessage::FetchFileTree => {
                let pwd = std::env::current_dir().unwrap();
                Task::perform(load_file_tree(pwd), |s| FilePanelMessage::LoadFileTree(s.unwrap()))
            }
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, FilePanelMessage> {
        let mut col = Column::new();
        if !self.file_tree.nodes.is_empty() {
            col = col.push(self.view_node(0, 0));
        }
        
        container(
            column![
                button("打开项目").on_press(FilePanelMessage::FetchFileTree),
                scrollable(col).direction(scrollable::Direction::Both { vertical: Scrollbar::new(), horizontal: Scrollbar::new() })
            ]
        )
        .style(|_| {
            container::Style {
                background: Some(Background::Color(FILE_PANEL_BG_COLOR)),
                ..container::Style::default()
            }
        })
    }
}