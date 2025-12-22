use crate::{
    common::*,
    file_panel::operation::{self, FileNode, load_file_tree},
};
use iced::{
    Element, Task,
    widget::{Column, scrollable},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tracing::{error, info};

#[derive(Debug)]
pub struct FileTree {
    roo_path: Option<PathBuf>,
    workplace_root_key: Option<u32>,
    temp_workplace_root_key: u32,
    pub all_file_nodes: HashMap<u32, FileNode>,
    hovered_file_node_id: Option<u32>,
    selected_file_node_id: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaveMode {
    AutoSave,
    ManualSave,
    SaveAs,
}

#[derive(Debug, Clone)]
pub enum FileTreeMessage {
    FetchFileTree(PathBuf),
    LoadWorkplace(u32, HashMap<u32, FileNode>, PathBuf),
    InsertToTempWorkplace(FileNode),
    ChangeSelectedFileNode(u32),
    ChangeHoveredFileNode(u32),

    InsertNewFile(u32, FileNode),
    SaveFile(SaveMode, FileData),
    ReturnSaveResult(Result<(), AppError>),

    SendFileDataToEditor(FileData),

    RenameFileNodeFromEditor(String),
    UpdateSelectedFileNodeCache(FileData),
    UpdateSelectedFilePathAndSave(PathBuf, FileData),
    LogError(String),

    None,
}

impl FileTree {
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
        file_tree_message: FileTreeMessage,
        setting: &AppSetting,
    ) -> Task<FileTreeMessage> {
        match file_tree_message {
            FileTreeMessage::FetchFileTree(path) => {
                Task::perform(load_file_tree(path.clone()), |tree| match tree {
                    Ok((root_file_node_key, all_files_tree)) => {
                        FileTreeMessage::LoadWorkplace(root_file_node_key, all_files_tree, path)
                    }
                    Err(error) => FileTreeMessage::LogError(error.to_string()),
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
            FileTreeMessage::LoadWorkplace(root_file_node_key, all_files_tree, root_path) => {
                self.workplace_root_key = Some(root_file_node_key);
                self.all_file_nodes.extend(all_files_tree);
                self.roo_path = Some(root_path);
                Task::none()
            }
            FileTreeMessage::InsertNewFile(new_file_id, new_file) => {
                self.all_file_nodes.insert(new_file_id, new_file);
                let temp_workplace_root_node = self
                    .all_file_nodes
                    .get_mut(&self.temp_workplace_root_key)
                    .expect("初始化必定插入此键值对不应当出错!");
                temp_workplace_root_node.children.push(new_file_id);
                Task::done(FileTreeMessage::ChangeSelectedFileNode(new_file_id))
            }
            FileTreeMessage::InsertToTempWorkplace(file_node) => {
                let temp_workplace_root_node = self
                    .all_file_nodes
                    .get_mut(&self.temp_workplace_root_key)
                    .unwrap();
                let file_node_id = file_node.id;
                temp_workplace_root_node.children.push(file_node.id);
                self.all_file_nodes.insert(file_node.id, file_node);
                Task::done(FileTreeMessage::ChangeSelectedFileNode(file_node_id))
            }
            FileTreeMessage::ChangeHoveredFileNode(id) => {
                self.hovered_file_node_id = Some(id);
                Task::none()
            }
            FileTreeMessage::ChangeSelectedFileNode(key) => {
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
                        return Task::done(FileTreeMessage::SendFileDataToEditor(
                            file_data,
                        ));
                    } else {
                        if let Some(ref path) = node.path {
                            return Task::perform(
                                operation::read_file(path.clone()),
                                |file_data| match file_data {
                                    Ok(file_data) => {
                                        info!("文件数据读取成功!");
                                        FileTreeMessage::UpdateSelectedFileNodeCache(file_data)
                                    }
                                    Err(error) => FileTreeMessage::LogError(error.to_string()),
                                },
                            );
                        } else {
                            let file_data = FileData {
                                version: 0,
                                content: Arc::new(String::new()),
                            };
                            return Task::done(FileTreeMessage::UpdateSelectedFileNodeCache(
                                file_data,
                            ));
                        }
                    }
                }
                Task::none()
            }
            FileTreeMessage::UpdateSelectedFileNodeCache(file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_file_nodes.get_mut(&key) {
                        selected_file_data.content_cache = Some(Arc::clone(&file_data.content));
                        return Task::done(FileTreeMessage::SendFileDataToEditor(
                            file_data,
                        ));
                    }
                }
                Task::none()
            }
            FileTreeMessage::UpdateSelectedFilePathAndSave(path, file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_file_nodes.get_mut(&key) {
                        if selected_file_data.path.is_none() {
                            selected_file_data.file_name =
                                path.file_name().unwrap().to_string_lossy().into();
                            selected_file_data.path = Some(path);
                            return Task::done(FileTreeMessage::SaveFile(
                                SaveMode::ManualSave,
                                file_data,
                            ));
                        } else {
                            selected_file_data.path = Some(path);
                            return Task::done(FileTreeMessage::SaveFile(
                                SaveMode::ManualSave,
                                file_data,
                            ))
                            .chain(Task::done(
                                FileTreeMessage::FetchFileTree(
                                    self.roo_path.as_ref().expect("不应当为空!").clone(),
                                ),
                            ));
                        }
                    }
                }
                Task::none()
            }
            FileTreeMessage::SaveFile(save_mode, file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_node) = self.all_file_nodes.get_mut(&key) {
                        selected_file_node.content_cache = Some(Arc::clone(&file_data.content));
                        selected_file_node.version = file_data.version;
                        if save_mode == SaveMode::SaveAs {
                            return Task::perform(
                                operation::save_file_dialog(selected_file_node.file_name.clone()),
                                move |path| match path {
                                    Some(path) => FileTreeMessage::UpdateSelectedFilePathAndSave(
                                        path,
                                        file_data.clone(),
                                    ),
                                    None => FileTreeMessage::ReturnSaveResult(Err(
                                        AppError::FilePanelError("文件路径为空!".to_string()),
                                    )),
                                },
                            );
                        }
                        if let Some(path) = selected_file_node.path.as_ref() {
                            return Task::perform(
                                operation::save_file(path.clone(), file_data.content),
                                |result| match result {
                                    Ok(_) => {
                                        info!("文件保存成功!");
                                        FileTreeMessage::ReturnSaveResult(Ok(()))
                                    }
                                    Err(error) => FileTreeMessage::ReturnSaveResult(Err(error)),
                                },
                            );
                        } else {
                            if save_mode == SaveMode::AutoSave {
                                info!("文件保存缓冲区成功!");
                                return Task::done(FileTreeMessage::ReturnSaveResult(Ok(())));
                            } else {
                                return Task::done(FileTreeMessage::SaveFile(
                                    SaveMode::SaveAs,
                                    file_data,
                                ));
                            }
                        }
                    };
                }
                Task::done(FileTreeMessage::ReturnSaveResult(Err(
                    AppError::FilePanelError("文件自动保存失败!".to_string()),
                )))
            }

            FileTreeMessage::LogError(error_info) => {
                error!(error_info);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    // 递归渲染文件树
    pub fn view(&self) -> Element<'_, FileTreeMessage> {
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
