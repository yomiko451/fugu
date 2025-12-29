use iced::{
    Background, Border, Color, Element, Font, Length, Padding, Pixels, Renderer, Theme, alignment, border::Radius, padding, widget::{
        center_x, checkbox, column, container, image,
        markdown::{self as iced_markdown, Bullet, Highlight, Row},
        rich_text, row, rule, scrollable, space, table, text,
    }
};
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

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
        inline_code_padding: Padding::ZERO,
        inline_code_highlight: Highlight {
            background: Background::Color(Color::TRANSPARENT),
            border: Border::default(),
        },
    })
});

pub struct CustomViewer<'a> {
    pub image: &'a HashMap<String, image::Handle>,
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
        container(column![
            row![
                space::horizontal(),
                text!("语言 {}", language.unwrap_or("文本")).color(CODE_BLOCK_TEXT_COLOR),
                text!(
                    "行数 {}",
                    if lines.is_empty() {
                        String::default()
                    } else {
                        lines.len().to_string()
                    }
                )
                .color(CODE_BLOCK_TEXT_COLOR),
                space::horizontal(),
            ]
            .spacing(SPACING)
            .padding(padding::bottom(PADDING_SMALLER)),
            column(
                lines
                    .iter()
                    .map(|text| rich_text(text.spans(settings.style)).into()),
            )
        ])
        .width(Length::Fill)
        .padding(Padding::from([PADDING_BASE, PADDING_BIGGER]))
        .style(move |theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                text_color: if language.is_none() {
                    Some(CODE_BLOCK_TEXT_COLOR)
                } else {
                    Some(palette.text)
                },
                background: Some(Background::Color(palette.background)),
                border: DEFAULT_BORDER,
                ..Default::default()
            }
        })
        .into()
    }

    fn heading(
        &self,
        settings: iced_markdown::Settings,
        level: &'a iced_markdown::HeadingLevel,
        text: &'a iced_markdown::Text,
        index: usize,
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        let iced_markdown::Settings {
            h1_size,
            h2_size,
            h3_size,
            h4_size,
            h5_size,
            h6_size,
            text_size,
            ..
        } = settings;

        let (size, color) = match level {
            iced_markdown::HeadingLevel::H1 => (h1_size, H1_COLOR),
            iced_markdown::HeadingLevel::H2 => (h2_size, H2_COLOR),
            iced_markdown::HeadingLevel::H3 => (h3_size, H3_COLOR),
            iced_markdown::HeadingLevel::H4 => (h4_size, H4_COLOR),
            iced_markdown::HeadingLevel::H5 => (h5_size, H5_COLOR),
            iced_markdown::HeadingLevel::H6 => (h6_size, H6_COLOR),
        };
        container(
            rich_text(text.spans(settings.style))
                .color(color)
                .on_link_click(MarkdownMessage::LinkClicked)
                .size(size),
        )
        .padding(padding::top(if index > 0 {
            text_size / 2.0
        } else {
            Pixels::ZERO
        }))
        .into()
    }

    fn quote(
        &self,
        settings: iced_markdown::Settings,
        contents: &'a [iced_markdown::Item],
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        container(
            row![
                rule::vertical(6).style(|theme: &Theme| {
                    let ex_palette = theme.extended_palette();
                    rule::Style {
                        color: ex_palette.background.weaker.color,
                        radius: Radius::from(3),
                        snap: true,
                        fill_mode: rule::FillMode::Full,
                    }
                }),
                column(
                    contents
                        .iter()
                        .enumerate()
                        .map(|(i, content)| iced_markdown::item(self, settings, content, i)),
                )
                .spacing(settings.spacing.0),
            ]
            .height(Length::Shrink)
            .spacing(settings.spacing.0),
        )
        .style(|_| container::Style {
            text_color: Some(QUOTE_MARK_COLOR),
            ..Default::default()
        })
        .into()
    }

    fn table(
        &self,
        settings: iced_markdown::Settings,
        columns: &'a [iced_markdown::Column],
        rows: &'a [iced_markdown::Row],
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        let table = iced_markdown::table(self, settings, columns, rows);

        container(table)
            .style(|theme: &Theme| {
                let palette = theme.palette();
                container::Style {
                    border: Border {
                        width: 1.,
                        color: palette.text,
                        ..DEFAULT_BORDER
                    },
                    ..Default::default()
                }
            })
            .padding(PADDING_SMALLEST)
            .into()
    }
}
