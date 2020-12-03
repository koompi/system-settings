pub mod gui;

use gui::SystemSetting;

fn main() -> iced::Result {
   SystemSetting::init()
}
