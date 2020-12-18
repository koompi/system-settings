use iced::{
   pick_list, slider, button, scrollable, Element, Align, Space, Length, Svg,
   Container, Checkbox, Row, Text, Button, Column, Scrollable, PickList, Slider, Radio,
};
use iced_custom_widget::Icon;
use vedas_core::svg;
use crate::helpers::ROOT_PATH;

use super::super::styles::{CustomButton, CustomContainer, CustomSlider, CustomCheckbox, CustomRadio, CustomSelect};
use smart_default::SmartDefault;

#[derive(Debug, Clone)]
pub enum KeyboardMessage {
   TabChanged(usize),
   KeyRepeatChanged(u8),
   DelayRepeatChanged(u8),
   AdjustBrightnessToggled(bool),
   TurnBacklightOffToggled(bool),
   BacklightOffDurationChanged(TurnBacklightOff),
   SetUpBluetoothKeyboard(bool),
   LeftTabSelected(usize),
   RightPaneSelectedToggled(usize, bool),
   RestoreDefaultClicked,
   KeyNavToggled(bool),
   InputSourceLeftTabSelected(usize),
   BtnAddClicked,
   BtnRemoveClicked,
   ShowInputMenuToggled(bool),
   AutoSwitchToggled(bool),
   DictationToggled(bool),
   LanguageChanged(Language),
   ShortcutChanged(ShortcutDict),
   AboutClicked,
}

