use std::{collections::HashMap, sync::LazyLock};

use iced::{
    Background, Element, Font, Length, Padding, Pixels, Renderer, Theme, alignment, padding,
    widget::{
        center_x, checkbox, column, container, image,
        markdown::{self as iced_markdown, Bullet, Highlight},
        rich_text, row, space, text,
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
        container(column![
            row![
                text!("语言 {}", language.unwrap_or("文本")).color(DEFAULT_THEME.palette().text),
                text!(
                    "行数 {}",
                    if lines.is_empty() {
                        String::default()
                    } else {
                        lines.len().to_string()
                    }
                )
                .color(DEFAULT_THEME.palette().text)
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
        .style(|theme: &Theme| {
            let palette = theme.palette();
            container::Style {
                text_color: Some(palette.text),
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

    fn unordered_list(
        &self,
        settings: iced_markdown::Settings,
        bullets: &'a [iced_markdown::Bullet],
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        container(
            column(bullets.iter().map(|bullet| {
                let (items, mark) = match bullet {
                    Bullet::Point { items } => (items, text("•").size(settings.text_size).into()),
                    Bullet::Task { items, done, .. } => (
                        items,
                        Element::from(
                            container(checkbox(*done).size(settings.text_size)).center_y(
                                text::LineHeight::default().to_absolute(settings.text_size),
                            ),
                        ),
                    ),
                };
                row![
                    mark,
                    iced_markdown::view_with(
                        items,
                        iced_markdown::Settings {
                            spacing: settings.spacing * 0.6,
                            ..settings
                        },
                        self
                    ),
                ]
                .spacing(settings.spacing)
                .into()
            }))
            .spacing(settings.spacing * 0.75)
            .padding([0.0, settings.spacing.0]),
        )
        .style(|_| container::Style {
            text_color: Some(UNORDERED_LSIT_COLOR),
            ..Default::default()
        })
        .into()
    }

    fn ordered_list(
        &self,
        settings: iced_markdown::Settings,
        start: u64,
        bullets: &'a [Bullet],
    ) -> Element<'a, MarkdownMessage, Theme, Renderer> {
        let digits = ((start + bullets.len() as u64).max(1) as f32)
            .log10()
            .ceil();

        container(
            column(bullets.iter().enumerate().map(|(i, bullet)| {
                let items = match bullet {
                    Bullet::Point { items } => items,
                    Bullet::Task { items, .. } => items,
                };
                row![
                    text!("{}.", i as u64 + start)
                        .size(settings.text_size)
                        .align_x(alignment::Horizontal::Right)
                        .width(settings.text_size * ((digits / 2.0).ceil() + 1.0)),
                    iced_markdown::view_with(
                        items,
                        iced_markdown::Settings {
                            spacing: settings.spacing * 0.6,
                            ..settings
                        },
                        self,
                    )
                ]
                .spacing(settings.spacing)
                .into()
            }))
            .spacing(settings.spacing * 0.75),
        )
        .style(|_| container::Style {
            text_color: Some(ORDERED_LSIT_COLOR),
            ..Default::default()
        })
        .into()
    }
}
