use std::path::PathBuf;
use tracing::info;
use crate::{
    constants::*,
    file_panel::operation::{FileTree, load_file_tree},
};
use iced::{alignment::Horizontal};
use iced::{
    Background, Length, Task, Theme,
    widget::{
        Column, Container, button, column, container, scrollable, text,
    },
};
mod context_menu;
mod operation; // 各种文件操作，新建、删除、重命名、移动等
mod search; // 过滤，搜索 // 右键菜单

#[derive(Debug, Clone)]
pub struct FilePanel {
    current_path: Option<PathBuf>,
    file_tree: Option<FileTree>,
    hovered_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum FilePanelMessage {
    FetchFileTree(Option<PathBuf>),
    LoadFileTree(Option<FileTree>),
    Expanded(usize),
    HoverEnter(usize),
    OperationOpenFolder,
}

impl FilePanel {
    pub fn new() -> Self {
        let file_panel = Self {
            current_path: None,
            file_tree: None,
            hovered_id: None,
        };

        file_panel
    }

    pub fn update(&mut self, file_panel_message: FilePanelMessage) -> Task<FilePanelMessage> {
        match file_panel_message {
            FilePanelMessage::LoadFileTree(file_tree) => {
                self.file_tree = file_tree;
                Task::none()
            }
            FilePanelMessage::FetchFileTree(path) => {
                if let Some(path) = path {
                    info!("获取文件路径成功!");
                    Task::perform(load_file_tree(path), |tree| {
                        FilePanelMessage::LoadFileTree(tree.ok())
                    })
                } else {
                    Task::none()
                }
            }
            FilePanelMessage::HoverEnter(id) => {
                self.hovered_id = Some(id);
                Task::none()
            }
            FilePanelMessage::Expanded(id) => {
                if let Some(ft) = self.file_tree.as_mut() {
                    ft.nodes[id].expanded = !ft.nodes[id].expanded
                };
                Task::none()
            }
            FilePanelMessage::OperationOpenFolder => {
                Task::perform(operation::open_filder(), 
                    
                        FilePanelMessage::FetchFileTree
                    )
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, FilePanelMessage> {
        container(scrollable(self.view_file_tree()))
            .padding(PADDING_BASE)
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weakest.color)),
                    ..container::Style::default()
                }
            })
    }
    
    // 递归渲染文件树
    pub fn view_file_tree(&self) -> Column<'_, FilePanelMessage> {
        let first_id = 0;
        let first_depth = 0;
        if let Some(ft) = &self.file_tree && !ft.nodes.is_empty() {
            if ft.nodes.len() > 1000 { // 避免递归爆栈
                return column![text("文件层级太深！").size(FONT_SIZE_BASE)].width(Length::Fill).align_x(Horizontal::Center);
            }
            operation::view_node(self.hovered_id.clone(), ft, first_id, first_depth)
        } else {
            column![button("添加文件").on_press(FilePanelMessage::OperationOpenFolder)].width(Length::Fill).align_x(Horizontal::Center)
        }
    }
}
