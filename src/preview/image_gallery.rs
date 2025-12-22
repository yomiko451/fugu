use std::{collections::HashMap, path::PathBuf};

use crate::common::*;
use iced::{
    Alignment, Element, Length, Padding, Task, Theme,
    border::Radius,
    mouse,
    widget::{Grid, column, container, image, mouse_area, row, rule, scrollable, space, text},
};
use tracing::info;

#[derive(Debug, Default)]
pub struct ImageGallery {
    mode: ImageGalleryMode,
    images: HashMap<String, image::Handle>,
}

#[derive(Debug, Clone, Copy, Default)]
enum ImageGalleryMode {
    #[default]
    GridView,
    ListView,
}

#[derive(Debug, Clone)]
pub enum ImageGalleryMessage {
    LoadImage(ImgData),
    ModeChange(ImageGalleryMode),
}

impl ImageGallery {
    pub fn update(
        &mut self,
        image_gallery_message: ImageGalleryMessage,
    ) -> Task<ImageGalleryMessage> {
        match image_gallery_message {
            ImageGalleryMessage::LoadImage(image_data) => {
                self.images.entry(image_data.name).or_insert_with(|| {
                    info!("图片插入插入成功!");
                    image_data.handle
                });
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ImageGalleryMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        let mut grid = Grid::new()
            .columns(2)
            .spacing(SPACING_BIGGER)
            .height(Length::Shrink);
        for (name, handle) in &self.images {
            let image = image(handle).content_fit(iced::ContentFit::Contain);
            grid = grid.push(
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
            container(scrollable(grid).direction(scrollable::Direction::Vertical(hidden_scroller)))
                .height(Length::Fill)
                .padding(Padding::from([PADDING_BASE, PADDING_BIGGER]))
        ])
        .into()
    }
}
