use crate::{
    common::*,
    file_panel::operation::{FileNode, load_file_tree},
};
use iced::{Alignment, border::Radius, mouse, widget::{mouse_area, row}};
use iced::{
    Background, Element, Length, Padding, Task, Theme,
    widget::{Column, Container, column, container, rule, scrollable, space, text},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tracing::{error, info, warn};
mod operation; // 各种文件操作，新建、删除、重命名、移动等

#[derive(Debug)]
pub struct FilePanel {
    roo_path: Option<PathBuf>,
    workplace_root_key: Option<u32>,
    temp_workplace_root_key: u32,
    all_file_nodes: HashMap<u32, FileNode>,
    hovered_file_node_id: Option<u32>,
    selected_file_node_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum FilePanelMessage {
    FetchFileTree(PathBuf),
    LoadWorkplace(u32, HashMap<u32, FileNode>, PathBuf),
    InsertToTempWorkplace(FileNode),
    ChangeSelectedFileNode(u32),
    ChangeHoveredFileNode(u32),

    OperationOpenFolder,
    OperationOpenFile,
    OperationCreateNewFile,
    OperationAutoSave(FileData),
    OperationSaveFile(FileData),
    OperationSaveAs(FileData),
    ReturnSaveResult(Result<(), AppError>),

    SendSelectedFileDataToEditor(FileData),

    RenameFileNodeFromEditor(String),
    UpdateSelectedFileNodeCache(FileData),
    UpdateSelectedFilePathAndSave(PathBuf, FileData),
    LogError(String),
    None,
}

impl FilePanel {
    pub fn new() -> Self {
        let temp_workplace_id = operation::get_next_id();
        let temp_workplace_root_node = FileNode {
            is_dir: true,
            id: temp_workplace_id,
            expanded: true,
            path: None,
            version: 0,
            file_name: "临时工作区".to_string(),
            children: vec![],
            content_cache: None,
        };
        let mut all_file_nodes = HashMap::new();
        all_file_nodes.insert(temp_workplace_id, temp_workplace_root_node);
        let file_panel = Self {
            roo_path: None,
            all_file_nodes,
            workplace_root_key: None,
            temp_workplace_root_key: temp_workplace_id,
            selected_file_node_id: None,
            hovered_file_node_id: None,
        };
        file_panel
    }

    pub fn update(
        &mut self,
        file_panel_message: FilePanelMessage,
        setting: &AppSetting,
    ) -> Task<FilePanelMessage> {
        match file_panel_message {
            FilePanelMessage::OperationCreateNewFile => {
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
                self.all_file_nodes.insert(new_file_id, new_file);
                let temp_workplace_root_node = self
                    .all_file_nodes
                    .get_mut(&self.temp_workplace_root_key)
                    .expect("初始化必定插入此键值对不应当出错!");
                temp_workplace_root_node.children.push(new_file_id);
                Task::done(FilePanelMessage::ChangeSelectedFileNode(new_file_id))
            }
            FilePanelMessage::FetchFileTree(path) => {
                Task::perform(load_file_tree(path.clone()), |tree| match tree {
                    Ok((root_file_node_key, all_files_tree)) => {
                        FilePanelMessage::LoadWorkplace(root_file_node_key, all_files_tree, path)
                    }
                    Err(error) => FilePanelMessage::LogError(error.to_string()),
                })
            }
            // FilePanelMessage::RenameFileNodeFromEditor(new_name) => {
            //     self.selected_file_node_id.and_then(|key| {
            //         self.all_file_nodes
            //             .get_mut(&key)
            //             .and_then(|selected_file_node| {
            //                 selected_file_node.file_name = new_name.clone();
            //                 selected_file_node.path.as_mut().map(|path| {
            //                     *path = path
            //                         .parent()
            //                         .expect("父路径必定存在不应该出错!")
            //                         .join(new_name);
            //                     Task::done(FilePanelMessage::OperationSaveFileData)
            //                 })
            //             })
            //     });
            //     Task::none()
            // }
            FilePanelMessage::LoadWorkplace(root_file_node_key, all_files_tree, root_path) => {
                self.workplace_root_key = Some(root_file_node_key);
                self.all_file_nodes.extend(all_files_tree);
                self.roo_path = Some(root_path);
                Task::none()
            }
            FilePanelMessage::InsertToTempWorkplace(file_node) => {
                let temp_workplace_root_node = self
                    .all_file_nodes
                    .get_mut(&self.temp_workplace_root_key)
                    .unwrap();
                let file_node_id = file_node.id;
                temp_workplace_root_node.children.push(file_node.id);
                self.all_file_nodes.insert(file_node.id, file_node);
                Task::done(FilePanelMessage::ChangeSelectedFileNode(file_node_id))
            }
            FilePanelMessage::ChangeHoveredFileNode(id) => {
                self.hovered_file_node_id = Some(id);
                Task::none()
            }
            FilePanelMessage::ChangeSelectedFileNode(key) => {
                let node = self.all_file_nodes.get_mut(&key).unwrap();
                if node.is_dir {
                    node.expanded = !node.expanded;
                } else {
                    self.selected_file_node_id = Some(key);
                    if let Some(ref cache) = node.content_cache {
                        let file_data = FileData {
                            version: node.version,
                            content: Arc::clone(cache),
                        };
                        return Task::done(FilePanelMessage::SendSelectedFileDataToEditor(
                            file_data,
                        ));
                    } else {
                        if let Some(ref path) = node.path {
                            return Task::perform(
                                operation::read_file(path.clone()),
                                |file_data| match file_data {
                                    Ok(file_data) => {
                                        info!("文件数据读取成功!");
                                        FilePanelMessage::UpdateSelectedFileNodeCache(file_data)
                                    }
                                    Err(error) => FilePanelMessage::LogError(error.to_string()),
                                },
                            );
                        } else {
                            let file_data = FileData {
                                version: 0,
                                content: Arc::new(String::new()),
                            };
                            return Task::done(FilePanelMessage::UpdateSelectedFileNodeCache(
                                file_data,
                            ));
                        }
                    }
                }
                Task::none()
            }
            FilePanelMessage::UpdateSelectedFileNodeCache(file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_file_nodes.get_mut(&key) {
                        selected_file_data.content_cache = Some(Arc::clone(&file_data.content));
                        return Task::done(FilePanelMessage::SendSelectedFileDataToEditor(
                            file_data,
                        ));
                    }
                }
                Task::none()
            }
            FilePanelMessage::UpdateSelectedFilePathAndSave(path, file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_file_nodes.get_mut(&key) {
                        if selected_file_data.path.is_none() {
                            selected_file_data.file_name =
                                path.file_name().unwrap().to_string_lossy().into();
                            selected_file_data.path = Some(path);
                            return Task::done(FilePanelMessage::OperationSaveFile(file_data));
                        } else {
                            selected_file_data.path = Some(path);
                            return Task::done(FilePanelMessage::OperationSaveFile(file_data))
                                .chain(Task::done(FilePanelMessage::FetchFileTree(
                                    self.roo_path.as_ref().expect("不应当为空!").clone(),
                                )));
                        }
                    }
                }
                Task::none()
            }
            FilePanelMessage::OperationOpenFolder => Task::future(operation::open_folder_dialog())
                .then(|path| {
                    match path {
                        Some(path) => {
                            info!("获取文件路径成功!");
                            Task::done(FilePanelMessage::FetchFileTree(path))
                        }
                        None => {
                            warn!("文件路径为空!");
                            Task::none()
                        } // TODO
                    }
                }),
            FilePanelMessage::OperationOpenFile => Task::future(operation::open_file_dialog())
                .then(|path| match path {
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
                        Task::done(FilePanelMessage::InsertToTempWorkplace(file_node))
                    }
                    None => {
                        warn!("文件路径为空!");
                        Task::none()
                    }
                }),
            FilePanelMessage::OperationAutoSave(file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_file_nodes.get_mut(&key) {
                        selected_file_data.content_cache = Some(Arc::clone(&file_data.content));
                        selected_file_data.version = file_data.version;
                        if let Some(path) = selected_file_data.path.as_ref() {
                            return Task::perform(
                                operation::save_file(path.clone(), file_data.content),
                                |result| match result {
                                    Ok(_) => {
                                        info!("文件自动保存成功!");
                                        FilePanelMessage::ReturnSaveResult(Ok(()))
                                    }
                                    Err(error) => FilePanelMessage::ReturnSaveResult(Err(error)),
                                },
                            );
                        } else {
                            info!("文件保存缓冲区成功!");
                            return Task::done(FilePanelMessage::ReturnSaveResult(Ok(())));
                        }
                    };
                }
                Task::done(FilePanelMessage::ReturnSaveResult(Err(
                    AppError::FilePanelError("文件自动保存失败!".to_string()),
                )))
            }
            FilePanelMessage::OperationSaveFile(file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_file_nodes.get_mut(&key) {
                        selected_file_data.content_cache = Some(Arc::clone(&file_data.content));
                        selected_file_data.version = file_data.version;
                        if let Some(path) = selected_file_data.path.as_ref() {
                            return Task::perform(
                                operation::save_file(path.clone(), file_data.content),
                                |result| match result {
                                    Ok(_) => {
                                        info!("文件手动保存成功!");
                                        FilePanelMessage::ReturnSaveResult(Ok(()))
                                    }
                                    Err(error) => FilePanelMessage::ReturnSaveResult(Err(error)),
                                },
                            );
                        } else {
                            return Task::done(FilePanelMessage::OperationSaveAs(file_data));
                        }
                    };
                }
                Task::done(FilePanelMessage::ReturnSaveResult(Err(
                    AppError::FilePanelError("文件保存失败!".to_string()),
                )))
            }
            FilePanelMessage::OperationSaveAs(file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_node) = self.all_file_nodes.get(&key) {
                        return Task::future(operation::save_file_dialog(
                            selected_node.file_name.clone(),
                        ))
                        .then(move |path| match path {
                            Some(path) => {
                                Task::done(FilePanelMessage::UpdateSelectedFilePathAndSave(
                                    path,
                                    file_data.clone(),
                                ))
                            }
                            None => {
                                warn!("文件路径为空!");
                                Task::none()
                            }
                        });
                    }
                }
                Task::none()
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
            row![
                
                mouse_area(text("文件").size(FONT_SIZE_BIGGER)
                    .width(Length::FillPortion(1))
                    .align_x(Alignment::Center))
                    .interaction(mouse::Interaction::Pointer),
                mouse_area(text("大纲").size(FONT_SIZE_BIGGER)
                    .width(Length::FillPortion(1))
                    .align_x(Alignment::Center))
                    .interaction(mouse::Interaction::Pointer),
                
            ]
            .spacing(SPACING_SMALLER)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink),
            rule::horizontal(1).style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                rule::Style {
                    color: ex_palette.background.weaker.color,
                    radius: Radius::default(),
                    snap: true,
                    fill_mode: rule::FillMode::Full,
                }
            }),
            container(self.view_file_tree())
                .padding(PADDING_BASE)
                .height(Length::Fill),
            container(
                // 去除默认添加的临时工作区根节点所以要 -1
                text!("文件节点数  {}", self.all_file_nodes.len() - 1).size(FONT_SIZE_BASE),
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

    // 递归渲染文件树
    pub fn view_file_tree(&self) -> Element<'_, FilePanelMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        let mut workplace_view = Column::new();
        let temp_wordplace_root_node = self
            .all_file_nodes
            .get(&self.temp_workplace_root_key)
            .expect("初始化必定插入此键值对不应当出错!");
        if !temp_wordplace_root_node.children.is_empty() {
            workplace_view = workplace_view
                .push(operation::view_node(
                    self.hovered_file_node_id.clone(),
                    self.selected_file_node_id,
                    &self.all_file_nodes,
                    self.temp_workplace_root_key,
                    0,
                ))
                .spacing(SPACING);
        }
        if let Some(key) = self.workplace_root_key {
            workplace_view = workplace_view
                .push(operation::view_node(
                    self.hovered_file_node_id.clone(),
                    self.selected_file_node_id,
                    &self.all_file_nodes,
                    key,
                    0,
                ))
                .spacing(SPACING);
        }
        scrollable(workplace_view)
            .direction(scrollable::Direction::Vertical(hidden_scroller))
            .into()
    }
}
