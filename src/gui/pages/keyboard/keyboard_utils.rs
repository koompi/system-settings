use super::add_input_source_sec::AddInputSrcSec;
use super::conf_input_source_sec::ConfigInputSrcSec;
use iced::{button, pick_list, scrollable, slider};
use iced_custom_widget::Icons;
use smart_default::SmartDefault;
#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum TurnBacklightOff {
    #[default]
    _5s,
    _10s,
    _30s,
    _1m,
    _5m,
}

impl TurnBacklightOff {
    pub const ALL: [TurnBacklightOff; 5] = [TurnBacklightOff::_5s, TurnBacklightOff::_10s, TurnBacklightOff::_30s, TurnBacklightOff::_1m, TurnBacklightOff::_5m];
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
                TurnBacklightOff::_5m => "5 mins",
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
    pub shortcuts_tab: Vec<(Icons, &'static str, button::State)>,
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
                (Icons::User, "Menu & Dock", button::State::new()),
                (Icons::User, "Workspaces", button::State::new()),
                (Icons::User, "Keyboard", button::State::new()),
                (Icons::User, "Input Sources", button::State::new()),
                (Icons::User, "Screenshots", button::State::new()),
                (Icons::User, "Services", button::State::new()),
                (Icons::User, "Spotlight", button::State::new()),
            ],
            shortcuts_tab_map: vec![
                vec![(true, "Turn Dock Hiding On/Off", "shift+ctrl+D"), (false, "Show Menu", "")],
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
                vec![(false, "Select the previous source", "shift+Space"), (false, "Select next source in Input menu", "ctrl+shift+Space")],
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
                vec![(false, "Select the previous source", "shift+Space"), (false, "Select next source in Input menu", "ctrl+shift+Space")],
            ],
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputSources {
    pub btn_add_state: button::State,
    pub btn_remove_state: button::State,
    pub btn_up_state: button::State,
    pub btn_down_state: button::State,
    pub btn_config_state: button::State,
    pub input_sources: Vec<(char, String, button::State)>,
    pub input_sources_selected: Option<usize>,
    pub show_input_menu: bool,
    pub auto_switch: bool,
    // add input source
    pub is_adding: bool,
    pub add_input_source_sec: AddInputSrcSec,
    // config input source
    pub is_config: bool,
    pub config_input_source_sec: ConfigInputSrcSec,
    pub left_pane_scroll: scrollable::State,
    pub right_pane_scroll: scrollable::State,
}

impl InputSources {
    pub fn new() -> Self {
        Self {
            input_sources: vec![('\u{f57e}', "Khmer".to_string(), button::State::new()), ('\u{f0ac}', "English".to_string(), button::State::new())],
            add_input_source_sec: AddInputSrcSec::new(),
            config_input_source_sec: ConfigInputSrcSec::new(),
            show_input_menu: true,
            auto_switch: false,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GlobalOptions {
    pub hotkey_sec: HotKey,
    pub behavior_sec: Behavior,
}

#[derive(Debug, Clone, Default)]
pub struct HotKey {
    pub toggle_inp_src_state: pick_list::State<String>,
    pub toggle_inp_src_val: Option<String>,
    pub show_press_toggle_repeat: bool,
    pub temp_switch_first_n_cur_inp_src_state: pick_list::State<String>,
    pub temp_switch_first_n_cur_inp_src_val: Option<String>,
    pub switch_inp_src_fw_state: pick_list::State<String>,
    pub switch_inp_src_fw_val: Option<String>,
    pub switch_inp_src_bw_state: pick_list::State<String>,
    pub switch_inp_src_bw_val: Option<String>,
    pub skip_first_inp_src_switch: bool,
    pub act_inp_src_state: pick_list::State<String>,
    pub act_inp_src_val: Option<String>,
    pub deact_inp_src_state: pick_list::State<String>,
    pub deact_inp_src_val: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Behavior {
    pub act_by_def: bool,
    pub share_inp_state: pick_list::State<String>,
    pub share_inp_state_val: Option<String>,
    pub switch_show_inp_src_info: bool,
    pub change_focus_show_inp_src_info: bool,
}

#[allow(non_upper_case_globals)]
impl GlobalOptions {
    pub const hotkey_opts: [&'static str; 6] = ["Control+Left Shift", "Control+Right Shift", "Super+Space", "Shift+Super+Space", "Shift+Tab", "Control+Alt+Space"];
    pub const share_inp_state_opt: [&'static str; 3] = ["All", "Application", "No"];

    pub fn new() -> Self {
        Self {
            hotkey_sec: HotKey {
                toggle_inp_src_val: Some(Self::hotkey_opts[0].to_owned()),
                show_press_toggle_repeat: true,
                switch_inp_src_fw_val: Some(Self::hotkey_opts[0].to_owned()),
                switch_inp_src_bw_val: Some(Self::hotkey_opts[1].to_owned()),
                skip_first_inp_src_switch: false,
                act_inp_src_val: Some(Self::hotkey_opts[2].to_owned()),
                deact_inp_src_val: Some(Self::hotkey_opts[3].to_owned()),
                ..HotKey::default()
            },
            behavior_sec: Behavior {
                act_by_def: false,
                share_inp_state_val: Some(Self::share_inp_state_opt[2].to_owned()),
                switch_show_inp_src_info: true,
                change_focus_show_inp_src_info: false,
                ..Behavior::default()
            },
        }
    }
}
