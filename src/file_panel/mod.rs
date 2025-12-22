use crate::{
    common::*,
    file_panel::{file_tree::{FileTree, FileTreeMessage, SaveMode}, operation::FileNode},
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
    OpenFolder,
    OpenFile,
    CreateNewFile,
    AutoSave(FileData),
    Save(FileData),
    SaveAs(FileData),
    ChangeMode(Mode),
    ReturnSaveResult(Result<(), AppError>),
    SendFileDataToEditor(FileData),
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
                    _ => self.file_tree.update(file_tree_message, setting).map(FilePanelMessage::FileTree)
                }
            }
            FilePanelMessage::OpenFolder => {
                Task::perform(operation::open_folder_dialog(),
                    |path| {
                        match path {
                            Some(path) => {
                                info!("获取文件路径成功!");
                                FilePanelMessage::FileTree(FileTreeMessage::FetchFileTree(path))
                            }
                            None => {
                                warn!("文件路径为空!");
                                FilePanelMessage::None
                            } // TODO
                        }
                    })
            }
            FilePanelMessage::OpenFile => {
                Task::perform(operation::open_file_dialog(), |path| match path {
                        Some(path) => {
                            let file_node = FileNode {
                                is_dir: path.is_dir(),
                                id: operation::get_next_id(),
                                expanded: false,
                                version: 0,
                                file_name: path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .into_owned(),
                                path: Some(path),
                                children: vec![],
                                content_cache: None,
                            };
                            FilePanelMessage::FileTree(FileTreeMessage::InsertToTempWorkplace(file_node))
                        }
                        None => {
                            warn!("文件路径为空!");
                            FilePanelMessage::None
                        }
                    })
            }
            FilePanelMessage::CreateNewFile => {
                let new_file_id = operation::get_next_id();
                let new_file = FileNode {
                    id: new_file_id,
                    is_dir: false,
                    children: vec![],
                    content_cache: None,
                    path: None,
                    expanded: false,
                    version: 0,
                    file_name: "新建文件.md".to_string(),
                };
                Task::done(FilePanelMessage::FileTree(FileTreeMessage::InsertNewFile(new_file_id, new_file)))
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
                text!("文件节点数  {}", self.file_tree.all_file_nodes.len() - 1).size(FONT_SIZE_BASE),
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
