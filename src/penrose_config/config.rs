use crate::colors;
use crate::layouts;
use crate::top_bar;

use penrose::{
    core::{
        config::Config,
        layout::{monocle, side_stack, Layout},
    },
    PenroseError,
};

use layouts::custom::{spiral_horizontal_split_first, split_longsided};

pub fn get_config() -> Result<Config, PenroseError> {
    let default = Config::default();
    let mut builder = Config::builder(&default);
    Ok(builder
        .bar_height(top_bar::consts::BAR_HEIGHT)
        .border_px(layouts::consts::BORDER_SIZE)
        .gap_px(layouts::consts::GAPS_SIZE)
        .floating_classes(layouts::consts::FLOATING)
        .focused_border(colors::consts::FOCUSED_BORDER)?
        .unfocused_border(colors::consts::UNFOCUSED_BORDER)?
        .layouts(vec![
            Layout::new(
                "[spiral]",
                layouts::consts::DEFAULT_LAYOUT_CONFIG,
                spiral_horizontal_split_first,
                1,
                layouts::consts::MAIN_AREA_RATIO,
            ),
            Layout::new(
                "[split]",
                layouts::consts::DEFAULT_LAYOUT_CONFIG,
                split_longsided,
                1,
                layouts::consts::MAIN_AREA_RATIO,
            ),
            Layout::new(
                "[side]",
                layouts::consts::DEFAULT_LAYOUT_CONFIG,
                side_stack,
                1,
                layouts::consts::MAIN_AREA_RATIO,
            ),
            Layout::new(
                "[mono]",
                layouts::consts::DEFAULT_LAYOUT_CONFIG,
                monocle,
                1,
                layouts::consts::MAIN_AREA_RATIO,
            ),
            Layout::floating("[----]"),
        ])
        .build()
        .unwrap_or_default())
}
