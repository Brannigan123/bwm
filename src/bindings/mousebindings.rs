use std::collections::HashMap;
use x11rb::rust_connection::{DefaultStream, RustConnection};

use penrose::{
    core::bindings::{MouseEvent, MouseEventKind, MouseState},
    x11rb::X11rbConnection,
    Backward, Forward, PenroseError, WindowManager,
};

pub fn get_mousebindings() -> HashMap<
    (MouseEventKind, MouseState),
    Box<
        dyn FnMut(
            &mut WindowManager<X11rbConnection<RustConnection<DefaultStream>>>,
            &MouseEvent,
        ) -> Result<(), PenroseError>,
    >,
> {
    gen_mousebindings! {
        Press Right + [Meta] => |wm: &mut WindowManager<X11rbConnection<RustConnection<DefaultStream>>>, _: &MouseEvent| wm.cycle_workspace(Forward),
        Press Left + [Meta] => |wm: &mut WindowManager<X11rbConnection<RustConnection<DefaultStream>>>, _: &MouseEvent| wm.cycle_workspace(Backward)
    }
}
