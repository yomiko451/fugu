use std::sync::{Arc, OnceLock, mpsc::Receiver};

use crate::common::*;
use iced::{
    Element, Length, Padding, Subscription, Task, Theme,
    border::Radius,
    futures::SinkExt,
    mouse,
    widget::{Column, column, container, mouse_area, row, rule, scrollable, space, text},
};
use std::collections::VecDeque;
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt::{MakeWriter, time::ChronoLocal}};

static LOG_SENDER: OnceLock<UnboundedSender<String>> = OnceLock::new();

#[derive(Debug)]
pub struct LogViewer {
    log: VecDeque<String>,
}

#[derive(Debug, Clone)]
pub struct LogWriter;

impl std::io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        {
            if let Some(sender) = LOG_SENDER.get() {
                let _ = sender.send(s.to_string()); // 忽略错误
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for LogWriter {
    type Writer = LogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub enum LogViewerMessage {
    WriteLog(String),
}

impl LogViewer {
    pub fn new() -> Self {
        let timer_format = ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string());
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("fugu=info"))
            .with_ansi(false)
            .with_writer(LogWriter)
            .with_timer(timer_format)
            .init();

        Self {
            log: VecDeque::with_capacity(100),
        }
    }

    pub fn update(&mut self, message: LogViewerMessage) -> Task<LogViewerMessage> {
        match message {
            LogViewerMessage::WriteLog(new_log) => {
                if self.log.len() == 100 {
                    self.log.pop_front();
                }
                if !new_log.is_empty() {
                    self.log.push_back(new_log.trim_end().to_string());
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn view(&self) -> Element<'_, LogViewerMessage> {
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
            container(scrollable(
                Column::from_vec(
                    self.log
                        .iter()
                        .map(|log_item| { text(log_item).size(FONT_SIZE_SMALLER).into() })
                        .collect()
                )
                .width(Length::Fill)
                .spacing(SPACING_SMALLER)
            ))
            .height(Length::Fill)
            .padding(Padding::from([PADDING_BASE, PADDING_BIGGER]))
        ])
        .into()
    }

    pub fn subscription(&self) -> Subscription<LogViewerMessage> {
        Subscription::run(|| {
            iced::stream::channel(100, async move |mut output| {
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let _ = LOG_SENDER.set(tx); // 忽略错误
                while let Some(new_log) = rx.recv().await {
                    output
                        .send(LogViewerMessage::WriteLog(new_log))
                        .await
                        .unwrap()
                }
            })
        })
    }
}
