use std::collections::HashMap;

use iced::{
    Alignment, Element, Length, Padding, Task, Theme, border::Radius, mouse, widget::{
        Column, Grid, center, center_x, column, container, grid, image, mouse_area, row, rule,
        scrollable, space, text,
    }
};
use tracing::instrument::WithSubscriber;

use crate::common::*;

#[derive(Debug, Default)]
pub struct ImageGallery {
    mode: ImageGalleryMode,
    images: HashMap<String, image::Handle>,
}

#[derive(Debug, Clone, Copy, Default)]
enum ImageGalleryMode {
    #[default]
    ListView,
    GridView,
}

#[derive(Debug, Clone)]
pub enum ImageGalleryMessage {
    LoadImage,
    ModeChange(ImageGalleryMode),
}

impl ImageGallery {
    pub fn update(
        &mut self,
        image_gallery_message: ImageGalleryMessage,
    ) -> Task<ImageGalleryMessage> {
        match image_gallery_message {
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, ImageGalleryMessage> {
        let hidden_scroller = scrollable::Scrollbar::new().scroller_width(0).width(0);
        let mut grid = Grid::new()
            .columns(2)
            .spacing(SPACING_BIGGER)
            .height(Length::Shrink);
        // let mut column = Column::new()
        //     .spacing(SPACING)
        //     .padding(Padding::from([PADDING_BASE, PADDING_BIGGER]));
        let c = ["test.png", "test3.png", "test2.jpg"];
        for i in 1..10 {
            let image =
                image(image::Handle::from_path(c[i % 3])).content_fit(iced::ContentFit::Contain);
            grid = grid.push(column![
                text("图片1sdfds").size(FONT_SIZE_SMALLER).width(Length::Fill).align_x(Alignment::Center),
                image
            ].spacing(SPACING_SMALLER));
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
            container(scrollable(grid)
                .direction(scrollable::Direction::Vertical(hidden_scroller)))
                .height(Length::Fill)
                .padding(Padding::from([PADDING_BASE, PADDING_BIGGER]))
        ])
        .into()
    }
}
