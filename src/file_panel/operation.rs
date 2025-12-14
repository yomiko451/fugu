use crate::{common::*, file_panel::FilePanelMessage};
use anyhow::{Ok, anyhow};
use iced::{
    Background, Color, Length, Padding, Theme, mouse,
    widget::{Column, Row, column, container, mouse_area, text},
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
const EXTENSION_STR: [&str; 3] = ["md", "png", "jpg"];

pub fn get_next_id() -> u32 {
    FILE_NODE_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub id: u32,
    pub is_dir: bool,
    pub expanded: bool,
    pub file_name: String,
    pub path: Option<PathBuf>,
    pub content_cache: Option<Arc<String>>,
    pub children: Vec<u32>,
}

pub async fn open_file_dialog() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("打开文件")
        .add_filter("markdown文件(*md)", &["md"])
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

pub async fn save_file_dialog(file_name: String) -> anyhow::Result<PathBuf> {
    let result = rfd::AsyncFileDialog::new()
        .add_filter("markdown文件(*md)", &["md"])
        .set_file_name(file_name)
        .set_title("保存文件")
        .save_file()
        .await;

    if let Some(file_handle) = result {
        Ok(file_handle.path().to_path_buf())
    } else {
        Err(anyhow!("FileHandle路径为空!"))
    }
}

pub async fn read_file(path: PathBuf) -> anyhow::Result<FileData> {
    let content = tokio::fs::read_to_string(&path).await?;
    let name = path.file_name().unwrap().to_string_lossy().into_owned();
    let file_data = FileData {
        name,
        content: Arc::new(content),
    };
    Ok(file_data)
}

pub async fn save_file(path: PathBuf, content: Arc<String>) -> anyhow::Result<()> {
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}

// #[derive(Debug, Clone)]
// pub struct FileTree(pub Vec<FileNode>);

// 异步读取文件并生成节点树
pub async fn load_file_tree(path: PathBuf) -> anyhow::Result<(u32, HashMap<u32, FileNode>)> {
    let mut file_tree = HashMap::new();
    let root_file_node_key = get_next_id();
    let root_node = FileNode {
        is_dir: path.is_dir(),
        id: root_file_node_key,
        expanded: true,
        path: Some(path.to_path_buf()),
        file_name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        children: vec![],
        content_cache: None,
    };

    let mut node_index_stack = vec![(root_node.path.as_ref().unwrap().clone(), root_node.id)];
    file_tree.insert(root_node.id, root_node);

    while let Some((path, key)) = node_index_stack.pop() {
        let mut dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                if let Some(extension) = path.extension()
                    && EXTENSION_STR.contains(&extension.to_string_lossy().as_ref())
                {
                    let child_file_node = FileNode {
                        is_dir: path.is_dir(),
                        id: get_next_id(),
                        expanded: false,
                        path: Some(path.clone()),
                        file_name: path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned(),
                        children: vec![],
                        content_cache: None,
                    };

                    if let Some(file_node) = file_tree.get_mut(&key) {
                        file_node.children.push(child_file_node.id);
                    }
                    file_tree.insert(child_file_node.id, child_file_node);
                } else {
                    continue;
                }
            } else {
                let child_file_node = FileNode {
                    is_dir: path.is_dir(),
                    id: get_next_id(),
                    expanded: false,
                    path: Some(path.clone()),
                    file_name: path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned(),
                    children: vec![],
                    content_cache: None,
                };
                node_index_stack.push((path, child_file_node.id));
                if let Some(file_node) = file_tree.get_mut(&key) {
                    file_node.children.push(child_file_node.id);
                }
                file_tree.insert(child_file_node.id, child_file_node);
            }
        }
    }

    anyhow::Ok((root_file_node_key, file_tree))
}

// 递归渲染节点树
pub fn view_node(
    hovered_id: Option<u32>,
    selected_id: Option<u32>,
    all_file_nodes: &HashMap<u32, FileNode>,
    key: u32,
    depth: u16,
) -> Column<'_, FilePanelMessage> {
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
        if node.is_dir {
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
    .on_press(FilePanelMessage::ChangeSelectedFileNode(node.id))
    .on_enter(FilePanelMessage::ChangeHoveredFileNode(node.id));

    match children_node_view {
        Some(children) => column![node_view, children],
        None => column![node_view],
    }
}
