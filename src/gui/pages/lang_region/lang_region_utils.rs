use chrono::prelude::*;
use iced::{button, pick_list, scrollable, text_input};
use smart_default::SmartDefault;
use std::fmt::{self, Display, Formatter};
use iced_custom_widget::Icons;
#[derive(Debug, Clone, SmartDefault)]
pub struct GeneralTab {
    pub prefered_langs: Vec<(PreferedLang, button::State)>,
    pub selected_lang: Option<usize>,
    pub prefered_lang_scroll: scrollable::State,
    pub add_state: button::State,
    pub remove_state: button::State,
    pub up_state: button::State,
    pub down_state: button::State,
    // right section
    pub region_state: pick_list::State<LCKeyVal>,
    pub selected_region: Option<LCKeyVal>,
    pub firstday_state: pick_list::State<String>,
    pub selected_firstday: Option<String>,
    pub time_format: pick_list::State<LCKeyVal>,
    pub selected_time_format: Option<LCKeyVal>,
    pub is_24_hours_format: bool,
    pub num_format: pick_list::State<LCKeyVal>,
    pub selected_num_format: Option<LCKeyVal>,
    pub currency_format: pick_list::State<LCKeyVal>,
    pub selected_currency_format: Option<LCKeyVal>,
    pub measure_format: pick_list::State<LCKeyVal>,
    pub selected_measure_format: Option<LCKeyVal>,
    pub content_scroll: scrollable::State,
    #[default(Local.ymd(1991, 10, 23).and_hms(9, 0, 0))]
    pub now: DateTime<Local>,
    #[default(12345)]
    pub number: i32,
    #[default(56789)]
    pub currency: i32,
    // add prefered lang section
    pub is_adding: bool,
    pub add_langs: Vec<(PreferedLang, button::State)>,
    pub filtered_add_langs: Vec<(PreferedLang, button::State)>,
    pub search_prefered_lang_state: text_input::State,
    pub search_prefered_lang_val: String,
    pub selected_add_lang: Option<PreferedLang>,
    pub btn_okay_state: button::State,
    pub btn_cancel_state: button::State,
}

impl GeneralTab {
    pub(super) fn get_formatted_prefered_lang(&self) -> String {
        self.prefered_langs
            .iter()
            .map(|(prefered_lang, ..)| *prefered_lang.key.split(".").collect::<Vec<&str>>().first().unwrap())
            .collect::<Vec<&str>>()
            .join(":")
            .to_owned()
    }
}

#[derive(Debug, Clone, Default)]
pub struct AppsTab {
    pub app_list: Vec<(Icons, String, pick_list::State<String>, String, button::State)>,
    pub selected_app: Option<usize>,
    pub add_state: button::State,
    pub remove_state: button::State,
    pub scroll: scrollable::State,
}

use std::cmp::Ordering;

#[derive(Debug, Clone, Default, Eq, PartialEq, PartialOrd)]
pub struct LCKeyVal {
    pub key: String,
    pub val: String,
}

impl LCKeyVal {
    pub(super) fn new<T: Into<String>>(key_val: (T, T)) -> Self {
        Self { key: key_val.0.into(), val: key_val.1.into() }
    }
}

impl Display for LCKeyVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Ord for LCKeyVal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq)]
pub struct PreferedLang {
    pub key: String,
    pub lang: String,
    pub reg: String,
}

impl PreferedLang {
    pub fn new(key: &str, lang: &str, reg: &str) -> Self {
        Self {
            key: key.to_owned(),
            lang: lang.to_owned(),
            reg: reg.to_owned(),
        }
    }
}

impl Display for PreferedLang {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} — {}", self.lang, self.reg)
    }
}

impl Ord for PreferedLang {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
