use crate::{common::*, preview::{image_gallery::{ImageGallery, ImageGalleryMessage}, markdown::{Markdown, MarkdownMessage}}};
use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Subscription, Task, Theme,
    alignment::Horizontal,
    border::Radius,
    mouse,
    overlay::menu,
    widget::{
        Container, column, container, markdown as iced_markdown, mouse_area, pick_list, row, rule, scrollable,
        space, text, text_editor,
    },
};
use jiff::civil::Weekday;
use std::{path::PathBuf, sync::Arc};
use tracing::info;
mod image_gallery;
mod operation;
mod markdown;
mod viewer;
#[derive(Debug)]
pub struct Preview {
    current_weekday: String,
    current_date_time: String,
    current_page: PreviewPage,
    
    // 各个字组件
    content: Option<Arc<String>>,
    marddown: Markdown,
    image_gallery: ImageGallery,
    snap_shot_index_state: Vec<String>,
    current_snap_shot_index: Option<String>,
    current_snapshot_content: text_editor::Content,
}

#[derive(Debug, Clone)]
pub enum PreviewMessage {
    SyncContnetWithEditor(Arc<String>),
    GetImgPathFromFilePanel(ImgData),
    EditorAction(text_editor::Action),
    ChangePageTo(PreviewPage),
    ChangeSnapShot(String),
    UpdateTimeStr,
    // 处理子模块消息
    Markdown(MarkdownMessage),
    ImageGallery(ImageGalleryMessage)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PreviewPage {
    MarkDown,
    SnapShot,
    ImageGallery,
    Log,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            current_weekday: Preview::get_week_str(),
            current_date_time: Preview::get_time_str(),
            current_page: PreviewPage::MarkDown,
            current_snapshot_content: text_editor::Content::default(),
            content: None,
            marddown: Markdown::default(),
            image_gallery: ImageGallery::default(),
            current_snap_shot_index: None,
            snap_shot_index_state: vec![
                "快照1".to_string(),
                "快照2".to_string(),
                "快照3".to_string(),
            ],
        }
    }

    pub fn update(&mut self, preview_message: PreviewMessage, setting: &AppSetting) -> Task<PreviewMessage> {
        match preview_message {
            PreviewMessage::GetImgPathFromFilePanel(image_data) => {
                Task::done(PreviewMessage::ImageGallery(ImageGalleryMessage::LoadImage(image_data)))
            }
            PreviewMessage::UpdateTimeStr => {
                self.current_date_time = Preview::get_time_str();
                Task::none()
            }
            PreviewMessage::SyncContnetWithEditor(raw) => {
                self.marddown.update(MarkdownMessage::LoadRawText(raw)).map(PreviewMessage::Markdown)
            }
            PreviewMessage::EditorAction(action) => {
                // 禁止编辑操作让快照页面只读
                if !action.is_edit() {
                    self.current_snapshot_content.perform(action);
                }
                Task::none()
            }
            PreviewMessage::ChangePageTo(page) => {
                self.current_page = page;
                Task::none()
            }
            PreviewMessage::ChangeSnapShot(snap_shot_index) => {
                self.current_snap_shot_index = Some(snap_shot_index);
                Task::none()
            }
            // 处理各种子模块预览界面消息
            PreviewMessage::Markdown(markdown_message) => {
                match markdown_message {
                    
                    _ => self.marddown.update(markdown_message).map(PreviewMessage::Markdown)
                }
            }
            PreviewMessage::ImageGallery(image_gallery_message) => {
                match image_gallery_message {
                    
                    _ => self.image_gallery.update(image_gallery_message).map(PreviewMessage::ImageGallery)
                }
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, PreviewMessage> {
        let preview = match self.current_page {
            PreviewPage::MarkDown => self.marddown.view().map(PreviewMessage::Markdown),
            PreviewPage::ImageGallery => self.image_gallery.view().map(PreviewMessage::ImageGallery),
            PreviewPage::SnapShot => self.generate_snapshot_component(),
            _ => self.marddown.view().map(PreviewMessage::Markdown),
        };

        container(column![
            container(
                row![
                    self.generate_page_change_button("预览", PreviewPage::MarkDown),
                    self.generate_page_change_button("图片", PreviewPage::ImageGallery),
                    self.generate_page_change_button("快照", PreviewPage::SnapShot),
                    self.generate_page_change_button("日志", PreviewPage::Log),
                ]
                .height(Length::Shrink)
            )
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weaker.color)),
                    ..container::Style::default()
                }
            }),
            preview,
            container(
                row![
                    space::horizontal(),
                    text!("{}  {}", self.current_date_time, self.current_weekday).size(FONT_SIZE_BASE),
                ]
                .width(Length::Fill)
            )
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .height(Length::Shrink)
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weaker.color)),
                    ..container::Style::default()
                }
            })
        ])
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weakest.color)),
                ..container::Style::default()
            }
        })
    }

    pub fn subscription(&self) -> Subscription<PreviewMessage> {
        iced::time::every(iced::time::Duration::from_secs(1)).map(|_| PreviewMessage::UpdateTimeStr)
    }

    pub fn generate_page_change_button(
        &self,
        label: &'static str,
        page: PreviewPage,
    ) -> Element<'_, PreviewMessage> {
        mouse_area(
            container(
                text(label)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .size(FONT_SIZE_BIGGER),
            )
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .style(move |theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(if self.current_page == page {
                        ex_palette.background.weakest.color
                    } else {
                        ex_palette.background.weaker.color
                    })),
                    ..container::Style::default()
                }
            }),
        )
        .interaction(mouse::Interaction::Pointer)
        .on_press(PreviewMessage::ChangePageTo(page))
        .into()
    }

   

    pub fn generate_snapshot_component(&self) -> Element<'_, PreviewMessage> {
        column![
            row![
                pick_list(
                    self.snap_shot_index_state.as_slice(),
                    self.current_snap_shot_index.as_ref(),
                    PreviewMessage::ChangeSnapShot
                )
                .text_size(FONT_SIZE_SMALLER)
                .text_line_height(1.)
                .style(|theme: &Theme, _| {
                    let ex_palette = theme.extended_palette();
                    let palette = theme.palette();
                    pick_list::Style {
                        text_color: palette.text,
                        background: Background::Color(ex_palette.background.weaker.color),
                        border: Border::default(),
                        placeholder_color: palette.text,
                        handle_color: palette.text,
                    }
                })
                .menu_style(|theme: &Theme| {
                    let ex_palette = theme.extended_palette();
                    let palette = theme.palette();
                    menu::Style {
                        background: Background::Color(ex_palette.background.weaker.color),
                        selected_background: Background::Color(ex_palette.background.base.color),
                        selected_text_color: palette.text,
                        text_color: palette.text,
                        border: Border::default(),
                        shadow: SHADOW_BASE,
                    }
                }),
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
            text_editor(&self.current_snapshot_content)
                .on_action(PreviewMessage::EditorAction)
                .height(Length::Fill)
                .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
                .style(|theme: &Theme, _| {
                    let palette = theme.palette();
                    text_editor::Style {
                        background: Background::Color(Color::TRANSPARENT),
                        border: Border::default(),
                        placeholder: palette.text,
                        value: palette.text,
                        selection: palette.primary,
                    }
                })
        ]
        .into()
    }

    pub fn get_time_str() -> String {
        let now = jiff::Zoned::now();
        now.strftime("%Y/%m/%d  %H:%M:%S").to_string()
    }
    
    pub fn get_week_str() -> String {
        let weekday = jiff::Zoned::now().weekday();
        match weekday {
            Weekday::Monday => "星期一".to_string(),
            Weekday::Tuesday => "星期二".to_string(),
            Weekday::Wednesday => "星期三".to_string(),
            Weekday::Thursday => "星期四".to_string(),
            Weekday::Friday => "星期五".to_string(),
            Weekday::Saturday => "星期六".to_string(),
            Weekday::Sunday => "星期天".to_string(),
        }
    }
}
