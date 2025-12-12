use std::{path::PathBuf, sync::LazyLock};
use iced::{Color, Theme, theme::Palette};
// 这里定义各种公共类型
#[derive(Debug, Clone)]
pub struct FileData {
    pub name: String,
    pub content: String,
    pub path: PathBuf,
    pub is_saved: bool,
}


//这里主要定义各种常量
// 
// 
// app相关常量
// 
// app名称
pub const APP_NAME: &str = "fugu";
// 默认窗口大小
pub const DEFAULT_WINDOW_SIZE: [f32; 2] = [1280., 720.];
// 默认字体大小
pub const FONT_SIZE_BIGGEST: u32 = 20;
pub const FONT_SIZE_BIGGER: u32 = 18;
pub const FONT_SIZE_BASE: u32 = 16;
pub const FONT_SIZE_SMALLER: u32 = 14;
pub const FONT_SIZE_SMALLEST: u32 = 12;
// 默认内边距
pub const PADDING_BIGGER: u16 = 20;
pub const PADDING_BASE: u16 = 10;
pub const PADDING_SMALLER: u16 = 5;
pub const PADDING_SMALLEST: u16 = 2;
// 默认间距
pub const SPACING: u32 = 10;
//默认边框弧度
pub const BORDER_RADIUS: f32 = 5.;
//默认边框宽度
pub const BORDER_WIDTH: f32 = 1.;
// 默认背景色
const BACKGROUND_COLOR: Color = Color::from_rgb8(40, 44, 51);
// 默认文本色
const TEXT_COLOR: Color = Color::from_rgb8(171, 178, 191);
// 默认主色
const PRIMARY_COLOR: Color = Color::from_rgb8(56, 178, 172);
// 默认成功状态色
const SUCCESS_COLOR: Color = Color::from_rgb(0.52, 0.63, 0.50);
// 默认警告状态色
const WARNING_COLOR: Color = Color::from_rgb(0.83, 0.69, 0.42);
// 默认危险状态色
const DANGER_COLOR: Color = Color::from_rgb(0.78, 0.54, 0.49);
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
pub const TEXT_INDENTATION: u16 = 15;
// 背景颜色
//pub const FILE_PANEL_BG_COLOR: Color = Color::from_rgb8(47, 52, 62);
// 默认文字大小




// 菜单栏/状态栏相关常量
//pub const MENU_BAR_AND_STATUS_BAR_BG_COLOR: Color = Color::from_rgb8(59, 65, 77);

pub const MENU_WIDTH: f32 = 150.;
pub const MENU_OFFSET: f32 = 5.;

// 编辑区相关常量
//pub const EDITOR_BG_COLOR: Color = Color::from_rgb8(40, 44, 51);
// 预览区相关常量
//pub const PREVIEW_BG_COLOR: Color = Color::from_rgb8(47, 52, 62);









