use iced::{
    border::Radius, mouse, widget::{container, mouse_area, text, Container, MouseArea}, Background, Border, Color, Element, Length, Padding, Task, Theme
};
use iced_aw::{Menu, MenuBar as AWMenuBar, menu::Item};

use crate::constants::*;

#[derive(Debug, Clone)]
pub struct MenuBar {
    hovered_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum MenuBarMessage {
    HoverEnter(usize),
    None,
    FileMenuOpenFolder,
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
        let file_menu = Item::with_menu(
            mouse_area(text("文件(T)").size(FONT_SIZE_BASE))
                .interaction(mouse::Interaction::Pointer),
            Menu::new(
                [
                    ("新建文件", MenuBarMessage::None),
                    ("打开文件", MenuBarMessage::None),
                     ("打开文件夹", MenuBarMessage::FileMenuOpenFolder),
                    ("文件另存为", MenuBarMessage::None),
                ]
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
        .close_on_click(true);

        // let edit_menu = Item::with_menu(
        //     text("编辑(E)").size(FONT_SIZE_BIGGER),
        //     Menu::new(
        //         [
        //             Item::new(mouse_area(text("aaa")).on_press(MenuBarMessage::None)),
        //             Item::new("save"),
        //             Item::new("close"),
        //         ]
        //         .into(),
        //     )
        //     .width(MENU_WIDTH),
        // );

        // let view_menu = Item::with_menu(
        //     text("视图(V)").size(FONT_SIZE_BIGGER),
        //     Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
        //         .width(MENU_WIDTH),
        // );

        // let tool_menu = Item::with_menu(
        //     text("设置(S)").size(FONT_SIZE_BIGGER),
        //     Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
        //         .width(MENU_WIDTH),
        // );

        // let help_menu = Item::with_menu(
        //     text("帮助(H)").size(FONT_SIZE_BIGGER),
        //     Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
        //         .width(MENU_WIDTH),
        // );

        AWMenuBar::new(vec![file_menu, ])
            .width(Length::Shrink)
            .style(|theme: &Theme, _| {
                let ex_palette = theme.extended_palette();
                iced_aw::menu::Style {
                    menu_background: Background::Color(ex_palette.background.weakest.color),
                    bar_background: Background::Color(ex_palette.background.weaker.color),
                    ..iced_aw::menu::Style::default()
                }
            })
            .spacing(SPACING)
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
                        radius: Radius::new(BORDER_RADIUS)
                    })
                }),
        )
        .on_enter(MenuBarMessage::HoverEnter(id))
        .interaction(mouse::Interaction::Pointer)
        .on_press(message)
    }
}
