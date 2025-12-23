use crate::{
    common::*,
    file_panel::operation::{self, FileNode, ImageFile, MdFile, NodeContent, TempDir},
};
use iced::{
    Element, Task,
    widget::{Column, image, scrollable},
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tracing::{error, info};

#[derive(Debug)]
pub struct FileTree {
    workplace_root_key: Option<u32>,
    temp_workplace_root_key: Option<u32>,
    temp_img_library_root_key: Option<u32>,
    pub all_nodes: HashMap<u32, FileNode>,
    hovered_file_node_id: Option<u32>,
    selected_node_id: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaveMode {
    AutoSave,
    ManualSave,
    SaveAs,
}

#[derive(Debug, Clone)]
pub enum FileTreeMessage {
    FetchMdFileData(PathBuf),
    FetchImgFileData(PathBuf),
    FetchFileTree(PathBuf),
    FetchImgHandle(PathBuf),
    LoadFileTree(u32, HashMap<u32, FileNode>),
    LoadImgHandle(HashMap<u32, FileNode>),
    InsertToFileTree(FileNode),
    ChangeSelectedNode(u32),
    ChangeHoveredNode(u32),
    LoadSelectedNodeData,
    SaveFile(SaveMode, FileData),
    ReturnSaveResult(Result<(), AppError>),
    SendFileDataToEditor(FileData),
    SendImgDataToPreview(Vec<ImgData>),
    UpdateStringCache(String),
    UpdateSelectedNodePath(PathBuf, FileData),
    HandleError(AppError),
    None,
}

impl FileTree {
    pub fn new() -> Self {
        let file_panel = Self {
            all_nodes: HashMap::new(),
            workplace_root_key: None,
            temp_workplace_root_key: None,
            temp_img_library_root_key: None,
            selected_node_id: None,
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
            FileTreeMessage::FetchMdFileData(path) => {
                Task::perform(operation::read_file(path), |result| match result {
                    Ok((path, text)) => {
                        let file_node = FileNode::new(
                            operation::get_file_name(&path),
                            NodeContent::Markdown(MdFile {
                                path: Some(path),
                                version: 0,
                                cache: Some(Arc::new(text)),
                            }),
                        );
                        FileTreeMessage::InsertToFileTree(file_node)
                    }
                    Err(error) => FileTreeMessage::HandleError(error),
                })
            }
            FileTreeMessage::FetchImgFileData(path) => {
                Task::perform(operation::read_img_file(path), |result| match result {
                    Ok((path, handle)) => {
                        let img_name = operation::get_file_name(&path);
                        let file_node = FileNode::new(
                            img_name,
                            NodeContent::Image(ImageFile {
                                path,
                                name: format!("图片 {}", operation::get_next_img_id()),
                                cache: handle,
                            }),
                        );
                        FileTreeMessage::InsertToFileTree(file_node)
                    }
                    Err(error) => FileTreeMessage::HandleError(error),
                })
            }
            FileTreeMessage::FetchFileTree(file_node) => Task::perform(
                operation::fetch_file_tree(file_node.clone()),
                |tree| match tree {
                    Ok((root_node_key, all_nodes)) => {
                        FileTreeMessage::LoadFileTree(root_node_key, all_nodes)
                    }
                    Err(error) => FileTreeMessage::HandleError(error),
                },
            ),
            FileTreeMessage::FetchImgHandle(file_node) => Task::perform(
                operation::fetch_img_handle(file_node),
                |result| match result {
                    Ok(img_nodes) => FileTreeMessage::LoadImgHandle(img_nodes),
                    Err(error) => FileTreeMessage::HandleError(error),
                },
            ),
            FileTreeMessage::LoadFileTree(root_node_key, all_nodes) => {
                self.workplace_root_key = Some(root_node_key);
                self.all_nodes.extend(all_nodes);
                Task::none()
            }
            FileTreeMessage::LoadImgHandle(img_nodes) => {
                let ids = img_nodes.keys().copied().collect::<Vec<u32>>();
                let img_datas = img_nodes
                    .values()
                    .into_iter()
                    .filter_map(|img_node| img_node.try_get_img().ok())
                    .map(|img_file| ImgData {
                        name: img_file.name.clone(),
                        handle: img_file.cache.clone(),
                    })
                    .collect::<Vec<ImgData>>();
                self.all_nodes.extend(img_nodes);
                self.insert_node_to_temp_img_library(ids);
                Task::done(FileTreeMessage::SendImgDataToPreview(img_datas))
            }
            FileTreeMessage::InsertToFileTree(file_node) => {
                if file_node.is_md_file() {
                    self.insert_node_to_temp_workplace(vec![file_node.id]);
                } else {
                    // 通过前置逻辑可知必定是图片无需再次判断
                    self.insert_node_to_temp_img_library(vec![file_node.id]);
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
                    if selected_file_node.is_directory() {
                        selected_file_node.reverse_expanded_if_node_is_directory();
                        return Task::none();
                    } else {
                        self.selected_node_id = Some(key);
                        return Task::done(FileTreeMessage::LoadSelectedNodeData);
                    }
                }
                Task::none()
            }
            FileTreeMessage::LoadSelectedNodeData => self
                .selected_node_id
                .and_then(|key| self.all_nodes.get_mut(&key))
                .map(|selected_node| {
                    if selected_node.is_img_file() {
                        if let Ok(img_file) = selected_node.try_get_img() {
                            let image_data = ImgData {
                                name: img_file.name.clone(),
                                handle: img_file.cache.clone(),
                            };
                            return Task::done(FileTreeMessage::SendImgDataToPreview(vec![
                                image_data,
                            ]));
                        }
                        return Task::none();
                    } else {
                        if let Ok(MdFile {
                            version,
                            cache: Some(cache),
                            ..
                        }) = selected_node.try_get_md()
                        {
                            let file_data = FileData {
                                version: *version,
                                content: Arc::clone(cache),
                            };
                            return Task::done(FileTreeMessage::SendFileDataToEditor(file_data));
                        } else {
                            if let Ok(path) = selected_node.try_get_path() {
                                return Task::perform(
                                    operation::read_file(path.to_path_buf()),
                                    |content| match content {
                                        Ok((_, content)) => {
                                            info!("文件数据读取成功!");
                                            FileTreeMessage::UpdateStringCache(content)
                                        }
                                        Err(error) => FileTreeMessage::HandleError(error),
                                    },
                                );
                            } else {
                                return Task::done(FileTreeMessage::UpdateStringCache(
                                    String::default(),
                                ));
                            }
                        }
                    }
                })
                .unwrap_or(Task::none()),
            FileTreeMessage::UpdateStringCache(content) => self
                .selected_node_id
                .and_then(|key| self.all_nodes.get_mut(&key))
                .map(|selected_node| {
                    let content = Arc::new(content);
                    if let Ok(md_file) = selected_node.try_get_md_mut() {
                        md_file.cache = Some(Arc::clone(&content));
                    }
                    let file_data = FileData {
                        version: 0,
                        content: Arc::clone(&content),
                    };
                    Task::done(FileTreeMessage::SendFileDataToEditor(file_data))
                })
                .unwrap_or(Task::none()),
            FileTreeMessage::UpdateSelectedNodePath(path, file_data) => self
                .selected_node_id
                .and_then(|key| self.all_nodes.get_mut(&key))
                .map(|selected_node| {
                    if selected_node.is_temp_file() {
                        selected_node.name = operation::get_file_name(&path);
                    }
                    if let NodeContent::Markdown(MdFile {
                        path: ref mut file_path,
                        ..
                    }) = selected_node.node_content
                    {
                        *file_path = Some(path)
                    }
                    return Task::done(FileTreeMessage::SaveFile(SaveMode::ManualSave, file_data));
                })
                .unwrap_or(Task::none()),
            FileTreeMessage::SaveFile(save_mode, file_data) => self
                .selected_node_id
                .and_then(|key| self.all_nodes.get_mut(&key))
                .and_then(|selected_file_node| {
                    let name = selected_file_node.name.clone();
                    selected_file_node
                        .try_get_md_mut()
                        .ok()
                        .map(|md_file| (md_file, name))
                })
                .map(|(md_file, name)| {
                    md_file.cache = Some(Arc::clone(&file_data.content));
                    md_file.version = file_data.version;
                    if save_mode == SaveMode::SaveAs {
                        return Task::perform(operation::save_file_dialog(name), move |path| {
                            match path {
                                Some(path) => {
                                    FileTreeMessage::UpdateSelectedNodePath(path, file_data.clone())
                                }
                                None => FileTreeMessage::ReturnSaveResult(Err(
                                    AppError::FilePanelError("文件路径为空!".to_string()),
                                )),
                            }
                        });
                    }
                    if let Some(path) = md_file.path.as_ref() {
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
                })
                .unwrap_or(Task::done(FileTreeMessage::ReturnSaveResult(Err(
                    AppError::FilePanelError("文件自动保存失败!".to_string()),
                )))),

            FileTreeMessage::HandleError(error) => {
                info!("{}", error.to_string());
                Task::none()
            }
            _ => Task::none(),
        }
    }

    // 递归渲染文件树
    pub fn view(&self) -> Element<'_, FileTreeMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        let mut workplace_view = Column::new().spacing(SPACING_SMALLER);
        if let Some(key) = self.temp_workplace_root_key {
            workplace_view = workplace_view.push(operation::view_node(
                self.hovered_file_node_id.clone(),
                self.selected_node_id,
                &self.all_nodes,
                key,
                0,
            ));
        }
        if let Some(key) = self.temp_img_library_root_key {
            workplace_view = workplace_view.push(operation::view_node(
                self.hovered_file_node_id.clone(),
                self.selected_node_id,
                &self.all_nodes,
                key,
                0,
            ));
        }
        if let Some(key) = self.workplace_root_key {
            workplace_view = workplace_view.push(operation::view_node(
                self.hovered_file_node_id.clone(),
                self.selected_node_id,
                &self.all_nodes,
                key,
                0,
            ));
        }
        scrollable(workplace_view)
            .direction(scrollable::Direction::Vertical(hidden_scroller))
            .into()
    }

    pub fn insert_node_to_temp_workplace(&mut self, ids: Vec<u32>) {
        if let Some(key) = self.temp_workplace_root_key {
            self.all_nodes.get_mut(&key).map(|root_node| {
                root_node
                    .try_get_children_mut()
                    .map(|children| {
                        for id in ids {
                            children.push(id)
                        }
                    })
                    .ok()
            });
        } else {
            let temp_workplace_root_node = FileNode::new(
                "临时工作区".to_string(),
                NodeContent::DirectoryTemp(TempDir {
                    children: ids,
                    expanded: true,
                }),
            );
            let temp_workplace_id = temp_workplace_root_node.id;
            self.all_nodes
                .insert(temp_workplace_id, temp_workplace_root_node);
            self.temp_workplace_root_key = Some(temp_workplace_id);
        }
    }

    pub fn insert_node_to_temp_img_library(&mut self, ids: Vec<u32>) {
        if let Some(key) = self.temp_img_library_root_key {
            self.all_nodes.get_mut(&key).map(|root_node| {
                root_node
                    .try_get_children_mut()
                    .map(|children| {
                        for id in ids {
                            children.push(id)
                        }
                    })
                    .ok()
            });
        } else {
            let temp_img_library_root_node = FileNode::new(
                "临时图片库".to_string(),
                NodeContent::DirectoryTemp(TempDir {
                    children: ids,
                    expanded: true,
                }),
            );
            let temp_img_library_id = temp_img_library_root_node.id;
            self.all_nodes
                .insert(temp_img_library_id, temp_img_library_root_node);
            self.temp_img_library_root_key = Some(temp_img_library_id);
        }
    }
}
