#[macro_use]
extern crate penrose;

pub mod bindings;
pub mod colors;
pub mod commands;
pub mod layouts;
pub mod penrose_config;
pub mod top_bar;

use penrose::{
    logging_error_handler,
    x11rb::{new_x11rb_rust_backed_window_manager, X11rbConnection},
    Result as PenroseResult, WindowManager,
};
use simplelog::{LevelFilter, SimpleLogger};
use std::{
    io::Error,
    process::{Command, Stdio},
};
use x11rb::rust_connection::{DefaultStream, RustConnection};

pub fn main() -> PenroseResult<()> {
    init_logger();

    let config = penrose_config::config::get_config().unwrap_or_default();
    let mut wm = new_x11rb_rust_backed_window_manager(config, vec![], logging_error_handler())?;

    set_wm_name(&wm, "blessed-wm");

    wm.grab_keys_and_run(
        bindings::keybindings::get_keybindings(),
        bindings::mousebindings::get_mousebindings(),
    )?;

    Ok(())
}

fn init_logger() {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };
}

fn set_wm_name(wm: &WindowManager<X11rbConnection<RustConnection<DefaultStream>>>, name: &str) {
    wm.set_root_window_name(name).ok();
    match get_wm_id() {
        Ok(id) => update_wm_name(id.as_str(), name),
        Err(_) => {}
    }
}

fn get_wm_id() -> Result<String, Error> {
    Command::new("xprop")
        .arg("-root")
        .arg("-notype")
        .arg("_NET_SUPPORTING_WM_CHECK")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .split_ascii_whitespace()
                .last()
                .unwrap_or("0x0")
                .to_string()
        })
}

fn update_wm_name(id: &str, name: &str) {
    Command::new("xprop")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-f")
        .arg("MY_VAR1")
        .arg("8s")
        .arg("-set")
        .arg("WM_NAME")
        .arg(name)
        .arg("-id")
        .arg(format!("{}", id))
        .output()
        .ok();
}
