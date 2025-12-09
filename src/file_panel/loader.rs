use std::path::{Path, PathBuf};

use anyhow::Ok;

#[derive(Debug, Clone)]
pub struct FileNode {
    pub node: PathBuf,
    pub children: Vec<usize>,
}
#[derive(Debug, Clone)]
pub struct FileTree {
    pub nodes: Vec<FileNode>,
}

pub async fn load_file_tree(path: PathBuf) -> anyhow::Result<FileTree> {
    let mut file_tree = FileTree { nodes: vec![] };

    let root_node = FileNode {
        node: path.to_path_buf(),
        children: vec![],
    };

    let mut node_index_stack = vec![(root_node.node.clone(), 0)];

    file_tree.nodes.push(root_node);

    while let Some((path, index)) = node_index_stack.pop() {
        let mut dir = tokio::fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();

            let sub_file_node = FileNode {
                node: path.clone(),
                children: vec![],
            };

            let child_index = file_tree.nodes.len();
            file_tree.nodes.push(sub_file_node);
            file_tree.nodes[index].children.push(child_index);
            
            if path.is_dir() {
                node_index_stack.push((path, child_index));
            }
        }
    }

    Ok(file_tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn file_tree_test() {
        let root_path = std::env::current_dir().unwrap();

        let tree = load_file_tree(root_path).await.unwrap();

        for i in &tree.nodes[3].children {
            println!("{:?}", tree.nodes[*i].node)
        }
    }
}
