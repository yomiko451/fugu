use crate::{
    common::*,
    preview::{
        markdown,
        viewer::{CUSTOM_SETTINGS, CustomViewer},
    },
};
use iced::{
    Element, Length, Padding, Task, Theme,
    border::Radius,
    mouse,
    widget::{
        column, container, image,
        markdown::{self as iced_markdown, Uri},
        mouse_area, row, rule, scrollable, space, text,
    },
    window,
};
use std::{collections::HashMap, path::PathBuf, result, sync::Arc};
use tracing::info;

#[derive(Debug, Default)]
pub struct Markdown {
    raw: Option<Arc<String>>,
    content: iced_markdown::Content,
    image: HashMap<String, image::Handle>,
    image_base_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ImgHandle {
    ab_path: PathBuf,
    handle: image::Handle
}

#[derive(Debug, Clone)]
pub enum MarkdownMessage {
    GetImgBasePathFromFilePanel(PathBuf),
    LoadRawText(Arc<String>),
    RenderMarkdown,
    HandleImageUrl(Vec<(PathBuf, String)>),
    InsertImageToDict(Vec<(String, image::Handle)>),
    SendImgUrlToFilePanel(Vec<PathBuf>),
    LinkClicked(iced_markdown::Uri),
}

impl Markdown {
    pub fn update(&mut self, markdown_message: MarkdownMessage) -> Task<MarkdownMessage> {
        match markdown_message {
            MarkdownMessage::GetImgBasePathFromFilePanel(path) => {
                self.image_base_path = Some(path);
                Task::none()
            }
            MarkdownMessage::LoadRawText(raw) => {
                self.raw = Some(raw);
                Task::done(MarkdownMessage::RenderMarkdown)
            }
            MarkdownMessage::RenderMarkdown => {
                if let Some(ref content) = self.raw {
                    self.content = iced_markdown::Content::parse(content);
                    if let Some(ref path) = self.image_base_path {
                        let url_vec = self
                            .content
                            .images()
                            .iter()
                            .filter(|url| {
                                (url.ends_with("jpg") || url.ends_with("png"))
                                    && !self.image.contains_key(url.as_str())
                            })
                            // TODO
                            // 这里需要大量重写，采用缓存机制，
                            // 编辑过程中只暂时用缓存handle预览图片,保存时再创建文件同名文件夹并复制图片文件过去
                            // 如果用户直接输入的绝对路径，照常载入handle到markdown的images
                            // 如果用户通过图片面板插入，preview内部markdown和image_gallery通信传递handle即可
                            // 用户输入的源码尽量不动，重点在于源码路径与真实路径间的映射与文件拷贝                        
                            
                            .map(|url| (path.parent().expect("必定合法路径不应当出错!").join(url), url.to_string()))
                            .collect::<Vec<(PathBuf, String)>>();
                        return Task::done(MarkdownMessage::HandleImageUrl(url_vec));
                    }
                }
                Task::none()
            }
            MarkdownMessage::HandleImageUrl(url_vec) => 
            // TODO 让file_panel读取图片，用read_many_img_files函数
            // 还要额外做各种处理最后统一存成ImgHandle
            Task::future(async {
                tokio::task::spawn_blocking(|| {
                    url_vec
                        .into_iter()
                        .map(|(path, key)| {
                            let handle = image::Handle::from_path(&path);
                            (key, handle)
                        })
                        .collect::<Vec<(String, image::Handle)>>()
                })
                .await
            })
            .then(|result| match result {
                Ok(images) => Task::done(MarkdownMessage::InsertImageToDict(images)),
                Err(error) => {
                    info!("{}", error.to_string());
                    Task::none()
                }
            }),
            MarkdownMessage::InsertImageToDict(images) => {
                for (url, handle) in images {
                    
                    self.image.insert(url, handle);
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, MarkdownMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        container(column![
            row![
                space::horizontal(),
                mouse_area(text("恢复").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer),
                mouse_area(text("删除").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer),
                mouse_area(text("另存为").size(FONT_SIZE_BIGGER))
                    .interaction(mouse::Interaction::Pointer)
            ]
            .spacing(SPACING_BIGGER)
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
            container(
                scrollable(iced_markdown::view_with(
                    self.content.items(),
                    *CUSTOM_SETTINGS,
                    &CustomViewer { image: &self.image },
                ))
                .direction(scrollable::Direction::Vertical(hidden_scroller))
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        ])
        .into()
    }
}
