use std::{ffi::OsStr, path::PathBuf};
use crate::{constants::*, file_panel::FilePanelMessage};
use iced::{mouse, widget::{column, container, mouse_area, text, Column, Row}, Background, Length, Padding, Theme};

const EXTENSION_STR: [&str; 3] = ["md", "png", "jpg"];

pub async fn open_file_dialog() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("打开文件")
        .add_filter("markdown文件(*md)", &["md"])
        .pick_file()
        .await?;

    Some(path.path().to_path_buf())
}

pub async fn open_filder() -> Option<PathBuf> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("打开文件夹")
        .pick_folder()
        .await?;

    Some(path.path().to_path_buf())
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub expanded: bool,
    pub node: PathBuf,
    pub children: Vec<usize>,
}
#[derive(Debug, Clone)]
pub struct FileTree {
    pub nodes: Vec<FileNode>,
}

// 异步读取文件并生成节点树
pub async fn load_file_tree(path: PathBuf) -> anyhow::Result<FileTree> {
    let mut file_tree = FileTree { nodes: vec![] };

    let root_node = FileNode {
        expanded: true,
        node: path.to_path_buf(),
        children: vec![],
    };

    let mut node_index_stack = vec![(root_node.node.clone(), 0)];
    file_tree.nodes.push(root_node);

    while let Some((path, index)) = node_index_stack.pop() {
        let mut dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if !path.is_dir() {
                if let Some(extension) = path.extension()
                    && EXTENSION_STR.contains(&extension.to_string_lossy().as_ref())
                {
                    let sub_file_node = FileNode {
                        expanded: false,
                        node: path.clone(),
                        children: vec![],
                    };

                    let child_index = file_tree.nodes.len();
                    file_tree.nodes.push(sub_file_node);
                    file_tree.nodes[index].children.push(child_index);
                } else {
                    continue;
                }
            } else {
                let sub_file_node = FileNode {
                    expanded: false,
                    node: path.clone(),
                    children: vec![],
                };

                let child_index = file_tree.nodes.len();
                file_tree.nodes.push(sub_file_node);
                file_tree.nodes[index].children.push(child_index);
                node_index_stack.push((path, child_index));
            }
        }
    }

    anyhow::Ok(file_tree)
}

// 递归渲染节点树
pub fn view_node(hovered_id: Option<usize>, file_tree: &FileTree, id: usize, depth: usize) -> Column<'_, FilePanelMessage> {
        let node = &file_tree.nodes[id];
        let row = mouse_area(
            container(
                Row::new()
                    .padding(Padding::default().left(depth as u32 * 10))
                    .push(
                        text(node.node.file_name().unwrap().to_string_lossy())
                            .size(FONT_SIZE_SMALLER)
                            .wrapping(text::Wrapping::None),
                    ),
            )
            .width(Length::Fill)
            .style(move |theme: &Theme| {
                let ex_palette = theme.extended_palette();
                if hovered_id == Some(id) {
                    container::Style {
                        background: Some(Background::Color(ex_palette.background.weaker.color)),
                        ..container::Style::default()
                    }
                } else {
                    container::Style::default()
                }
            }),
        )
        .interaction(mouse::Interaction::Pointer)
        .on_press(FilePanelMessage::Expanded(id))
        .on_enter(FilePanelMessage::HoverEnter(id));

        if !node.children.is_empty() {
            let mut column = Column::new().height(match node.expanded {
                false => 0.into(),
                true => Length::Shrink,
            });
            for &child in &node.children {
                column = column.push(view_node(hovered_id, file_tree, child, depth + 1));
            }
            column![row, column]
        } else {
            column![row]
        }
    }