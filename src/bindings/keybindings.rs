use crate::commands;

use std::collections::HashMap;
use x11rb::rust_connection::{DefaultStream, RustConnection};

use penrose::{
    core::helpers::index_selectors, gen_keybindings, x11rb::X11rbConnection, Backward, Forward,
    Less, More, PenroseError, Selector, WindowManager, __test_helpers::KeyCode,
};

use commands::ext_commands::*;

pub fn get_keybindings() -> HashMap<
    KeyCode,
    Box<
        dyn FnMut(
            &mut WindowManager<X11rbConnection<RustConnection<DefaultStream>>>,
        ) -> Result<(), PenroseError>,
    >,
> {
    gen_keybindings! {

        // WM
        "M-S-q" => run_internal!(exit);

        // External programs
        "M-semicolon" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Brigtness Controll
        "XF86MonBrightnessUp" => run_external!(BRIGHTNESS_UP);
        "XF86MonBrightnessDown" => run_external!(BRIGHTNESS_UP);

        // Audio Control
        "XF86AudioMute" => run_external!(TOGGLE_MUTE);
        "XF86AudioLowerVolume" => run_external!(VOLUME_DOWN);
        "XF86AudioRaiseVolume" => run_external!(VOLUME_UP);

        // Media controll
        "XF86AudioNext" =>  run_external!(NEXT_MEDIA);
        "XF86AudioPrev" =>  run_external!(PREVIOUS_MEDIA);
        "XF86AudioStop" =>  run_external!(STOP_MEDIA);
        "XF86AudioPlay" =>  run_external!(TOGGLE_PLAY_PAUSE_MEDIA);

        // workspace management
        "M-BackSpace" => run_internal!(toggle_workspace);
        "M-Tab" => run_internal!(cycle_workspace, Forward);
        "M-S-Tab" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-Up" => run_internal!(cycle_layout, Forward);
        "M-Down" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        // Client management
        "M-Right" => run_internal!(cycle_client, Forward);
        "M-Left" => run_internal!(cycle_client, Backward);
        "M-C-Right" => run_internal!(drag_client, Forward);
        "M-C-Left" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-q" => run_internal!(kill_client);

        // workspace management
        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9", "0" } to index_selectors(10) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };

    }
}
