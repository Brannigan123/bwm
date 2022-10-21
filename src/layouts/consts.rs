use penrose::core::layout::LayoutConf;

pub const FLOATING: [&str; 3] = ["dunst", "polybar", "dmenu"];
pub const BORDER_SIZE: u32 = 3;
pub const GAPS_SIZE: u32 = 4;

pub const MAIN_AREA_RATIO: f32 = 0.6;

pub const DEFAULT_LAYOUT_CONFIG: LayoutConf = LayoutConf {
    floating: false,
    gapless: false,
    follow_focus: true,
    allow_wrapping: true,
};
