use crate::{
    common::*,
    file_panel::operation::{self, FileNode, NodeType, load_file_tree},
};
use iced::{
    Element, Task,
    widget::{Column, image, scrollable},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct FileTree {
    roo_path: Option<PathBuf>,
    workplace_root_key: Option<u32>,
    temp_workplace_root_key: u32,
    temp_img_library_root_key: u32,
    pub all_nodes: HashMap<u32, FileNode>,
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
    FetchFileTree(FileNode),
    LoadFileTree(u32, HashMap<u32, FileNode>, PathBuf),
    InsertToFileTree(FileNode),
    ChangeSelectedNode(u32),
    ChangeHoveredNode(u32),
    LoadSelectedNodeData,
    SaveFile(SaveMode, FileData),
    OpenFile(NodeType),
    ReturnSaveResult(Result<(), AppError>),
    SendFileDataToEditor(FileData),
    SendImgDataToPreview(ImgData),
    UpdateStringCache(String),
    UpdateHandleCache(image::Handle),
    UpdateSelectedNodePath(PathBuf, FileData),
    LogError(String),
    None,
}

impl FileTree {
    pub fn new() -> Self {
        let mut temp_workplace_root_node =
            FileNode::new(None, NodeType::DirectoryMd, Some("临时工作区".to_string()));
        temp_workplace_root_node.expanded = true;
        let temp_workplace_id = temp_workplace_root_node.id;
        let mut temp_img_library_root_node =
            FileNode::new(None, NodeType::DirectoryImg, Some("临时图片库".to_string()));
        temp_img_library_root_node.expanded = true;
        let temp_img_library_id = temp_img_library_root_node.id;
        let mut all_nodes = HashMap::new();
        all_nodes.insert(temp_workplace_id, temp_workplace_root_node);
        all_nodes.insert(temp_img_library_id, temp_img_library_root_node);
        let file_panel = Self {
            roo_path: None,
            all_nodes,
            workplace_root_key: None,
            temp_workplace_root_key: temp_workplace_id,
            temp_img_library_root_key: temp_img_library_id,
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
            FileTreeMessage::OpenFile(node_type) => Task::perform(
                operation::open_file(node_type),
                move |file_node| match file_node {
                    Some(file_node) => match node_type {
                        NodeType::DirectoryMd => FileTreeMessage::FetchFileTree(file_node),
                        NodeType::DirectoryImg => FileTreeMessage::InsertToFileTree(file_node),
                        _ => FileTreeMessage::InsertToFileTree(file_node),
                    },
                    None => {
                        warn!("文件路径为空!");
                        FileTreeMessage::None
                    }
                },
            ),
            FileTreeMessage::FetchFileTree(file_node) => {
                Task::perform(load_file_tree(file_node.clone()), |tree| match tree {
                    Ok((root_node_key, all_nodes)) => FileTreeMessage::LoadFileTree(
                        root_node_key,
                        all_nodes,
                        file_node.path.expect("前置逻辑必定不会出错!"),
                    ),
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
            FileTreeMessage::LoadFileTree(root_file_node_key, all_files_tree, root_path) => {
                self.workplace_root_key = Some(root_file_node_key);
                self.all_nodes.extend(all_files_tree);
                self.roo_path = Some(root_path);
                Task::none()
            }
            FileTreeMessage::InsertToFileTree(file_node) => {
                if file_node.node_type == NodeType::Markdown {
                    let temp_workplace_root_node = self
                        .all_nodes
                        .get_mut(&self.temp_workplace_root_key)
                        .expect("初始化必定插入不应当出错!");
                    temp_workplace_root_node.children.push(file_node.id);
                } else {
                    // 通过前置逻辑可知必定是图片无需再次判断
                    let temp_img_library_root_node = self
                        .all_nodes
                        .get_mut(&self.temp_img_library_root_key)
                        .expect("初始化必定插入不应当出错!");
                    temp_img_library_root_node.children.push(file_node.id);
                };
                let file_node_id = file_node.id;
                self.all_nodes.insert(file_node.id, file_node);
                Task::done(FileTreeMessage::ChangeSelectedNode(file_node_id))
            }
            FileTreeMessage::ChangeHoveredNode(id) => {
                self.hovered_file_node_id = Some(id);
                Task::none()
            }
            FileTreeMessage::ChangeSelectedNode(key) => {
                if let Some(selected_file_node) = self.all_nodes.get_mut(&key) {
                    match selected_file_node.node_type {
                        NodeType::DirectoryMd | NodeType::DirectoryImg => {
                            selected_file_node.expanded = !selected_file_node.expanded;
                            return Task::none();
                        }
                        _ => {
                            self.selected_file_node_id = Some(key);
                            return Task::done(FileTreeMessage::LoadSelectedNodeData);
                        }
                    }
                }
                Task::none()
            }
            FileTreeMessage::LoadSelectedNodeData => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_node) = self.all_nodes.get_mut(&key) {
                        match selected_node.node_type {
                            NodeType::Image => {
                                if let Some(ref handle) = selected_node.handle_cache {
                                    let image_data = ImgData {
                                        name: selected_node.file_name.clone(),
                                        handle: handle.clone(),
                                    };
                                    return Task::done(FileTreeMessage::SendImgDataToPreview(
                                        image_data,
                                    ));
                                } else {
                                    if let Some(ref path) = selected_node.path {
                                        return Task::perform(
                                            operation::get_img_handle(path.clone()),
                                            |result| match result {
                                                Ok(handle) => {
                                                    FileTreeMessage::UpdateHandleCache(handle)
                                                }
                                                Err(error) => {
                                                    FileTreeMessage::LogError(error.to_string())
                                                }
                                            },
                                        );
                                    }
                                }
                                return Task::none();
                            }
                            NodeType::Markdown => {
                                if let Some(ref cache) = selected_node.string_cache {
                                    let file_data = FileData {
                                        version: selected_node.version,
                                        content: Arc::clone(cache),
                                    };
                                    return Task::done(FileTreeMessage::SendFileDataToEditor(
                                        file_data,
                                    ));
                                } else {
                                    if let Some(ref path) = selected_node.path {
                                        return Task::perform(
                                            operation::read_file(path.clone()),
                                            |content| match content {
                                                Ok(content) => {
                                                    info!("文件数据读取成功!");
                                                    FileTreeMessage::UpdateStringCache(content)
                                                }
                                                Err(error) => {
                                                    FileTreeMessage::LogError(error.to_string())
                                                }
                                            },
                                        );
                                    } else {
                                        return Task::done(FileTreeMessage::UpdateStringCache(
                                            String::default(),
                                        ));
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
                Task::none()
            }
            FileTreeMessage::UpdateStringCache(content) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_node) = self.all_nodes.get_mut(&key) {
                        let content = Arc::new(content);
                        selected_node.string_cache = Some(Arc::clone(&content));
                        let file_data = FileData {
                            version: 0,
                            content: Arc::clone(&content),
                        };
                        return Task::done(FileTreeMessage::SendFileDataToEditor(file_data));
                    }
                }
                Task::none()
            }
            FileTreeMessage::UpdateHandleCache(handle) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_node) = self.all_nodes.get_mut(&key) {
                        selected_node.handle_cache = Some(handle.clone());
                        let image_data = ImgData {
                            name: selected_node.file_name.clone(),
                            handle,
                        };
                        return Task::done(FileTreeMessage::SendImgDataToPreview(image_data));
                    }
                }
                Task::none()
            }
            FileTreeMessage::UpdateSelectedNodePath(path, file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_data) = self.all_nodes.get_mut(&key) {
                        if selected_file_data.path.is_none() {
                            selected_file_data.file_name =
                                path.file_name().unwrap().to_string_lossy().into();
                        }
                        selected_file_data.path = Some(path);
                        return Task::done(FileTreeMessage::SaveFile(
                            SaveMode::ManualSave,
                            file_data,
                        ));
                    }
                }
                Task::none()
            }
            FileTreeMessage::SaveFile(save_mode, file_data) => {
                if let Some(key) = self.selected_file_node_id {
                    if let Some(selected_file_node) = self.all_nodes.get_mut(&key) {
                        selected_file_node.string_cache = Some(Arc::clone(&file_data.content));
                        selected_file_node.version = file_data.version;
                        if save_mode == SaveMode::SaveAs {
                            return Task::perform(
                                operation::save_file_dialog(selected_file_node.file_name.clone()),
                                move |path| match path {
                                    Some(path) => FileTreeMessage::UpdateSelectedNodePath(
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
            .all_nodes
            .get(&self.temp_workplace_root_key)
            .expect("初始化必定插入此键值对不应当出错!");
        let temp_img_library_root_node = self
            .all_nodes
            .get(&self.temp_img_library_root_key)
            .expect("初始化必定插入此键值对不应当出错!");
        if !temp_wordplace_root_node.children.is_empty() {
            workplace_view = workplace_view
                .push(operation::view_node(
                    self.hovered_file_node_id.clone(),
                    self.selected_file_node_id,
                    &self.all_nodes,
                    self.temp_workplace_root_key,
                    0,
                ))
                .spacing(SPACING);
        }
        if !temp_img_library_root_node.children.is_empty() {
            workplace_view = workplace_view
                .push(operation::view_node(
                    self.hovered_file_node_id.clone(),
                    self.selected_file_node_id,
                    &self.all_nodes,
                    self.temp_img_library_root_key,
                    0,
                ))
                .spacing(SPACING);
        }
        if let Some(key) = self.workplace_root_key {
            workplace_view = workplace_view
                .push(operation::view_node(
                    self.hovered_file_node_id.clone(),
                    self.selected_file_node_id,
                    &self.all_nodes,
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
