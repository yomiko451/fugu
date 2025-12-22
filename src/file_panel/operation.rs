use crate::{common::*, file_panel::file_tree::FileTreeMessage};
use iced::{
    Background, Color, Length, Padding, Theme, mouse,
    widget::{Column, Row, column, container, image, mouse_area, text},
};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};
use tokio::io::AsyncWriteExt;

static FILE_NODE_COUNTER: AtomicU32 = AtomicU32::new(0);

pub fn get_next_id() -> u32 {
    FILE_NODE_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub id: u32,
    pub node_type: NodeType,
    pub expanded: bool,
    pub version: u64,
    pub file_name: String,
    pub path: Option<PathBuf>,
    pub string_cache: Option<Arc<String>>,
    pub handle_cache: Option<image::Handle>,
    pub children: Vec<u32>,
}

impl FileNode {
    pub fn new(path: Option<PathBuf>, node_type: NodeType, file_name: Option<String>) -> Self {
        FileNode {
            node_type,
            id: get_next_id(),
            expanded: false,
            version: 0,
            file_name: if let Some(file_name) = file_name {
                file_name
            } else {
                path.as_ref()
                    .expect("调用时必定是Some")
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            },
            path: path,
            children: vec![],
            string_cache: None,
            handle_cache: None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    DirectoryMd,
    DirectoryImg,
    Markdown,
    Image,
}

pub async fn open_file(node_type: NodeType) -> Option<FileNode> {
    let path = match node_type {
        NodeType::Markdown => {
            rfd::AsyncFileDialog::new()
                .set_title("打开文件")
                .add_filter("markdown文件(*md)", &["md"])
                .pick_file()
                .await?
        }
        NodeType::Image => {
            rfd::AsyncFileDialog::new()
                .set_title("打开文件")
                .add_filter("图片文件(*jpg,*png)", &["jpg", "png"])
                .pick_file()
                .await?
        }
        _ => {
            rfd::AsyncFileDialog::new()
                .set_title("打开文件夹")
                .pick_folder()
                .await?
        }
    };

    Some(FileNode::new(
        Some(path.path().to_path_buf()),
        node_type,
        None,
    ))
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

pub async fn read_file(path: PathBuf) -> Result<String, AppError> {
    let content = tokio::fs::read_to_string(&path).await?;
    Ok(content)
}

pub async fn get_img_handle(path: PathBuf) -> Result<image::Handle, AppError> {
    let bytes = tokio::fs::read(path).await?;
    let handle = image::Handle::from_bytes(bytes);
    Ok(handle)
}

pub async fn save_file(path: PathBuf, content: Arc<String>) -> Result<(), AppError> {
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

// 异步读取文件并生成节点树
pub async fn load_file_tree(
    mut root_node: FileNode,
) -> Result<(u32, HashMap<u32, FileNode>), AppError> {
    let mut file_tree = HashMap::new();
    root_node.expanded = true;
    let root_node_key = root_node.id;

    let mut node_index_stack = vec![(root_node.path.as_ref().unwrap().clone(), root_node.id)];
    file_tree.insert(root_node.id, root_node);

    while let Some((path, key)) = node_index_stack.pop() {
        let mut dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                if let Some(extension) = path.extension()
                    && extension.to_string_lossy().as_ref() == "md"
                {
                    let child_file_node = FileNode::new(Some(path), NodeType::Markdown, None);
                    if let Some(file_node) = file_tree.get_mut(&key) {
                        file_node.children.push(child_file_node.id);
                    }
                    file_tree.insert(child_file_node.id, child_file_node);
                } else {
                    continue;
                }
            } else {
                let child_file_node = FileNode::new(Some(path.clone()), NodeType::DirectoryMd, None);
                node_index_stack.push((path, child_file_node.id));
                if let Some(file_node) = file_tree.get_mut(&key) {
                    file_node.children.push(child_file_node.id);
                }
                file_tree.insert(child_file_node.id, child_file_node);
            }
        }
    }
    Ok((root_node_key, file_tree))
}

// 异步读取图片并生成节点列表
pub async fn get_img_handle_from_folder(path: PathBuf) -> Result<Vec<FileNode>, AppError> {
    
    // 之后遍历全部加入到all_nodes的字典中
    Ok(vec![])
} // 怎么设计？

// 递归渲染节点树
pub fn view_node(
    hovered_id: Option<u32>,
    selected_id: Option<u32>,
    all_file_nodes: &HashMap<u32, FileNode>,
    key: u32,
    depth: u16,
) -> Column<'_, FileTreeMessage> {
    let node = all_file_nodes.get(&key).unwrap();
    let mut row = Row::new();

    let children_node_view = if !node.children.is_empty() {
        if node.expanded {
            row = row.push(text(" ▼ ").size(FONT_SIZE_SMALLER));
        } else {
            row = row.push(text(" ▶ ").size(FONT_SIZE_SMALLER));
        }
        let mut column = Column::new().height(match node.expanded {
            false => 0.into(),
            true => Length::Shrink,
        });
        for child_key in &node.children {
            column = column.push(view_node(
                hovered_id,
                selected_id,
                all_file_nodes,
                *child_key,
                depth + 1,
            ));
        }
        Some(column)
    } else {
        if node.node_type == NodeType::DirectoryMd {
            row = row.push(text(" ▷ ").size(FONT_SIZE_SMALLER));
        }
        if node.path.is_none() {
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
                text(&node.file_name)
                    .size(FONT_SIZE_SMALLER)
                    .wrapping(text::Wrapping::None),
            )
            .padding(Padding::from([PADDING_SMALLEST, depth * TEXT_INDENTATION])),
        )
        .width(Length::Fill)
        .style(move |theme: &Theme| {
            let ex_palette = theme.extended_palette();
            let bg = if selected_id == Some(node.id) {
                ex_palette.background.strong.color.scale_alpha(0.75)
            } else if hovered_id == Some(node.id) {
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
    .on_press(FileTreeMessage::ChangeSelectedNode(node.id))
    .on_enter(FileTreeMessage::ChangeHoveredNode(node.id));

    match children_node_view {
        Some(children) => column![node_view, children],
        None => column![node_view],
    }
}
