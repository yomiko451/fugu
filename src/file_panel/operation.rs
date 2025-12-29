use crate::{common::*, file_panel::file_tree::FileTreeMessage};
use iced::{
    Background, Color, Length, Padding, Theme, mouse,
    widget::{Column, Row, column, container, image, mouse_area, text},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};
use tokio::io::AsyncWriteExt;

static FILE_NODE_COUNTER: AtomicU32 = AtomicU32::new(0);
static IMG_NAME_COUNTER: AtomicU32 = AtomicU32::new(1);

pub fn get_next_id() -> u32 {
    FILE_NODE_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn get_next_img_id() -> u32 {
    IMG_NAME_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub fn get_file_name(path: &Path) -> String {
    match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => "暂无名称".to_string(),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IsAutoSave(pub bool);

#[derive(Debug, Clone)]
pub struct FileNode {
    pub global_id: u32,
    pub name: String,
    pub node_content: NodeContent,
}

impl FileNode {
    pub fn new(name: String, node_content: NodeContent) -> Self {
        FileNode {
            global_id: get_next_id(),
            name,
            node_content,
        }
    }

    pub fn try_get_children_mut(&mut self) -> Result<&mut Vec<u32>, AppError> {
        match self {
            FileNode {
                node_content: NodeContent::DirectoryMd(Dir { children, .. }),
                ..
            } => Ok(children),
            FileNode {
                node_content: NodeContent::DirectoryTemp(TempDir { children, .. }),
                ..
            } => Ok(children),
            _ => Err(AppError::FilePanelError(
                "该节点不是文件夹类型!".to_string(),
            )),
        }
    }

    pub fn try_get_children(&self) -> Result<&Vec<u32>, AppError> {
        match self {
            FileNode {
                node_content: NodeContent::DirectoryMd(Dir { children, .. }),
                ..
            } => Ok(children),
            FileNode {
                node_content: NodeContent::DirectoryTemp(TempDir { children, .. }),
                ..
            } => Ok(children),
            _ => Err(AppError::FilePanelError("该节点不是文件夹!".to_string())),
        }
    }
    
    pub fn is_children_empty(&self) -> bool {
        match self {
            FileNode {
                node_content: NodeContent::DirectoryMd(Dir {children,..}),
                ..
            } => {
                children.is_empty()
            }
            FileNode {
                node_content: NodeContent::DirectoryTemp(TempDir {children,..}),
                ..
            } => {
                children.is_empty()
            }
            _ => true
        }
    }
    
    pub fn try_get_img(&self) -> Result<&ImageFile, AppError> {
        match self {
            FileNode {
                node_content: NodeContent::Image(image),
                ..
            } => Ok(image),
            _ => Err(AppError::FilePanelError("该节点不是图片!".to_string())),
        }
    }

    pub fn try_get_md(&self) -> Result<&MdFile, AppError> {
        match self {
            FileNode {
                node_content: NodeContent::Markdown(md_file),
                ..
            } => Ok(md_file),
            _ => Err(AppError::FilePanelError("该节点不是md文件!".to_string())),
        }
    }

    pub fn try_get_md_mut(&mut self) -> Result<&mut MdFile, AppError> {
        match self {
            FileNode {
                node_content: NodeContent::Markdown(md_file),
                ..
            } => Ok(md_file),
            _ => Err(AppError::FilePanelError("该节点不是md文件!".to_string())),
        }
    }

    pub fn try_get_path(&self) -> Result<&Path, AppError> {
        match self {
            FileNode {
                node_content: NodeContent::Image(ImageFile { path, .. }),
                ..
            } => Ok(path),
            FileNode {
                node_content: NodeContent::DirectoryMd(Dir { path, .. }),
                ..
            } => Ok(path),
            FileNode {
                node_content: NodeContent::Markdown(MdFile { path, .. }),
                ..
            } => match path {
                Some(path) => Ok(path),
                None => Err(AppError::FilePanelError("路径为空!".to_string())),
            },
            _ => Err(AppError::FilePanelError("临时节点无路径!".to_string())),
        }
    }

    pub fn is_expanded(&self) -> bool {
        match self {
            FileNode {
                node_content: NodeContent::DirectoryMd(Dir { expanded, .. }),
                ..
            } => *expanded,
            FileNode {
                node_content: NodeContent::DirectoryTemp(TempDir { expanded, .. }),
                ..
            } => *expanded,
            _ => false,
        }
    }

    pub fn reverse_expanded_if_node_is_directory(&mut self) {
        match self {
            FileNode {
                node_content: NodeContent::DirectoryMd(Dir { expanded, .. }),
                ..
            } => *expanded = !*expanded,
            FileNode {
                node_content: NodeContent::DirectoryTemp(TempDir { expanded, .. }),
                ..
            } => *expanded = !*expanded,
            _ => {}
        }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            FileNode {
                node_content: NodeContent::DirectoryMd { .. },
                ..
            } => true,
            FileNode {
                node_content: NodeContent::DirectoryTemp { .. },
                ..
            } => true,
            _ => false,
        }
    }

    pub fn is_temp_file(&self) -> bool {
        match self {
            FileNode {
                node_content: NodeContent::Markdown(MdFile { path, .. }),
                ..
            } => path.is_none(),
            _ => false,
        }
    }

    pub fn is_md_file(&self) -> bool {
        match self {
            FileNode {
                node_content: NodeContent::Markdown { .. },
                ..
            } => true,
            _ => false,
        }
    }

    pub fn is_img_file(&self) -> bool {
        match self {
            FileNode {
                node_content: NodeContent::Image { .. },
                ..
            } => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dir {
    pub path: PathBuf,
    pub children: Vec<u32>,
    pub expanded: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TempDir {
    pub children: Vec<u32>,
    pub expanded: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct MdFile {
    pub path: Option<PathBuf>,
    pub version: u64,
    pub cache: Option<Arc<String>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ImageFile {
    pub path: PathBuf,
    pub indep_id: u32,
    pub cache: image::Handle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeContent {
    DirectoryMd(Dir),
    DirectoryTemp(TempDir),
    Markdown(MdFile),
    Image(ImageFile),
}

pub async fn open_md_file_dialog() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("打开文件")
        .add_filter("markdown文件(*md)", &["md"])
        .pick_file()
        .await?;

    Some(path.path().to_path_buf())
}

pub async fn open_img_file_dialog() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("打开文件")
        .add_filter("图片文件(*jpg,*png)", &["jpg", "png"])
        .pick_file()
        .await?;

    Some(path.path().to_path_buf())
}

pub async fn open_folder_dialog() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("打开文件夹")
        .pick_folder()
        .await?;

    Some(path.path().to_path_buf())
}

pub async fn open_img_folder_dialog() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("导入图片文件夹")
        .pick_folder()
        .await?;

    Some(path.path().to_path_buf())
}

pub async fn save_file_dialog(file_name: String) -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .add_filter("markdown文件(*md)", &["md"])
        .set_file_name(file_name)
        .set_title("保存文件")
        .save_file()
        .await
        .map(|file_handle| file_handle.path().to_path_buf())
}

pub async fn read_file(path: PathBuf) -> Result<(PathBuf, String), AppError> {
    let content = tokio::fs::read_to_string(&path).await?;
    Ok((path, content))
}

pub async fn read_img_file(path: PathBuf) -> Result<(PathBuf, image::Handle), AppError> {
    let bytes = tokio::fs::read(&path).await?;
    let handle = image::Handle::from_bytes(bytes);
    Ok((path, handle))
}

pub async fn save_file(path: PathBuf, content: Arc<String>) -> Result<(), AppError> {
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

// 异步读取文件并生成节点树
pub async fn fetch_file_tree(
    root_path: PathBuf,
) -> Result<(u32, HashMap<u32, FileNode>), AppError> {
    let mut all_nodes = HashMap::new();
    let root_node_name = get_file_name(&root_path);
    let root_node_content = NodeContent::DirectoryMd(Dir {
        path: root_path.clone(),
        children: vec![],
        expanded: true,
    });
    let root_node = FileNode::new(root_node_name, root_node_content);
    let root_node_key = root_node.global_id;
    let mut node_index_stack = vec![(root_path, root_node.global_id)];
    all_nodes.insert(root_node.global_id, root_node);

    while let Some((path, key)) = node_index_stack.pop() {
        let mut dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                if let Some(extension) = path.extension()
                    && extension.to_string_lossy().as_ref() == "md"
                {
                    let child_node_name = get_file_name(&path);
                    let child_node_content = NodeContent::Markdown(MdFile {
                        path: Some(path),
                        version: 0,
                        cache: None,
                    });

                    let child_node = FileNode::new(child_node_name, child_node_content);
                    if let Some(file_node) = all_nodes.get_mut(&key) {
                        file_node.try_get_children_mut()?.push(child_node.global_id);
                    }
                    all_nodes.insert(child_node.global_id, child_node);
                } else {
                    continue;
                }
            } else {
                let child_node_name = get_file_name(&path);
                let child_node_content = NodeContent::DirectoryMd(Dir {
                    path: path.clone(),
                    children: vec![],
                    expanded: false,
                });

                let child_node = FileNode::new(child_node_name, child_node_content);
                node_index_stack.push((path, child_node.global_id));
                if let Some(file_node) = all_nodes.get_mut(&key) {
                    file_node.try_get_children_mut()?.push(child_node.global_id);
                }
                all_nodes.insert(child_node.global_id, child_node);
            }
        }
    }
    Ok((root_node_key, all_nodes))
}

// 异步读取图片并生成节点列表
pub async fn fetch_img_handle(root_path: PathBuf) -> Result<HashMap<u32, FileNode>, AppError> {
    let mut path_stack = vec![root_path];
    let mut img_nodes = HashMap::new();
    while let Some(path) = path_stack.pop() {
        let mut dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();

            if !path.is_dir() {
                if let Some(extension) = path.extension()
                    && ["jpg", "png"].contains(&extension.to_string_lossy().as_ref())
                {
                    let (_, handle) = read_img_file(path.clone()).await?;
                    let img_node_name = get_file_name(&path);
                    let img_node_content = NodeContent::Image(ImageFile {
                        path,
                        indep_id: get_next_img_id(),
                        cache: handle,
                    });
                    let file_node = FileNode::new(img_node_name, img_node_content);
                    img_nodes.insert(file_node.global_id, file_node);
                }
            } else {
                path_stack.push(path);
            }
        }
    }

    Ok(img_nodes)
}

pub async fn copy_img_to_folder(md_path: PathBuf, img_path: PathBuf) -> Result<String, AppError> {
    let parent_path = md_path.parent().expect("必定合法路径不应当出错!");
    let img_folder_path = parent_path.join("fugu-images");
    let img_file_name = img_path.file_name().expect("必定合法路径不应当出错!");
    let new_img_path = img_folder_path.join(img_file_name);
    if tokio::fs::try_exists(&img_folder_path).await? {
        if !tokio::fs::try_exists(&new_img_path).await? {
            tokio::fs::copy(&img_path, &new_img_path).await?;
        }
    } else {
        tokio::fs::create_dir(img_folder_path).await?;
        tokio::fs::copy(&img_path, &new_img_path).await?;
    }
    let code = format!("![](fugu-images/{})\n\n", img_file_name.to_string_lossy());
    Ok(code)
}

// 递归渲染节点树
pub fn view_node(
    hovered_id: Option<u32>,
    selected_id: Option<u32>,
    all_file_nodes: &HashMap<u32, FileNode>,
    key: u32,
    depth: u16,
) -> Column<'_, FileTreeMessage> {
    let node = all_file_nodes.get(&key).expect("前置逻辑不会出错!");
    let mut row = Row::new();
    let children_node_view = if let Ok(children) = node.try_get_children() && !children.is_empty() 
    {
        let mut column = Column::new().height(match node.is_expanded() {
            false => {
                row = row.push(text(" ▶ ").size(FONT_SIZE_SMALLER));
                0.into()
            }
            true => {
                row = row.push(text(" ▼ ").size(FONT_SIZE_SMALLER));
                Length::Shrink
            }
        });
        if let Ok(children) = node.try_get_children() {
            for child_key in children {
                column = column.push(view_node(
                    hovered_id,
                    selected_id,
                    all_file_nodes,
                    *child_key,
                    depth + 1,
                ));
            }
        }
        Some(column)
    } else {
        if node.is_directory() {
            row = row.push(text(" ▷ ").size(FONT_SIZE_SMALLER));
        }
        if node.is_temp_file() {
            row = row.push(
                text("临时 ")
                    .size(FONT_SIZE_SMALLER)
                    .style(|theme: &Theme| {
                        let palette = theme.palette();
                        text::Style {
                            color: Some(palette.warning),
                        }
                    }),
            )
        }
        None
    };

    let node_view = mouse_area(
        container(
            row.push(
                text(&node.name)
                    .size(FONT_SIZE_SMALLER)
                    .wrapping(text::Wrapping::None),
            )
            .padding(Padding::from([PADDING_SMALLEST, (depth * TEXT_INDENTATION) as f32])),
        )
        .width(Length::Fill)
        .style(move |theme: &Theme| {
            let ex_palette = theme.extended_palette();
            let bg = if selected_id == Some(node.global_id) {
                ex_palette.background.strong.color.scale_alpha(0.75)
            } else if hovered_id == Some(node.global_id) {
                ex_palette.background.weaker.color
            } else {
                Color::TRANSPARENT
            };
            container::Style {
                background: Some(Background::Color(bg)),
                ..container::Style::default()
            }
        }),
    )
    .interaction(mouse::Interaction::Pointer)
    .on_press(FileTreeMessage::ChangeSelectedNode(node.global_id))
    .on_enter(FileTreeMessage::ChangeHoveredNode(node.global_id));

    match children_node_view {
        Some(children) => column![node_view, children],
        None => column![node_view],
    }
}
