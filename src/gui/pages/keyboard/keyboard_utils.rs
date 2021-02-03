use smart_default::SmartDefault;
use iced::{
   slider, button, scrollable, pick_list
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum TurnBacklightOff {
   #[default]
   _5s,
   _10s,
   _30s,
   _1m,
   _5m
}

impl TurnBacklightOff {
   pub const ALL: [TurnBacklightOff; 5] = [
      TurnBacklightOff::_5s,
      TurnBacklightOff::_10s,
      TurnBacklightOff::_30s,
      TurnBacklightOff::_1m,
      TurnBacklightOff::_5m
   ];
}

impl std::fmt::Display for TurnBacklightOff {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            TurnBacklightOff::_5s => "5 secs",
            TurnBacklightOff::_10s => "10 secs",
            TurnBacklightOff::_30s => "30 secs",
            TurnBacklightOff::_1m => "1 min",
            TurnBacklightOff::_5m => "5 mins"  
         }
      )
   }
}

#[derive(Debug, Clone, Default)]
pub struct Keyboard {
   pub key_repeat_state: slider::State,
   pub key_repeat_val: u8,
   pub delay_repeat_state: slider::State,
   pub delay_repeat_val: u8,
   pub adjust_brightness_low_light: bool,
   pub turn_backlight_off: bool,
   pub turn_backlight_off_after_state: pick_list::State<TurnBacklightOff>,
   pub turn_backlight_off_after_val: TurnBacklightOff,
}

impl Keyboard {
   pub fn new() -> Self {
      Self {
         key_repeat_val: 7,
         delay_repeat_val: 3,
         adjust_brightness_low_light: true,
         turn_backlight_off: false,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct Shortcuts {
   pub btn_restore: button::State,
   pub shortcuts_tab: Vec<(char, &'static str, button::State)>,
   pub shortcuts_tab_map: Vec<Vec<(bool, &'static str, &'static str)>>,
   pub left_pane_selected: usize,
   pub right_pane_selected: usize,
   pub use_keyboard_nav: bool,
   pub left_pane_scroll: scrollable::State,
   pub right_pane_scroll: scrollable::State,
}

impl Shortcuts {
   pub fn new() -> Self {
      Self {
         shortcuts_tab: vec![
            ('\u{f86d}', "Menu & Dock", button::State::new()), 
            ('\u{f86d}', "Workspaces", button::State::new()),
            ('\u{f11c}', "Keyboard", button::State::new()), 
            ('\u{f11c}', "Input Sources", button::State::new()), 
            ('\u{f083}', "Screenshots", button::State::new()), 
            ('\u{f552}', "Services", button::State::new()), 
            ('\u{f002}', "Spotlight", button::State::new()), 
         ],
         shortcuts_tab_map: vec![
            vec![
               (true, "Turn Dock Hiding On/Off", "shift+ctrl+D"),
               (false, "Show Menu", ""),
            ],
            vec![
               (true, "Workspaces", "shift+UP"),
               (false, "Show Notification Center", ""),
               (true, "Turn Do Not Disturb On/Off", ""),
               (true, "Application Windows", "shift+DOWN"),
               (true, "Show Desktop", "F11"),
            ],
            vec![
               (true, "Change the way Tab moves focus", "shift+F7"),
               (true, "Turn keyboard access On/Off", "shift+F1"),
               (true, "Move focus to menu bar", "shift+F2"),
               (true, "Move focus to Dock", "shift+F3"),
               (true, "Move focus to Window toolbar", "shift+F4"),
               (true, "Move focus to next Window", "ctrl+`"),
               (true, "Move focus to window drawer", "ctrl+shift+`"),
               (true, "Move focus to status menus", "shift+F5"),
            ],
            vec![
               (false, "Select the previous source", "shift+Space"),
               (false, "Select next source in Input menu", "ctrl+shift+Space"),
            ],
            vec![
               (true, "Workspaces", "shift+UP"),
               (false, "Show Notification Center", ""),
               (true, "Turn Do Not Disturb On/Off", ""),
               (true, "Application Windows", "shift+DOWN"),
               (true, "Show Desktop", "F11"),
            ],
            vec![
               (true, "Change the way Tab moves focus", "shift+F7"),
               (true, "Turn keyboard access On/Off", "shift+F1"),
               (true, "Move focus to menu bar", "shift+F2"),
               (true, "Move focus to Dock", "shift+F3"),
               (true, "Move focus to Window toolbar", "shift+F4"),
               (true, "Move focus to next Window", "ctrl+`"),
               (true, "Move focus to window drawer", "ctrl+shift+`"),
               (true, "Move focus to status menus", "shift+F5"),
            ],
            vec![
               (false, "Select the previous source", "shift+Space"),
               (false, "Select next source in Input menu", "ctrl+shift+Space"),
            ],
         ],
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct InputSources {
   pub btn_add_state: button::State, 
   pub btn_remove_state: button::State, 
   pub input_sources_tab: Vec<(char, String, button::State)>,
   pub input_sources_selected: Option<usize>,
   pub show_input_menu: bool,
   pub auto_switch: bool,
   pub left_pane_scroll: scrollable::State,
   pub right_pane_scroll: scrollable::State,
}

impl InputSources {
   pub fn new() -> Self {
      Self {
         input_sources_tab: vec![
            ('\u{f0ac}', "English".to_string(), button::State::new()),
            ('\u{f57e}', "Khmer".to_string(), button::State::new()),
         ],
         input_sources_selected: Some(1),
         show_input_menu: true,
         auto_switch: false,
         ..Default::default()
      }
   }
}

// #[derive(Debug, Clone, Default)]
// pub struct Dictation {
//    btn_about: button::State, 
//    turn_on_dict: bool,
//    language_state: pick_list::State<Language>,
//    language_val: Language,
//    shortcut_state: pick_list::State<ShortcutDict>,
//    shortcut_val: ShortcutDict,
// }

// impl Dictation {
//    pub fn new() -> Self {
//       Self::default()
//    }
// }

// #[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
// pub enum Language {
//    English,
//    #[default]
//    Khmer,
//    AddNew
// }

// impl Language {
//    const ALL: [Language; 3] = [
//       Language::English,
//       Language::Khmer,
//       Language::AddNew,
//    ];
// }

// impl std::fmt::Display for Language {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       write!(
//          f,
//          "{}",
//          match self {
//             Language::English => "English (US)",
//             Language::Khmer => "Khmer",
//             Language::AddNew => "Add Language...",
//          }
//       )
//    }
// }

// #[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
// pub enum ShortcutDict {
//    Off,
//    #[default]
//    CtrlTwice,
//    FnTwice,
//    Customize
// }

// impl ShortcutDict {
//    const ALL: [ShortcutDict; 4] = [
//       ShortcutDict::Off,
//       ShortcutDict::CtrlTwice,
//       ShortcutDict::FnTwice,
//       ShortcutDict::Customize,
//    ];
// }

// impl std::fmt::Display for ShortcutDict {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       write!(
//          f,
//          "{}",
//          match self {
//             ShortcutDict::Off => "Off",
//             ShortcutDict::CtrlTwice => "Press Ctrl Key Twice",
//             ShortcutDict::FnTwice => "Press Fn Key Twice",
//             ShortcutDict::Customize => "Customize...",
//          }
//       )
//    }
// }