use std::{path::PathBuf, sync::{Arc, LazyLock}};
use iced::{Border, Color, Font, Settings, Shadow, Theme, Vector, border::Radius, font, theme::Palette, widget::image};
// 这里定义各种公共类型
// FileData用于文件区和编辑区交互
#[derive(Debug, Clone)]
pub struct FileData {
    pub global_id: u32,
    pub version: u64,
    pub content: Arc<String>,
}

#[derive(Debug, Clone)]
pub struct ImgData {
    pub global_id: u32,
    pub indep_id: u32,
    pub handle: image::Handle,
}

// 包括各种App设定
#[derive(Debug, Clone)]
pub struct AppSetting {
    pub auto_save: bool,
}
// 全局错误类型
#[derive(Debug, Clone)]
pub enum AppError {
    FilePanelError(String),
    EditorError(String),
    PreviewError(String),
    MenuBarError(String),
    OtherError(String)
}
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::FilePanelError(error) => write!(f, "文件模块错误：{}", error),
            AppError::EditorError(error) => write!(f, "编辑模块错误：{}", error),
            AppError::PreviewError(error) => write!(f, "预览模块错误：{}", error),
            AppError::MenuBarError(error) => write!(f, "菜单模块错误：{}", error),
            AppError::OtherError(error) => write!(f, "其他错误：{}", error),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::FilePanelError(value.to_string())
    }
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
pub const PADDING_BIGGER: f32 = 20.;
pub const PADDING_BASE: f32 = 10.;
pub const PADDING_SMALLER: f32 = 5.;
pub const PADDING_SMALLEST: f32 = 2.;
// 默认间距
pub const SPACING_BIGGER: u32 = 20;
pub const SPACING: u32 = 10;
pub const SPACING_SMALLER: u32 = 5;
//默认边框弧度
pub const DEFAULT_BORDER: Border = Border {
    color: Color::TRANSPARENT,
    width: 1.,
    radius: Radius {
        top_left: 5.,
        top_right: 5.,
        bottom_left: 5.,
        bottom_right: 5.,
    }
};
// 默认背景色
const BACKGROUND_COLOR: Color = Color::from_rgb8(40, 44, 51);
// 默认文本色
const TEXT_COLOR: Color = Color::from_rgb8(171, 178, 191);
// 默认主色
const PRIMARY_COLOR: Color = Color::from_rgb8(152, 195, 121);
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
pub const SHADOW_BASE: Shadow = Shadow {
    color: Color::from_rgba(0., 0., 0., 0.5),
    blur_radius: 5.,
    offset: Vector::new(0., 1.)
};
pub const SHADOW_BASE_0_OFFSET: Shadow = Shadow {
    color: Color::from_rgba(0., 0., 0., 0.5),
    blur_radius: 10.,
    offset: Vector::ZERO
};
// 默认设置
pub const DEFAULT_USER_SETTING: AppSetting = AppSetting {
    auto_save: false
};
// 默认aaa设置
pub const DEFAULT_APP_SETTING: LazyLock<Settings> = LazyLock::new(|| {
    Settings {
        fonts: vec![
            include_bytes!("../../assets/NotoSansMonoCJKsc-VF.ttf").into()
        ],
        default_font: Font::with_name("Noto Sans Mono CJK SC"),
        ..Default::default()
    }
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
pub const TABLE_DIALOG_HEIGHT: u32 = 150;
pub const TABLE_DIALOG_WIDTH: u32 = 200;
pub const H1_COLOR: Color = Color::from_rgb8(229, 192, 123);
pub const H2_COLOR: Color = Color::from_rgb8(198, 120, 221);
pub const H3_COLOR: Color = Color::from_rgb8(97, 175, 239);
pub const H4_COLOR: Color = Color::from_rgb8(86, 182, 194);
pub const H5_COLOR: Color = Color::from_rgb8(199, 199, 199);
pub const H6_COLOR: Color = Color::from_rgb8(150, 150, 150);
pub const ORDERED_LSIT_COLOR: Color = Color::from_rgb8(209, 154, 102);
pub const CODE_BLOCK_TEXT_COLOR: Color = Color::from_rgb8(152, 195, 121);
pub const QUOTE_MARK_COLOR: Color = Color::from_rgb8(224, 108, 117);
// 预览区相关常量
//pub const PREVIEW_BG_COLOR: Color = Color::from_rgb8(47, 52, 62);









