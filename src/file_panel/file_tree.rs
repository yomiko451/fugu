use crate::{
    common::*,
    file_panel::operation::{self, FileNode, ImageFile, IsAutoSave, MdFile, NodeContent, TempDir},
};
use iced::{
    Element, Task,
    widget::{Column, scrollable},
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

#[derive(Debug, Clone)]
pub enum FileTreeMessage {
    FetchMdFileData(PathBuf),
    FetchImgFileData(PathBuf),
    FetchFileTree(PathBuf),
    FetchImgHandles(PathBuf),
    LoadFileTree(u32, HashMap<u32, FileNode>),
    LoadImgHandles(HashMap<u32, FileNode>),
    InsertToFileTree(FileNode),
    ChangeSelectedNode(u32),
    ChangeHoveredNode(u32),
    LoadSelectedNodeData,
    SaveFile(PathBuf, Arc<String>),
    SaveAs(FileData),
    ReturnSaveResult(Result<(), AppError>),
    SendFileDataToEditor(FileData),
    SendImgCodeToEditor(String),
    SendImgDataToPreview(Vec<ImgData>),
    SendImgBasePathToPreview(PathBuf),
    CreateMdCache(String, u32),
    UpdateNodePath(PathBuf, FileData),
    UpdateNodeInfo(IsAutoSave, FileData),
    HandleError(AppError),
    CopyImgFileData(u32),
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
                                indep_id: operation::get_next_img_id(),
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
            FileTreeMessage::FetchImgHandles(file_node) => Task::perform(
                operation::fetch_img_handle(file_node),
                |result| match result {
                    Ok(img_nodes) => FileTreeMessage::LoadImgHandles(img_nodes),
                    Err(error) => FileTreeMessage::HandleError(error),
                },
            ),
            FileTreeMessage::LoadFileTree(root_node_key, all_nodes) => {
                self.workplace_root_key = Some(root_node_key);
                self.all_nodes.extend(all_nodes);
                Task::none()
            }
            FileTreeMessage::LoadImgHandles(img_nodes) => {
                let ids = img_nodes.keys().copied().collect::<Vec<u32>>();
                let img_datas = img_nodes
                    .values()
                    .into_iter()
                    .filter_map(|img_node| {
                        img_node
                            .try_get_img()
                            .ok()
                            .map(|img_file| (img_file, img_node.global_id))
                    })
                    .map(|(img_file, id)| ImgData {
                        global_id: id,
                        indep_id: img_file.indep_id.clone(),
                        handle: img_file.cache.clone(),
                    })
                    .collect::<Vec<ImgData>>();
                self.all_nodes.extend(img_nodes);
                self.insert_node_to_temp_img_library(ids);
                Task::done(FileTreeMessage::SendImgDataToPreview(img_datas))
            }
            FileTreeMessage::InsertToFileTree(file_node) => {
                if file_node.is_md_file() {
                    self.insert_node_to_temp_workplace(vec![file_node.global_id]);
                } else {
                    // 通过前置逻辑可知必定是图片无需再次判断
                    self.insert_node_to_temp_img_library(vec![file_node.global_id]);
                };
                let file_node_id = file_node.global_id;
                self.all_nodes.insert(file_node.global_id, file_node);
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
                } else {
                    Task::done(FileTreeMessage::HandleError(AppError::FilePanelError(
                        "[FileTree-ChangeSelectedNode]:找不到指定id的节点!".to_string(),
                    )))
                }
            }
            FileTreeMessage::LoadSelectedNodeData => {
                if let Some(node) = self
                    .selected_node_id
                    .and_then(|key| self.all_nodes.get(&key))
                {
                    let id = node.global_id;
                    if node.is_img_file() {
                        if let Ok(img_file) = node.try_get_img() {
                            let image_data = ImgData {
                                global_id: id,
                                indep_id: img_file.indep_id,
                                handle: img_file.cache.clone(),
                            };
                            return Task::done(FileTreeMessage::SendImgDataToPreview(vec![
                                image_data,
                            ]));
                        }
                    } else {
                        if let Ok(MdFile {
                            version,
                            cache: Some(cache),
                            path: Some(path),
                        }) = node.try_get_md()
                        {
                            let file_data = FileData {
                                global_id: id,
                                version: *version,
                                content: Arc::clone(cache),
                            };
                            return Task::done(FileTreeMessage::SendFileDataToEditor(file_data))
                                .chain(Task::done(FileTreeMessage::SendImgBasePathToPreview(
                                    path.clone(),
                                )));
                        } else {
                            if let Ok(path) = node.try_get_path() {
                                return Task::perform(
                                    operation::read_file(path.to_path_buf()),
                                    move |content| match content {
                                        Ok((_, content)) => {
                                            info!("文件数据读取成功!");
                                            FileTreeMessage::CreateMdCache(content, id)
                                        }
                                        Err(error) => FileTreeMessage::HandleError(error),
                                    },
                                );
                            } else {
                                if let Ok(MdFile {
                                    version,
                                    path: None,
                                    ..
                                }) = node.try_get_md()
                                {
                                    let new_file_data = FileData {
                                        global_id: node.global_id,
                                        version: *version,
                                        content: Arc::new(String::default()),
                                    };
                                    return Task::done(FileTreeMessage::SendFileDataToEditor(
                                        new_file_data,
                                    ));
                                }
                            }
                        }
                    }
                };
                Task::done(FileTreeMessage::HandleError(AppError::FilePanelError(
                    "[FileTree-LoadSelectedNodeData]:载入指定id节点内容失败!".to_string(),
                )))
            }
            // 加载内容后，发送内容到editor前，预留缓存省去二次加载
            FileTreeMessage::CreateMdCache(content, id) => self
                .all_nodes
                .get_mut(&id)
                .and_then(|node| node.try_get_md_mut().ok())
                .map(|md_file| {
                    let content = Arc::new(content);
                    md_file.cache = Some(Arc::clone(&content));
                    let file_data = FileData {
                        global_id: id,
                        version: 0,
                        content: Arc::clone(&content),
                    };
                    Task::done(FileTreeMessage::SendFileDataToEditor(file_data)).chain(Task::done(
                        FileTreeMessage::SendImgBasePathToPreview(
                            md_file
                                .path
                                .as_ref()
                                .expect("前置逻辑必定合法路径不会出错!")
                                .clone(),
                        ),
                    ))
                })
                .unwrap_or(Task::done(FileTreeMessage::HandleError(
                    AppError::FilePanelError(
                        "[FileTree-UpdateStringCache]:更新md类型节点内容缓存失败!".to_string(),
                    ),
                ))),
            // 保存前先更新节点数据
            FileTreeMessage::UpdateNodeInfo(is_auto_save, file_data) => self
                .all_nodes
                .get_mut(&file_data.global_id)
                .and_then(|node| node.try_get_md_mut().ok())
                .map(|md_file| {
                    md_file.version = file_data.version;
                    md_file.cache = Some(file_data.content.clone());
                    if let Some(ref path) = md_file.path {
                        Task::done(FileTreeMessage::SaveFile(path.clone(), file_data.content))
                    } else if !is_auto_save.0 {
                        Task::done(FileTreeMessage::SaveAs(file_data))
                    } else {
                        info!("[FileTreeMessage::UpdateNodeInfo]:文件保存缓冲区成功!");
                        Task::done(FileTreeMessage::ReturnSaveResult(Ok(())))
                    }
                })
                .unwrap_or(Task::done(FileTreeMessage::HandleError(
                    AppError::FilePanelError(
                        "[FileTree-UpdateNodeInfo]:更新节点内容失败!".to_string(),
                    ),
                ))),
            FileTreeMessage::UpdateNodePath(path, mut file_data) => self
                .all_nodes
                .get_mut(&file_data.global_id)
                .map(|node| {
                    node.name = operation::get_file_name(&path);
                    if let NodeContent::Markdown(MdFile {
                        path: ref mut file_path,
                        ..
                    }) = node.node_content
                    {
                        *file_path = Some(path.clone())
                    }
                    Task::done(FileTreeMessage::SaveFile(path, file_data.content.clone()))
                        .chain(Task::done(FileTreeMessage::SendFileDataToEditor(file_data)))
                })
                .unwrap_or(Task::done(FileTreeMessage::HandleError(
                    AppError::FilePanelError(
                        "[FileTree-UpdateNodePath]:更新节点path字段失败!".to_string(),
                    ),
                ))),
            FileTreeMessage::SaveAs(file_data) => self
                .all_nodes
                .get(&file_data.global_id)
                .map(|node| {
                    let is_tmp_file = node.is_temp_file();
                    Task::perform(
                        operation::save_file_dialog(node.name.clone()),
                        move |result| match result {
                            Some(path) => {
                                if is_tmp_file {
                                    FileTreeMessage::UpdateNodePath(path, file_data)
                                } else {
                                    FileTreeMessage::SaveFile(path, file_data.content)
                                }
                            }
                            None => FileTreeMessage::HandleError(AppError::FilePanelError(
                                "[FileTree-SaveAs]:获取路径失败!".to_string(),
                            )),
                        },
                    )
                })
                .unwrap_or(Task::done(FileTreeMessage::HandleError(
                    AppError::FilePanelError("[FileTree-SaveAs]:获取节点名称失败!".to_string()),
                ))),
            FileTreeMessage::SaveFile(path, content) => {
                Task::perform(operation::save_file(path, content), |result| match result {
                    Ok(_) => {
                        info!("[FileTree-SaveFile]:文件保存成功!");
                        FileTreeMessage::ReturnSaveResult(Ok(()))
                    }
                    Err(error) => FileTreeMessage::ReturnSaveResult(Err(error)),
                })
            }
            FileTreeMessage::HandleError(error) => {
                info!("{}", error.to_string());
                Task::none()
            }
            FileTreeMessage::CopyImgFileData(id) => {
                if let Some(md_node) = self
                    .selected_node_id
                    .and_then(|selected_id| self.all_nodes.get(&selected_id))
                {
                    if md_node.is_temp_file() {
                        // 新文件没有路径必须保存一次确定路径才能插入图片
                        // 弹窗询问用户如何操作
                        // TODO
                        
                    } else {
                        if let Some((md_path, img_path)) =
                            md_node.try_get_path().ok().and_then(|md_path| {
                                self.all_nodes
                                    .get(&id)
                                    .and_then(|img_node| img_node.try_get_path().ok())
                                    .map(|img_path| (md_path, img_path))
                            })
                        {
                            return Task::perform(
                                operation::copy_img_to_folder(
                                    md_path.to_owned(),
                                    img_path.to_owned(),
                                ),
                                |result| match result {
                                    Ok(code) => FileTreeMessage::SendImgCodeToEditor(code),
                                    Err(error) => FileTreeMessage::HandleError(error),
                                },
                            );
                        }
                    }
                }
                Task::done(FileTreeMessage::HandleError(AppError::FilePanelError(
                    "[FileTree-CopyImgFileData]:图片数据复制失败!".to_string(),
                )))
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
        match self.temp_workplace_root_key {
            Some(key) => {
                if let Some(children) = self
                    .all_nodes
                    .get_mut(&key)
                    .and_then(|root_node| root_node.try_get_children_mut().ok())
                {
                    for id in ids {
                        children.push(id)
                    }
                }
            }
            None => {
                let temp_workplace_root_node = FileNode::new(
                    "临时工作区".to_string(),
                    NodeContent::DirectoryTemp(TempDir {
                        children: ids,
                        expanded: true,
                    }),
                );
                let temp_workplace_id = temp_workplace_root_node.global_id;
                self.all_nodes
                    .insert(temp_workplace_id, temp_workplace_root_node);
                self.temp_workplace_root_key = Some(temp_workplace_id);
            }
        }
    }

    pub fn insert_node_to_temp_img_library(&mut self, ids: Vec<u32>) {
        match self.temp_img_library_root_key {
            Some(key) => {
                if let Some(children) = self
                    .all_nodes
                    .get_mut(&key)
                    .and_then(|root_node| root_node.try_get_children_mut().ok())
                {
                    for id in ids {
                        children.push(id)
                    }
                }
            }
            None => {
                let temp_img_library_root_node = FileNode::new(
                    "临时图片库".to_string(),
                    NodeContent::DirectoryTemp(TempDir {
                        children: ids,
                        expanded: true,
                    }),
                );
                let temp_img_library_id = temp_img_library_root_node.global_id;
                self.all_nodes
                    .insert(temp_img_library_id, temp_img_library_root_node);
                self.temp_img_library_root_key = Some(temp_img_library_id);
            }
        }
    }
}
