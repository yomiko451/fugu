use std::path::PathBuf;

use crate::{
    common::*,
    file_panel::{file_tree::{FileTree, FileTreeMessage, SaveMode}, operation::{FileNode, NodeType}},
};
use iced::{
    Alignment,
    border::Radius,
    mouse,
    widget::{mouse_area, row},
};
use iced::{
    Background, Length, Padding, Task, Theme,
    widget::{Container, column, container, rule, text},
};
use tracing::{info, warn};
mod operation; // 各种文件操作，新建、删除、重命名、移动等
mod file_tree;
mod outline;

#[derive(Debug)]
pub struct FilePanel {
    file_tree: FileTree,
    mode: Mode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    FileTree,
    Content,
}

#[derive(Debug, Clone)]
pub enum FilePanelMessage {
    OpenMdFolder,
    OpenFile,
    CreateNewFile,
    AutoSave(FileData),
    Save(FileData),
    SaveAs(FileData),
    ImportImg,
    ImportImgFolder,
    
    
    ChangeMode(Mode),
    ReturnSaveResult(Result<(), AppError>),
    SendFileDataToEditor(FileData),
    SendImgPathToPreview(ImgData),
    None,
    FileTree(FileTreeMessage)
}

impl FilePanel {
    pub fn new() -> Self {
        Self {
            file_tree: FileTree::new(),
            mode: Mode::FileTree,
        }
    }

    pub fn update(
        &mut self,
        file_panel_message: FilePanelMessage,
        setting: &AppSetting,
    ) -> Task<FilePanelMessage> {
        match file_panel_message {
            FilePanelMessage::ChangeMode(mode) => {
                self.mode = mode;
                Task::none()
            }
            // 转发给文件树模块
            FilePanelMessage::FileTree(file_tree_message) => {
                match file_tree_message {
                    FileTreeMessage::ReturnSaveResult(result) => {
                        Task::done(FilePanelMessage::ReturnSaveResult(result))
                    }
                    FileTreeMessage::SendFileDataToEditor(file_data) => {
                        Task::done(FilePanelMessage::SendFileDataToEditor(file_data))
                    }
                    FileTreeMessage::SendImgDataToPreview(image_data) => {
                        Task::done(FilePanelMessage::SendImgPathToPreview(image_data))
                    }
                    _ => self.file_tree.update(file_tree_message, setting).map(FilePanelMessage::FileTree)
                }
            }
            FilePanelMessage::OpenMdFolder => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::OpenFile(NodeType::DirectoryMd)))
            }
            FilePanelMessage::OpenFile => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::OpenFile(NodeType::Markdown)))
            }
            FilePanelMessage::ImportImg => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::OpenFile(NodeType::Image)))
            }
            FilePanelMessage::ImportImgFolder => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::OpenFile(NodeType::DirectoryImg)))
            }
            FilePanelMessage::CreateNewFile => {
                let new_file = FileNode::new(None, NodeType::Markdown, Some("新建文件.md".to_string()));
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::InsertToFileTree(new_file)))
            }
            FilePanelMessage::AutoSave(file_data) => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::SaveFile(SaveMode::AutoSave, file_data)))
            }
            FilePanelMessage::Save(file_data) => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::SaveFile(SaveMode::ManualSave, file_data)))
            }
            FilePanelMessage::SaveAs(file_data) => {
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::SaveFile(SaveMode::SaveAs, file_data)))
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, FilePanelMessage> {
        let panel = match self.mode {
            Mode::FileTree => self.file_tree.view().map(FilePanelMessage::FileTree),
            Mode::Content => text("aa").into()
        };
        container(column![
            row![
                mouse_area(
                    text("文件")
                        .size(FONT_SIZE_BIGGER)
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::Center)
                )
                .on_press(FilePanelMessage::ChangeMode(Mode::FileTree))
                .interaction(mouse::Interaction::Pointer),
                mouse_area(
                    text("大纲")
                        .size(FONT_SIZE_BIGGER)
                        .width(Length::FillPortion(1))
                        .align_x(Alignment::Center)
                )
                .on_press(FilePanelMessage::ChangeMode(Mode::Content))
                .interaction(mouse::Interaction::Pointer),
            ]
            .spacing(SPACING_SMALLER)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink),
            row![
                rule::horizontal(1).style(|theme: &Theme| {
                    let ex_palette = theme.extended_palette();
                    let palette = theme.palette();
                    rule::Style {
                        color: if self.mode == Mode::FileTree {
                            palette.text
                        } else {
                            ex_palette.background.weaker.color
                        },
                        radius: Radius::default(),
                        snap: true,
                        fill_mode: rule::FillMode::Full,
                    }
                }),
                rule::horizontal(1).style(|theme: &Theme| {
                    let ex_palette = theme.extended_palette();
                    let palette = theme.palette();
                    rule::Style {
                        color: if self.mode == Mode::Content {
                            palette.text
                        } else {
                            ex_palette.background.weaker.color
                        },
                        radius: Radius::default(),
                        snap: true,
                        fill_mode: rule::FillMode::Full,
                    }
                })
            ],
            container(panel)
                .padding(PADDING_BASE)
                .height(Length::Fill),
            container(
                // 去除默认添加的临时工作区根节点所以要 -1
                text!("文件节点数  {}", self.file_tree.all_nodes.len() - 1).size(FONT_SIZE_BASE),
            )
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink)
            .width(Length::Fill)
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
}
