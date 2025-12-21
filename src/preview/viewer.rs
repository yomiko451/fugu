use std::collections::HashMap;

use iced::{Element, Renderer, Theme, widget::{canvas::Image, center_x, image, markdown as iced_markdown, sensor, space, text}};

use crate::preview::{PreviewMessage, markdown::MarkdownMessage};



pub struct CustomViewer<'a> {
    pub image: &'a HashMap<iced_markdown::Uri, image::Handle>
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
                println!("{url}");
                sensor(text!("{}", url))
                    .key(url.clone())
                    .delay(iced::time::Duration::from_millis(500))
                    .on_show(move |_|{println!("emit: {url}"); MarkdownMessage::HandleImageUrl(url.clone())})
                    .into()
            }
    }
}