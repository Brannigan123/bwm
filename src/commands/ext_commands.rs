// AUdio
pub const TOGGLE_MUTE: &str = "pulsemixer --toggle-mute";
pub const VOLUME_UP: &str = "pulsemixer --change-volume +5";
pub const VOLUME_DOWN: &str = "pulsemixer --change-volume -5";
// Brightness
pub const BRIGHTNESS_UP: &str = "brightnessctl s 5%+";
pub const BRIGTHNESS_DOWN: &str = "brightnessctl s 5%-";
// Media Control
pub const NEXT_MEDIA: &str = "playerctl next";
pub const PREVIOUS_MEDIA: &str = "playerctl previous";
pub const TOGGLE_SHUFFLE_MEDIA: &str = "playerctl shuffle Toggle";
pub const TOGGLE_PLAY_PAUSE_MEDIA: &str = "playerctl play-pause";
pub const STOP_MEDIA: &str = "playerctl stop";
// Screenshots
pub const SCREEN_CAPTURE: &str = "flameshot screen -p ~/Pictures";
pub const DESKTOP_CAPTURE: &str = "flameshot full -p ~/Pictures";
// Applications
pub const TERMINAL: &str = "alacritty";
pub const LAUNCHER: &str = "rofi -show drun";
