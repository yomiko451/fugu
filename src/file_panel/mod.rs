use crate::{
    common::*,
    file_panel::operation::{FileTree, load_file_tree},
};
use iced::{
    Background, Border, Element, Length, Padding, Task, Theme,
    alignment::Horizontal::Left,
    border::Radius,
    widget::{Column, Container, button, column, container, scrollable, space, text},
};
use iced::{
    alignment::Horizontal,
    mouse,
    widget::{center, mouse_area, row},
};
use std::path::PathBuf;
use tracing::{error, info, warn};
mod operation; // 各种文件操作，新建、删除、重命名、移动等

#[derive(Debug, Clone)]
pub struct FilePanel {
    current_path: Option<PathBuf>,
    file_tree: Option<FileTree>,
    hovered_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum FilePanelMessage {
    LoadFileTree(Option<FileTree>),
    LoadFileContent(Option<PathBuf>),
    SelectFileNode(usize),
    HoverEnter(usize),
    OperationOpenFolder,
    OperationOpenFile,
    SendSelectedFileDataToEditor(FileData),
    SendSaveSuccessToEditor,
    SaveFileDataFromEditor(FileData),
    LogError(String),
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
            FilePanelMessage::LoadFileContent(path) => {
                if let Some(path) = path {
                    Task::perform(operation::read_file(path), |file_data| match file_data {
                        Ok(file_data) => {
                            info!("文件数据读取成功!");
                            FilePanelMessage::SendSelectedFileDataToEditor(file_data)
                        }
                        Err(error) => FilePanelMessage::LogError(error.to_string()),
                    })
                } else {
                    info!("文件路径获取失败!");
                    Task::none()
                }
            }
            FilePanelMessage::HoverEnter(id) => {
                self.hovered_id = Some(id);
                Task::none()
            }
            FilePanelMessage::SelectFileNode(id) => {
                if let Some(ft) = self.file_tree.as_mut() {
                    let node = &mut ft.nodes[id];
                    if node.node.is_dir() {
                        node.expanded = !node.expanded;
                    } else {
                        return Task::done(FilePanelMessage::LoadFileContent(Some(
                            node.node.clone(),
                        )));
                    }
                };
                Task::none()
            }
            FilePanelMessage::OperationOpenFolder => Task::future(operation::open_folder_dialog())
                .then(|path| {
                    match path {
                        Some(path) => {
                            info!("获取文件路径成功!");
                            Task::perform(load_file_tree(path), |tree| {
                                FilePanelMessage::LoadFileTree(tree.ok())
                            })
                        }
                        None => {
                            warn!("文件路径为空!");
                            Task::none()
                        } // TODO
                    }
                }),
            FilePanelMessage::OperationOpenFile => Task::perform(
                operation::open_file_dialog(),
                FilePanelMessage::LoadFileContent,
            ),
            FilePanelMessage::SaveFileDataFromEditor(file_data) => {
                Task::perform(operation::save_file(file_data), |result| match result {
                    Ok(_) => FilePanelMessage::SendSaveSuccessToEditor,
                    Err(error) => FilePanelMessage::LogError(error.to_string()),
                })
            }
            FilePanelMessage::LogError(error_info) => {
                error!(error_info);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, FilePanelMessage> {
        container(column![
            container(self.view_file_tree()).padding(PADDING_BASE)
                .height(Length::Fill),
            container(
                row![
                    text("恢复").size(FONT_SIZE_BASE),
                    space::horizontal(),
                    text("恢复").size(FONT_SIZE_BASE)
                ]
                .width(Length::Fill)
                .spacing(SPACING)
            )
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink)
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weaker.color)),
                    ..container::Style::default()
                }
            })
        ])
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weakest.color)),
                ..container::Style::default()
            }
        })
    }

    // 递归渲染文件树
    pub fn view_file_tree(&self) -> Element<'_, FilePanelMessage> {
        let first_id = 0;
        let first_depth = 0;
        if let Some(ft) = &self.file_tree
            && !ft.nodes.is_empty()
        {
            if ft.nodes.len() > 1000 {
                // 避免递归爆栈
                return column![text("文件层级太深！").size(FONT_SIZE_BASE)]
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .into();
            }
            scrollable(operation::view_node(
                self.hovered_id.clone(),
                ft,
                first_id,
                first_depth,
            ))
            .into()
        } else {
            center(space()).into()
        }
    }
}
