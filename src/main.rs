#[macro_use]
pub mod helpers;
pub mod gui;

use gui::SystemSetting;

fn main() -> iced::Result {
    std::env::set_var("WINIT_X11_SCALE_FACTOR", "1.2");
    SystemSetting::init()
}
