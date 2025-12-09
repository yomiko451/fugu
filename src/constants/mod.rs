use std::{str::FromStr, sync::LazyLock};
use iced::{Color, Theme, theme::Palette};
//这里主要定义各种常量
// 
// 
// app相关常量
// 
// app名称
pub const APP_NAME: &str = "fugu";
// 默认窗口大小
pub const DEFAULT_WINDOW_SIZE: [f32; 2] = [1280., 720.];
// 默认内边距
pub const PADDING: u16 = 10;
// 默认间距
pub const SPACING: u32 = 10;
//默认边框弧度
pub const BORDER_RADIUS: f32 = 5.;
//默认边框宽度
pub const BORDER_WIDTH: f32 = 1.;
// 默认背景色
pub const BACKGROUND_COLOR: Color = Color::TRANSPARENT;
// 默认文本色
pub const TEXT_COLOR: Color = Color::from_rgb(0.83, 0.83, 0.85);
// 默认主色
pub const PRIMARY_COLOR: Color = Color::from_rgb(0.55, 0.65, 0.58);
// 默认成功状态色
pub const SUCCESS_COLOR: Color = Color::from_rgb(0.52, 0.63, 0.50);
// 默认警告状态色
pub const WARNING_COLOR: Color = Color::from_rgb(0.83, 0.69, 0.42);
// 默认危险状态色
pub const DANGER_COLOR: Color = Color::from_rgb(0.78, 0.54, 0.49);
// 默认主题
pub const DEFAULT_THEME: LazyLock<Theme> = LazyLock::new(||{
    let palette = Palette {
        background: BACKGROUND_COLOR,
        text: TEXT_COLOR,
        primary: PRIMARY_COLOR,
        success: SUCCESS_COLOR,
        danger: DANGER_COLOR,
        warning: WARNING_COLOR
    };
    Theme::custom("FuguTheme", palette)
});

// 文件区相关常量
pub const FILE_PANEL_BG_COLOR: Color = Color::from_rgb8(47, 52, 62);
// 菜单栏/状态栏相关常量
pub const MENU_BAR_AND_STATUS_BAR_BG_COLOR: Color = Color::from_rgb8(59, 65, 77);


// 编辑区相关常量
pub const EDITOR_BG_COLOR: Color = Color::from_rgb8(40, 44, 51);
// 预览区相关常量
pub const PREVIEW_BG_COLOR: Color = Color::from_rgb8(47, 52, 62);







pub const CREATE_NEW_FILE_TEXT_INPUT_PLACEHOLDER: &str = "请输入文件名";


//菜单栏相关常量

pub const MENU_WIDTH: f32 = 200.;
pub const MENU_ITEM_SPACING: u32 = 2;