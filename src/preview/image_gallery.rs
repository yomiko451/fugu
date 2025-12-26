use std::{collections::HashMap, path::PathBuf};

use crate::common::*;
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme,
    border::Radius,
    mouse,
    overlay::menu,
    widget::{
        Grid, Row, center, center_x, column, container, image, mouse_area, pick_list, radio, row,
        rule, scrollable, space, text,
    },
};
use tracing::info;

#[derive(Debug, Default)]
pub struct ImageGallery {
    selected_img_name: Option<String>,
    mode: Option<ImageGalleryMode>,
    images: HashMap<String, image::Handle>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageGalleryMode {
    GridView,
    ListView,
}

#[derive(Debug, Clone)]
pub enum ImageGalleryMessage {
    LoadImage(Vec<ImgData>),
    ModeChange(ImageGalleryMode),
    ChangeSelectedImg(String),
    ShowImageGallery,
}

impl ImageGallery {
    pub fn update(
        &mut self,
        image_gallery_message: ImageGalleryMessage,
    ) -> Task<ImageGalleryMessage> {
        match image_gallery_message {
            ImageGalleryMessage::LoadImage(image_data) => {
                let image_count = image_data.len();
                if image_count == 1 {
                    let image = image_data
                        .into_iter()
                        .next()
                        .expect("必定有一个元素不应当出错!");
                    self.selected_img_name = Some(image.name.clone());
                    self.images.entry(image.name).or_insert_with(|| {
                        info!("图片载入成功!");
                        image.handle
                    });
                    self.mode = Some(ImageGalleryMode::ListView);
                    return Task::done(ImageGalleryMessage::ShowImageGallery);
                } else if image_count > 1 {
                    self.selected_img_name =
                        Some(image_data.first().expect("必定不出错!").name.clone());
                    for image in image_data {
                        self.images.entry(image.name).or_insert_with(|| {
                            info!("图片插入插入成功!");
                            image.handle
                        });
                    }
                    self.mode = Some(ImageGalleryMode::GridView);
                    return Task::done(ImageGalleryMessage::ShowImageGallery);
                }
                Task::none()
            }
            ImageGalleryMessage::ChangeSelectedImg(img_name) => {
                self.selected_img_name = Some(img_name);
                Task::none()
            }
            ImageGalleryMessage::ModeChange(mode) => {
                self.mode = Some(mode);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ImageGalleryMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        let (head, body): (
            Row<'_, ImageGalleryMessage>,
            Element<'_, ImageGalleryMessage>,
        ) = {
            match self.mode {
                Some(mode) => {
                    let radio_a = radio(
                        "预览视图",
                        ImageGalleryMode::ListView,
                        self.mode,
                        ImageGalleryMessage::ModeChange,
                    )
                    .style(|theme: &Theme, _| {
                        let palette = theme.palette();
                        radio::Style {
                            background: Background::Color(Color::TRANSPARENT),
                            text_color: Some(palette.text),
                            dot_color: palette.text,
                            border_color: palette.text,
                            border_width: DEFAULT_BORDER.width,
                        }
                    });
                    let radio_b = radio(
                        "网格视图",
                        ImageGalleryMode::GridView,
                        self.mode,
                        ImageGalleryMessage::ModeChange,
                    )
                    .style(|theme: &Theme, _| {
                        let palette = theme.palette();
                        radio::Style {
                            background: Background::Color(Color::TRANSPARENT),
                            text_color: Some(palette.text),
                            dot_color: palette.text,
                            border_color: palette.text,
                            border_width: DEFAULT_BORDER.width,
                        }
                    });
                    match mode {
                        ImageGalleryMode::GridView => {
                            let mut body = Grid::new()
                                .columns(2)
                                .spacing(SPACING_BIGGER)
                                .height(Length::Shrink);
                            let mut images = self.images.iter().collect::<Vec<_>>();
                            images.sort_by_key(|image| image.0);
                            for (name, handle) in images {
                                let image = image(handle).content_fit(iced::ContentFit::Contain);
                                body = body.push(
                                    column![
                                        text(name)
                                            .size(FONT_SIZE_SMALLER)
                                            .width(Length::Fill)
                                            .align_x(Alignment::Center),
                                        image
                                    ]
                                    .spacing(SPACING_SMALLER),
                                );
                            }

                            (row![text!("共 {} 张图片", self.images.len()).size(FONT_SIZE_BIGGER), space::horizontal(), radio_a, radio_b], body.into())
                        }
                        ImageGalleryMode::ListView => {
                            let body: Element<'_, ImageGalleryMessage> = if let Some(handle) =
                                self.selected_img_name
                                    .as_ref()
                                    .and_then(|key| self.images.get(key))
                            {
                                center(image(handle).content_fit(iced::ContentFit::Contain)).into()
                            } else {
                                space().into()
                            };
                            let mut options = self.images.keys().into_iter().collect::<Vec<_>>();
                            options.sort();
                            let pick_list =
                                pick_list(options, self.selected_img_name.as_ref(), |name| {
                                    ImageGalleryMessage::ChangeSelectedImg(name.to_string())
                                })
                                .text_size(FONT_SIZE_SMALLER)
                                .text_line_height(1.)
                                .style(|theme: &Theme, _| {
                                    let ex_palette = theme.extended_palette();
                                    let palette = theme.palette();
                                    pick_list::Style {
                                        text_color: palette.text,
                                        background: Background::Color(
                                            ex_palette.background.weaker.color,
                                        ),
                                        border: Border::default(),
                                        placeholder_color: palette.text,
                                        handle_color: palette.text,
                                    }
                                })
                                .menu_style(|theme: &Theme| {
                                    let ex_palette = theme.extended_palette();
                                    let palette = theme.palette();
                                    menu::Style {
                                        background: Background::Color(
                                            ex_palette.background.weaker.color,
                                        ),
                                        selected_background: Background::Color(
                                            ex_palette.background.base.color,
                                        ),
                                        selected_text_color: palette.text,
                                        text_color: palette.text,
                                        border: Border::default(),
                                        shadow: SHADOW_BASE,
                                    }
                                });

                            (
                                row![pick_list, space::horizontal(), radio_a, radio_b],
                                body,
                            )
                        }
                    }
                }
                None => (
                    row![center_x(
                        mouse_area(text("导入图片文件夹").size(FONT_SIZE_BIGGER))
                            .interaction(mouse::Interaction::Pointer)
                    )],
                    space().into(),
                ),
            }
        };
        container(column![
            head.spacing(SPACING_BIGGER)
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
                scrollable(body).direction(scrollable::Direction::Vertical(hidden_scroller))
            )
            .height(Length::Fill)
            .padding(Padding::from([PADDING_BASE, PADDING_BIGGER]))
        ])
        .into()
    }
}
