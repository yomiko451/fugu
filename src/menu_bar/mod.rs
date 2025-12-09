use iced::{widget::{button, container, row, Container}, Background, Element, Length, Padding, Task};
use iced_aw::{menu::Item, Menu, MenuBar as AWMenuBar};

use crate::constants::{MENU_BAR_AND_STATUS_BAR_BG_COLOR, MENU_ITEM_SPACING, MENU_WIDTH, PADDING, SPACING};



#[derive(Debug, Clone)]
pub struct MenuBar {
    
}

#[derive(Debug, Clone)]
pub enum MenuBarMessage {
    
}

impl MenuBar {
    pub fn new() -> Self {
        Self{}
    }
    
    pub fn update(&mut self, menu_bar_message: MenuBarMessage) -> Task<MenuBarMessage> {
        match menu_bar_message {
            _ => Task::none()
        }
    }
    
    pub fn view(&self) -> Container<'_, MenuBarMessage> {
        container(
            self.create_menu_bar()
        ).style(|_| {
            container::Style {
                background: Some(Background::Color(MENU_BAR_AND_STATUS_BAR_BG_COLOR)),
                ..container::Style::default()
            }
        })
    }
    
    pub fn create_menu_bar(&self) -> Element<'_, MenuBarMessage> {
        let file_menu = Item::with_menu(
            button("文件(T)"),
            Menu::new(
                vec![
                    Item::new(button("新建文件").width(Length::Fill)),
                    Item::new(button("打开文件").width(Length::Fill)),
                    Item::new(button("保存文件").width(Length::Fill)),
                    Item::new(button("文件另存为").width(Length::Fill))
                ]
            )
            .width(MENU_WIDTH)
            .spacing(MENU_ITEM_SPACING),
        );

        let edit_menu = Item::with_menu(
            button("编辑(E)"),
            Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
                .width(MENU_WIDTH),
        );

        let view_menu = Item::with_menu(
            button("视图(V)"),
            Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
                .width(MENU_WIDTH),
        );

        let tool_menu = Item::with_menu(
            button("设置(S)"),
            Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
                .width(MENU_WIDTH),
        );

        let help_menu = Item::with_menu(
            button("帮助(H)"),
            Menu::new([Item::new("open"), Item::new("save"), Item::new("close")].into())
                .width(MENU_WIDTH),
        );

        AWMenuBar::new(vec![file_menu, edit_menu, view_menu, tool_menu, help_menu])
            .width(Length::Shrink)
            .spacing(SPACING)
            .padding(Padding::from([0, PADDING]))
            .into()
    }
}