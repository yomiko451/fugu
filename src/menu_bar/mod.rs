use iced::{
    Background, Border, Color, Element, Length, Padding, Renderer, Task, Theme,
    border::Radius,
    mouse,
    widget::{Container, MouseArea, container, mouse_area, text},
};
use iced_aw::{Menu, menu::Item};

use crate::common::*;

#[derive(Debug, Clone)]
pub struct MenuBar {
    hovered_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum MenuBarMessage {
    HoverEnter(usize),
    None,
    CommandOpenFolder,
    CommandOpenFile,
    CommandCreateNewFile,
    CommandSaveFile,
}

impl MenuBar {
    pub fn new() -> Self {
        Self { hovered_id: None }
    }

    pub fn update(&mut self, menu_bar_message: MenuBarMessage) -> Task<MenuBarMessage> {
        match menu_bar_message {
            MenuBarMessage::HoverEnter(id) => {
                self.hovered_id = Some(id);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Container<'_, MenuBarMessage> {
        container(self.create_menu_bar())
            .padding(Padding::from([PADDING_SMALLER, PADDING_BIGGER]))
            .style(|theme: &Theme| {
                let ex_palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(ex_palette.background.weaker.color)),
                    ..container::Style::default()
                }
            })
    }

    pub fn create_menu_bar(&self) -> Element<'_, MenuBarMessage> {
        let file_menu = self.generate_menu(
            "文件(F)",
            vec![
                ("新建文件", MenuBarMessage::CommandCreateNewFile),
                ("打开文件", MenuBarMessage::CommandOpenFile),
                ("保存文件", MenuBarMessage::CommandSaveFile),
                ("打开文件夹", MenuBarMessage::CommandOpenFolder),
                ("文件另存为", MenuBarMessage::None),
            ],
        );

        let edit_menu = self.generate_menu(
            "编辑(E)",
            vec![
                ("撤销", MenuBarMessage::None),
                ("重做", MenuBarMessage::None),
                ("剪切", MenuBarMessage::None),
                ("复制", MenuBarMessage::None),
                ("粘贴", MenuBarMessage::None),
                ("删除", MenuBarMessage::None),
                ("全选", MenuBarMessage::None),
            ],
        );

        let view_menu = self.generate_menu(
            "视图(V)",
            vec![
                ("预览窗口", MenuBarMessage::None),
                ("快照窗口", MenuBarMessage::None),
                ("日志窗口", MenuBarMessage::None),
                ("剪切板窗口", MenuBarMessage::None),
            ],
        );
        
        let tool_menu = self.generate_menu(
            "工具(T)",
            vec![
                ("创建快照", MenuBarMessage::None),
                ("恢复快照", MenuBarMessage::None),
                ("删除快照", MenuBarMessage::None),
            ],
        );

        let setting_menu = self.generate_menu(
            "设置(S)",
            vec![
                ("自动保存", MenuBarMessage::None),
                ("快照窗口", MenuBarMessage::None),
                ("日志窗口", MenuBarMessage::None),
                ("剪切板窗口", MenuBarMessage::None),
            ],
        );

        let help_menu = self.generate_menu(
            "帮助(H)",
            vec![
                ("预览窗口", MenuBarMessage::None),
                ("快照窗口", MenuBarMessage::None),
                ("日志窗口", MenuBarMessage::None),
                ("剪切板窗口", MenuBarMessage::None),
            ],
        );

        iced_aw::MenuBar::new(vec![file_menu, edit_menu, view_menu, tool_menu, setting_menu, help_menu])
            .width(Length::Shrink)
            .style(|theme: &Theme, _| {
                let ex_palette = theme.extended_palette();
                iced_aw::menu::Style {
                    menu_background: Background::Color(ex_palette.background.weakest.color),
                    bar_background: Background::Color(ex_palette.background.weaker.color),
                    ..iced_aw::menu::Style::default()
                }
            })
            .spacing(SPACING_BIGGER)
            .close_on_item_click_global(true)
            .close_on_background_click_global(true)
            .into()
    }

    pub fn generate_menu_item(
        &self,
        menu_text: String,
        id: usize,
        message: MenuBarMessage,
    ) -> MouseArea<'_, MenuBarMessage> {
        mouse_area(
            container(text(menu_text).width(Length::Fill).size(FONT_SIZE_BASE))
                .padding(Padding::from([PADDING_SMALLER, PADDING_BASE]))
                .style(move |theme: &Theme| {
                    let ex_palette = theme.extended_palette();
                    let style = if self.hovered_id == Some(id) {
                        container::Style {
                            background: Some(Background::Color(ex_palette.background.weaker.color)),
                            ..container::Style::default()
                        }
                    } else {
                        container::Style {
                            background: Some(Background::Color(
                                ex_palette.background.weakest.color,
                            )),
                            ..container::Style::default()
                        }
                    };
                    style.border(Border {
                        color: ex_palette.background.weakest.color,
                        width: BORDER_WIDTH,
                        radius: Radius::new(BORDER_RADIUS),
                    })
                }),
        )
        .on_enter(MenuBarMessage::HoverEnter(id))
        .interaction(mouse::Interaction::Pointer)
        .on_press(message)
    }

    pub fn generate_menu(
        &self,
        label: &'static str,
        sub_item: Vec<(&'static str, MenuBarMessage)>,
    ) -> Item<'_, MenuBarMessage, Theme, Renderer> {
        Item::with_menu(
            mouse_area(text(label).size(FONT_SIZE_BIGGER)).interaction(mouse::Interaction::Pointer),
            Menu::new(
                sub_item
                    .into_iter()
                    .enumerate()
                    .map(|(id, (menu_text, message))| {
                        let item = self.generate_menu_item(menu_text.to_string(), id, message);
                        Item::new(item)
                    })
                    .collect(),
            )
            .offset(MENU_OFFSET)
            .padding(PADDING_SMALLEST)
            .width(MENU_WIDTH),
        )
        .close_on_click(true)
    }
}