#[derive(Debug, Clone)]
pub struct KeyboardPage {
   tabbar_state: Vec<(&'static str, button::State)>,
   current_tab_idx: usize,
   keyboard: Keyboard,
   shortcuts: Shortcuts,
   input_sources: InputSources,
   dictation: Dictation,
   btn_setup_bt_keyboard: button::State,
   is_setup_bt_keyboard: bool
}

impl KeyboardPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  Keyboard  ", button::State::new()),
            ("  Shortcuts  ", button::State::new()),
            ("  Input Sources  ", button::State::new()),
            ("  Dictation  ", button::State::new()),
         ],
         current_tab_idx: 0,
         keyboard: Keyboard::new(),
         shortcuts: Shortcuts::new(),
         input_sources: InputSources::new(),
         dictation: Dictation::new(),
         btn_setup_bt_keyboard: button::State::new(),
         is_setup_bt_keyboard: false,
      }
   }

   pub fn update(&mut self, msg: KeyboardMessage) {
      match msg {
         KeyboardMessage::TabChanged(idx) => self.current_tab_idx = idx,
         KeyboardMessage::KeyRepeatChanged(val) => self.keyboard.key_repeat_val = val,
         KeyboardMessage::DelayRepeatChanged(val) => self.keyboard.delay_repeat_val = val,
         KeyboardMessage::AdjustBrightnessToggled(val) => self.keyboard.adjust_brightness_low_light = val,
         KeyboardMessage::TurnBacklightOffToggled(val) => self.keyboard.turn_backlight_off = val,
         KeyboardMessage::BacklightOffDurationChanged(duration) => self.keyboard.turn_backlight_off_after_val = duration,
         KeyboardMessage::SetUpBluetoothKeyboard(val) => self.is_setup_bt_keyboard = val, 
         KeyboardMessage::LeftTabSelected(idx) => self.shortcuts.left_pane_selected = idx,
         KeyboardMessage::RightPaneSelectedToggled(idx, is_checked) => {
            self.shortcuts.right_pane_selected = idx;
            self.shortcuts.shortcuts_tab_map.get_mut(self.shortcuts.left_pane_selected).unwrap().get_mut(idx).unwrap().0 = is_checked;
         },
         KeyboardMessage::RestoreDefaultClicked => self.shortcuts = Shortcuts::new(),
         KeyboardMessage::KeyNavToggled(val) => self.shortcuts.use_keyboard_nav = val,
         KeyboardMessage::InputSourceLeftTabSelected(idx) => self.input_sources.input_sources_selected = Some(idx),
         KeyboardMessage::BtnAddClicked => self.input_sources.input_sources_tab.push(('\u{f1ab}', "Other".to_string(), button::State::new())),
         KeyboardMessage::BtnRemoveClicked => {
            if let Some(selected_idx) = self.input_sources.input_sources_selected {
               if self.input_sources.input_sources_tab.len() > 1 {
                  self.input_sources.input_sources_tab.remove(selected_idx);
               }
            }
            self.input_sources.input_sources_selected = None;
         },
         KeyboardMessage::ShowInputMenuToggled(val) => self.input_sources.show_input_menu = val,
         KeyboardMessage::AutoSwitchToggled(val) => self.input_sources.auto_switch = val,
         KeyboardMessage::DictationToggled(_) => self.dictation.turn_on_dict = !self.dictation.turn_on_dict,
         KeyboardMessage::LanguageChanged(language) => self.dictation.language_val = language,
         KeyboardMessage::ShortcutChanged(shortcut) => self.dictation.shortcut_val = shortcut,
         KeyboardMessage::AboutClicked => {}
      }
   }

   pub fn view(&mut self) -> Element<KeyboardMessage> {
      let KeyboardPage {
         tabbar_state,
         current_tab_idx,
         keyboard,
         shortcuts,
         input_sources,
         dictation,
         btn_setup_bt_keyboard,
         is_setup_bt_keyboard
      } = self;

      // របារផ្ទាំង
      let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
      for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
         let mut btn = Button::new(btn_state, Text::new(*name)).padding(5).on_press(KeyboardMessage::TabChanged(idx));
         if *current_tab_idx == idx {
            btn = btn.style(CustomButton::SelectedTab);
         } else {
            btn = btn.style(CustomButton::Tab);
         }
         tabbar = tabbar.push(btn);
      }
      let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
      let tabbar_section = Container::new(tabbar_con).padding(7).width(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let Keyboard {
               key_repeat_state,
               key_repeat_val,
               delay_repeat_state,
               delay_repeat_val,
               adjust_brightness_low_light,
               turn_backlight_off,
               turn_backlight_off_after_state,
               turn_backlight_off_after_val,
            } = keyboard;

            let lb_key_repeat = Text::new("Key Repeat").size(14);
            let slider_key_repeat = Slider::new(key_repeat_state, 1..=8, *key_repeat_val, KeyboardMessage::KeyRepeatChanged).width(Length::Units(175)).style(CustomSlider::Default);
            let lb_delay_repeat = Text::new("Delay Until Repeat").size(14);
            let slider_delay_repeat = Slider::new(delay_repeat_state, 1..=6, *delay_repeat_val, KeyboardMessage::DelayRepeatChanged).width(Length::Units(175)).style(CustomSlider::Default);
            let key_repeat_row = Row::new().width(Length::Fill).padding(20).spacing(50).align_items(Align::Center)
               .push(
                  Column::new().spacing(15).align_items(Align::Center)
                  .push(lb_key_repeat)
                  .push(
                     Column::new()
                     .push(slider_key_repeat)
                     .push(Row::new().width(Length::Units(175)).spacing(7).push(Text::new("off").size(12)).push(Text::new("slow").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("fast").size(12)))
                  )
               )
               .push(
                  Column::new().spacing(15).align_items(Align::Center)
                  .push(lb_delay_repeat)
                  .push(
                     Column::new()
                     .push(slider_delay_repeat)
                     .push(Row::new().width(Length::Units(175)).push(Text::new("long").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("short").size(12)))
                  )
               );
            let key_repeat_con = Container::new(key_repeat_row).center_x();

            let chk_adjust_brightness = Checkbox::new(*adjust_brightness_low_light, "Adjust keyboard brightness in low light", KeyboardMessage::AdjustBrightnessToggled).spacing(10).style(CustomCheckbox::Default);
            let chk_turn_backlight_off = Checkbox::new(*turn_backlight_off, "Turn keyboard backlight off after", KeyboardMessage::TurnBacklightOffToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_backlight_off_duration = PickList::new(turn_backlight_off_after_state, &TurnBacklightOff::ALL[..], Some(*turn_backlight_off_after_val), KeyboardMessage::BacklightOffDurationChanged).style(CustomSelect::Primary);
            let lb_inactivity = Text::new("of inactivity");
            let keyboard_backligh_off_row = Row::new().spacing(15).align_items(Align::Center)
               .push(chk_turn_backlight_off)
               .push(pl_backlight_off_duration)
               .push(lb_inactivity);

            Container::new(
               Column::new().width(Length::Fill).spacing(20).align_items(Align::Start)
               .push(key_repeat_con)
               .push(
                  Column::new().spacing(15)
                  .push(chk_adjust_brightness)
                  .push(keyboard_backligh_off_row)
               )
            ).width(Length::Fill).height(Length::Fill)
         },
         1 => {
            let Shortcuts {
               btn_restore,
               shortcuts_tab,
               shortcuts_tab_map,
               left_pane_selected,
               right_pane_selected,
               use_keyboard_nav,
               left_pane_scroll,
               right_pane_scroll,
            } = shortcuts;

            let lb_shortcuts = Text::new("To change a shortcut, select it, click key combination, and then type new keys.").size(15);

            // ផ្ទាំងខាងឆ្វេង
            let left_tab_col = shortcuts_tab.iter_mut().enumerate().fold(Scrollable::new(left_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (icon, title, state))| {
               col.push(
                  Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(*title))).width(Length::Fill).on_press(KeyboardMessage::LeftTabSelected(idx)).style(if *left_pane_selected == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
               )
            });
            
            let left_pane = Container::new(left_tab_col).width(Length::FillPortion(4)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្ទាំងខាងស្ដាំ
            let right_pane_col = shortcuts_tab_map.get_mut(*left_pane_selected).unwrap().iter_mut().enumerate().fold(Scrollable::new(right_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (is_checked, title, shortcut))| {
               let row = Row::new().align_items(Align::Center).padding(4)
                  .push(Checkbox::new(*is_checked, *title, move |is| KeyboardMessage::RightPaneSelectedToggled(idx, is)).spacing(10).style(CustomCheckbox::Default))
                  .push(Space::with_width(Length::Fill)).push(Text::new(*shortcut))
                  .push(Space::with_width(Length::Units(15)));

               col.push(Container::new(row).width(Length::Fill).style(if *right_pane_selected == idx {CustomContainer::Hovered} else {CustomContainer::ForegroundWhite}))
            });

            let right_pane = Container::new(right_pane_col).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្នែកខាងក្រោម
            let btn_restore = Button::new(btn_restore, Text::new("  Defaults  ")).on_press(KeyboardMessage::RestoreDefaultClicked).style(CustomButton::Default);
            let restore_row = Row::new().spacing(20).align_items(Align::Center).push(btn_restore);
            let restore_section = Container::new(restore_row).width(Length::Fill);
            
            let chb_keyboard_nav = Checkbox::new(*use_keyboard_nav, "Use keyboard navigations to move focus between controls", KeyboardMessage::KeyNavToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_hint = Text::new("Press the Tab key to move focus forward and Shift tab to move focus backward.");

            let bottom_col = Column::new().spacing(10).width(Length::Fill)
               .push(restore_section)
               .push(Space::with_height(Length::Units(50)))
               .push(chb_keyboard_nav)
               .push(Row::new().push(Space::with_width(Length::Units(30))).push(txt_hint));
            
            Container::new(
               Column::new().spacing(10)
               .push(lb_shortcuts)
               .push(
                  Container::new(
                     Row::new().spacing(15)
                     .push(left_pane)
                     .push(right_pane)
                  ).height(Length::FillPortion(11))
               )
               .push(Container::new(bottom_col).height(Length::FillPortion(5)))
            ).width(Length::Fill).height(Length::Fill)
         },
         2 => {
            let InputSources {
               btn_add_state, 
               btn_remove_state, 
               input_sources_tab,
               input_sources_selected,
               show_input_menu,
               auto_switch,
               left_pane_scroll,
               right_pane_scroll,
            } = input_sources;

            // ផ្ទាំងខាងឆ្វេង
            let tab_len = input_sources_tab.len();
            let left_tab_col = input_sources_tab.iter_mut().enumerate().fold(Scrollable::new(left_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (icon, title, state))| {
               col.push(
                  if let Some(selected_idx) = input_sources_selected {
                     Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(title.as_str()))).width(Length::Fill).on_press(KeyboardMessage::InputSourceLeftTabSelected(idx)).style(if *selected_idx == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
                  } else {
                     Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(title.as_str()))).width(Length::Fill).on_press(KeyboardMessage::InputSourceLeftTabSelected(idx)).style(CustomButton::Sidebar)
                  }
               )
            });
            let btn_add = Button::new(btn_add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(KeyboardMessage::BtnAddClicked).style(CustomButton::Text);
            let mut btn_remove = Button::new(btn_remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
            if input_sources_selected.is_some() && tab_len > 1 {
               btn_remove = btn_remove.on_press(KeyboardMessage::BtnRemoveClicked);
            }
            let btn_group = Container::new(
               Row::new().push(btn_add).push(btn_remove)
            ).width(Length::Fill).style(CustomContainer::Header);
            let left_pane = Container::new(
               Column::new()
               .push(left_tab_col)
               .push(btn_group)
            ).width(Length::FillPortion(4)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្ទាំងខាងស្ដាំ
            let keyboard_image_con = match input_sources_selected {
               Some(idx) => match idx {
                  0 => {
                     let en_keyboard = svg!(format!("{}/assets/images/keyboard.svg", ROOT_PATH())).height(Length::Units(250));
                     Container::new(
                        Row::new().push(Space::with_width(Length::FillPortion(1))).push(en_keyboard).push(Space::with_width(Length::FillPortion(1)))
                     ).width(Length::Fill).center_x().center_y()
                  },
                  1 => {
                     let kh_keyboard = svg!(format!("{}/assets/images/keyboard.svg", ROOT_PATH())).height(Length::Units(250));
                     Container::new(
                        Row::new().push(Space::with_width(Length::FillPortion(1))).push(kh_keyboard).push(Space::with_width(Length::FillPortion(1)))
                     ).width(Length::Fill).center_x().center_y()
                  },
                  _ => Container::new(Space::with_width(Length::Fill))
               }
               None => Container::new(Space::with_width(Length::Fill))
            };

            let right_pane = Container::new(
               Scrollable::new(right_pane_scroll).push(keyboard_image_con)
            ).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្នែកខាងក្រោម
            let chb_show_input_menu = Checkbox::new(*show_input_menu, "Show Input menu in menu bar", KeyboardMessage::ShowInputMenuToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_auto_switch = Checkbox::new(*auto_switch, "Automatically switch to a document's input source", KeyboardMessage::AutoSwitchToggled).spacing(10).style(CustomCheckbox::Default);
            let bottom_right_col = Column::new().spacing(10)
               .push(chb_show_input_menu)
               .push(chb_auto_switch);

            let bottom_row = Row::new().spacing(15).width(Length::Fill)
               .push(Space::with_width(Length::FillPortion(4)))
               .push(Container::new(bottom_right_col).width(Length::FillPortion(6)));
            
            Container::new(
               Column::new().spacing(10)
               .push(
                  Container::new(
                     Row::new().spacing(15)
                     .push(left_pane)
                     .push(right_pane)
                  ).height(Length::FillPortion(11))
               )
               .push(bottom_row)
            ).width(Length::Fill).height(Length::Fill)
         }, 
         3 => {
            let Dictation {
               btn_about, 
               turn_on_dict,
               language_state,
               language_val,
               shortcut_state,
               shortcut_val,
            } = dictation;

            // ផ្ទាំងខាងឆ្វេង
            let mic_image = svg!(format!("{}/assets/images/mic.svg", ROOT_PATH())).height(Length::Units(127));
            let mic_con = Container::new(mic_image).width(Length::FillPortion(3)).center_x();

            // ផ្ទាំងខាងស្ដាំ
            let txt_dictation = Text::new("Use dictation wherever you can type text. To start dictating,\nuse the shortcut or select Start Dictation from the Edit menu.");
            let lb_dictation = Text::new("Dictation:");
            let rd_dictaion_on = Radio::new(true, "On", Some(*turn_on_dict), KeyboardMessage::DictationToggled).size(15).spacing(10).style(if *turn_on_dict {CustomRadio::Active} else {CustomRadio::Disactive});
            let rd_dictaion_off = Radio::new(false, "Off", Some(*turn_on_dict), KeyboardMessage::DictationToggled).size(15).spacing(10).style(if !(*turn_on_dict) {CustomRadio::Active} else {CustomRadio::Disactive});
            let dictation_section = Row::new().spacing(10).align_items(Align::Center)
               .push(lb_dictation)
               .push(rd_dictaion_on)
               .push(rd_dictaion_off);

            let lb_language = Text::new("Language:");
            let pl_language = PickList::new(language_state, &Language::ALL[..], Some(*language_val), KeyboardMessage::LanguageChanged).style(CustomSelect::Primary);
            let language_section = Row::new().spacing(10).align_items(Align::Center)
               .push(lb_language)
               .push(pl_language);

            let lb_shortcut = Text::new("Shortcut:");
            let pl_shortcut = PickList::new(shortcut_state, &ShortcutDict::ALL[..], Some(*shortcut_val), KeyboardMessage::ShortcutChanged).style(CustomSelect::Primary);
            let shortcut_section = Row::new().spacing(10).align_items(Align::Center)
               .push(lb_shortcut)
               .push(pl_shortcut);
            
            let right_con = Container::new(
               Column::new().spacing(20)
               .push(txt_dictation)
               .push(
                  Column::new().spacing(10)
                  .push(dictation_section)
                  .push(language_section)
                  .push(shortcut_section)
               )
            ).width(Length::FillPortion(7)).height(Length::Fill);
         
            Container::new(
               Column::new().spacing(10)
               .push(
                  Container::new(
                     Row::new().spacing(15)
                     .push(mic_con)
                     .push(right_con)
                  ).height(Length::FillPortion(11))
               )
               .push(
                  Container::new(
                     Button::new(btn_about, Text::new("  About Dictation & Privacy  ")).on_press(KeyboardMessage::AboutClicked).style(CustomButton::Default)
                  ).width(Length::Fill).align_x(Align::End)
               )
            ).width(Length::Fill).height(Length::Fill)
         },
         _ => Container::new(Space::with_height(Length::Fill))
      };

      // ផ្នែកខាងក្រោម
      let bottom_row = Row::new().padding(15).spacing(20).align_items(Align::Center)
         .push(Text::new(if *is_setup_bt_keyboard {"Sorry, It's just UI"} else {""}))
         .push(Button::new(btn_setup_bt_keyboard, Text::new("  Set Up Bluetooth Keyboard...  ")).style(CustomButton::Default).on_press(KeyboardMessage::SetUpBluetoothKeyboard(!self.is_setup_bt_keyboard)));
      let bottom_section = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);

      // មាតិកា   
      let content = Column::new().width(Length::Fill).align_items(Align::Center)
         .push(tabbar_section)
         .push(tabview.height(Length::Fill).padding(20).style(CustomContainer::ForegroundGray))
         .push(bottom_section);

      Container::new(content).width(Length::FillPortion(15)).padding(20).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

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
   const ALL: [TurnBacklightOff; 5] = [
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
   key_repeat_state: slider::State,
   key_repeat_val: u8,
   delay_repeat_state: slider::State,
   delay_repeat_val: u8,
   adjust_brightness_low_light: bool,
   turn_backlight_off: bool,
   turn_backlight_off_after_state: pick_list::State<TurnBacklightOff>,
   turn_backlight_off_after_val: TurnBacklightOff,
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
   btn_restore: button::State,
   shortcuts_tab: Vec<(char, &'static str, button::State)>,
   shortcuts_tab_map: Vec<Vec<(bool, &'static str, &'static str)>>,
   left_pane_selected: usize,
   right_pane_selected: usize,
   use_keyboard_nav: bool,
   left_pane_scroll: scrollable::State,
   right_pane_scroll: scrollable::State,
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
   btn_add_state: button::State, 
   btn_remove_state: button::State, 
   input_sources_tab: Vec<(char, String, button::State)>,
   input_sources_selected: Option<usize>,
   show_input_menu: bool,
   auto_switch: bool,
   left_pane_scroll: scrollable::State,
   right_pane_scroll: scrollable::State,
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

#[derive(Debug, Clone, Default)]
pub struct Dictation {
   btn_about: button::State, 
   turn_on_dict: bool,
   language_state: pick_list::State<Language>,
   language_val: Language,
   shortcut_state: pick_list::State<ShortcutDict>,
   shortcut_val: ShortcutDict,
}

impl Dictation {
   pub fn new() -> Self {
      Self::default()
   }
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum Language {
   English,
   #[default]
   Khmer,
   AddNew
}

impl Language {
   const ALL: [Language; 3] = [
      Language::English,
      Language::Khmer,
      Language::AddNew,
   ];
}

impl std::fmt::Display for Language {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            Language::English => "English (US)",
            Language::Khmer => "Khmer",
            Language::AddNew => "Add Language...",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum ShortcutDict {
   Off,
   #[default]
   CtrlTwice,
   FnTwice,
   Customize
}

impl ShortcutDict {
   const ALL: [ShortcutDict; 4] = [
      ShortcutDict::Off,
      ShortcutDict::CtrlTwice,
      ShortcutDict::FnTwice,
      ShortcutDict::Customize,
   ];
}

impl std::fmt::Display for ShortcutDict {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            ShortcutDict::Off => "Off",
            ShortcutDict::CtrlTwice => "Press Ctrl Key Twice",
            ShortcutDict::FnTwice => "Press Fn Key Twice",
            ShortcutDict::Customize => "Customize...",
         }
      )
   }
}