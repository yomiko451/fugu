use crate::{
    common::*,
    preview::{
        image_gallery::{ImageGallery, ImageGalleryMessage},
        log_viewer::{LogViewer, LogViewerMessage},
        markdown::{Markdown, MarkdownMessage},
        text_board::{TextBoard, TextBoardMessage},
    },
};
use iced::{
    Background, Border, Color, Element, Length, Padding, Shadow, Subscription, Task, Theme,
    alignment::Horizontal,
    border::Radius,
    mouse,
    overlay::menu,
    widget::{
        Container, column, container, markdown as iced_markdown, mouse_area, pick_list, row, rule,
        scrollable, space, text, text_editor,
    },
};
use jiff::civil::Weekday;
use std::{path::PathBuf, sync::Arc};
use tracing::info;
mod image_gallery;
mod log_viewer;
mod markdown;
mod text_board;
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
    text_board: TextBoard,
    log_viewer: LogViewer,
}

#[derive(Debug, Clone)]
pub enum PreviewMessage {
    SyncContnetWithEditor(Arc<String>),
    GetImgPathFromFilePanel(Vec<ImgData>),
    EditorAction(text_editor::Action),
    ChangePageTo(PreviewPage),
    UpdateTimeStr,
    // 处理子模块消息
    Markdown(MarkdownMessage),
    ImageGallery(ImageGalleryMessage),
    TextBoard(TextBoardMessage),
    LogView(LogViewerMessage),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PreviewPage {
    MarkDown,
    TextBoard,
    ImageGallery,
    LogViewer,
}

impl Preview {
    pub fn new() -> Self {
        Self {
            current_weekday: Preview::get_week_str(),
            current_date_time: Preview::get_time_str(),
            current_page: PreviewPage::MarkDown,
            content: None,
            marddown: Markdown::default(),
            image_gallery: ImageGallery::default(),
            text_board: TextBoard::default(),
            log_viewer: LogViewer::new(),
        }
    }

    pub fn update(
        &mut self,
        preview_message: PreviewMessage,
        setting: &AppSetting,
    ) -> Task<PreviewMessage> {
        match preview_message {
            PreviewMessage::GetImgPathFromFilePanel(image_data) => Task::done(
                PreviewMessage::ImageGallery(ImageGalleryMessage::LoadImage(image_data)),
            ),
            PreviewMessage::UpdateTimeStr => {
                self.current_date_time = Preview::get_time_str();
                Task::none()
            }
            PreviewMessage::SyncContnetWithEditor(raw) => self
                .marddown
                .update(MarkdownMessage::LoadRawText(raw))
                .map(PreviewMessage::Markdown),
            PreviewMessage::ChangePageTo(page) => {
                self.current_page = page;
                Task::none()
            }
            // 处理各种子模块预览界面消息
            PreviewMessage::Markdown(markdown_message) => match markdown_message {
                _ => self
                    .marddown
                    .update(markdown_message)
                    .map(PreviewMessage::Markdown),
            },
            PreviewMessage::ImageGallery(image_gallery_message) => match image_gallery_message {
                ImageGalleryMessage::ShowImageGallery => {
                    self.current_page = PreviewPage::ImageGallery;
                    Task::none()
                }
                _ => self
                    .image_gallery
                    .update(image_gallery_message)
                    .map(PreviewMessage::ImageGallery),
            },
            PreviewMessage::TextBoard(text_board_message) => match text_board_message {
                _ => self
                    .text_board
                    .update(text_board_message)
                    .map(PreviewMessage::TextBoard),
            },
            PreviewMessage::LogView(log_view_message) => match log_view_message {
                _ => self
                    .log_viewer
                    .update(log_view_message)
                    .map(PreviewMessage::LogView),
            },
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, PreviewMessage> {
        let preview = match self.current_page {
            PreviewPage::MarkDown => self.marddown.view().map(PreviewMessage::Markdown),
            PreviewPage::ImageGallery => {
                self.image_gallery.view().map(PreviewMessage::ImageGallery)
            }
            PreviewPage::TextBoard => self.text_board.veiw().map(PreviewMessage::TextBoard),
            PreviewPage::LogViewer => self.log_viewer.view().map(PreviewMessage::LogView),
        };

        container(column![
            container(
                row![
                    self.generate_page_change_button("预览", PreviewPage::MarkDown),
                    self.generate_page_change_button("图片", PreviewPage::ImageGallery),
                    self.generate_page_change_button("文本", PreviewPage::TextBoard),
                    self.generate_page_change_button("日志", PreviewPage::LogViewer),
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
                    text!("{}  {}", self.current_date_time, self.current_weekday)
                        .size(FONT_SIZE_BASE),
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
        Subscription::batch([
            iced::time::every(iced::time::Duration::from_secs(1)).map(|_| PreviewMessage::UpdateTimeStr),
            self.log_viewer.subscription().map(PreviewMessage::LogView)
        ])
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
