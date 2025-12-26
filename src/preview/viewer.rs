use std::{collections::HashMap, sync::LazyLock};

use iced::{
    Background, Color, Element, Font, Length, Padding, Renderer, Theme, padding,
    widget::{
        center_x, column, container, image,
        markdown::{self as iced_markdown, Highlight},
        rich_text, sensor, space, text,
    },
};

use crate::{common::*, preview::markdown::MarkdownMessage};

pub const CUSTOM_SETTINGS: LazyLock<iced_markdown::Settings> = LazyLock::new(|| {
    let palette = DEFAULT_THEME.palette();
    let ex_palette = DEFAULT_THEME.extended_palette().to_owned();
    iced_markdown::Settings::with_style(iced_markdown::Style {
        font: Font::default(),
        inline_code_font: Font::default(),
        inline_code_color: palette.success,
        code_block_font: Font::default(),
        link_color: palette.success,
        inline_code_padding: padding::left(10).right(10),
        inline_code_highlight: Highlight {
            background: Background::Color(ex_palette.background.weaker.color),
            border: DEFAULT_BORDER,
        },
    })
});

pub struct CustomViewer<'a> {
    pub image: &'a HashMap<iced_markdown::Uri, image::Handle>,
}

impl<'a> iced_markdown::Viewer<'a, MarkdownMessage> for CustomViewer<'a> {
    fn on_link_click(url: iced_markdown::Uri) -> MarkdownMessage {
        MarkdownMessage::LinkClicked(url)
    }

    fn image(
        &self,
        settings: iced_markdown::Settings,
        url: &'a iced_markdown::Uri,
        title: &'a str,
        alt: &iced_markdown::Text,
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        if let Some(handle) = self.image.get(url) {
            center_x(image(handle)).into()
        } else {
            space().into()
        }
    }

    fn code_block(
        &self,
        settings: iced_markdown::Settings,
        language: Option<&'a str>,
        code: &'a str,
        lines: &'a [iced_markdown::Text],
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        container(column(
            lines
                .iter()
                .map(|text| rich_text(text.spans(settings.style)).into()),
        ))
        .width(Length::Fill)
        .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
        .style(|theme: &Theme| {
            let ex_palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(ex_palette.background.weaker.color)),
                border: DEFAULT_BORDER,
                ..Default::default()
            }
        })
        .into()
    }

    // fn heading(
    //         &self,
    //         settings: iced_markdown::Settings,
    //         level: &'a iced_markdown::HeadingLevel,
    //         text: &'a iced_markdown::Text,
    //         index: usize,
    //     ) -> Element<'a, MarkdownMessage, Theme, Renderer> {

    // }
}
