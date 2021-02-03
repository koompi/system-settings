// use super::super::styles::{
//    CustomButton, CustomCheckbox, CustomContainer, CustomSelect, CustomSlider,
// };
// use iced::{
//    button, pick_list, scrollable, slider, Align, Button, Checkbox, Color, Column, Container,
//    Element, Length, PickList, Row, Scrollable, Slider, Space, Text,
// };
// use iced_custom_widget::{table, Icon, Table, TableData, TableError, TableResult};
// use iced_custom_widget::{table_column, table_columns};
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use smart_default::SmartDefault;

// #[derive(Debug, Clone)]
// pub enum SoundMessage {
//    TabChanged(usize),
//    SoundEffectDeviceChanged(SoundEffectDevice),
//    AlertVolumeChanged(u8),
//    TogglePlayStartup(bool),
//    TogglePlaySoundEffects(bool),
//    TogglePlayFeedback(bool),
//    OutputVolumeChanged(u8),
//    ToggleMute(bool),
//    ToggleShowVolumn(bool),
//    OutputBalanceChanged(u8),
//    InputVolumeChanged(u8),
// }

// #[derive(Debug, Clone)]
// pub struct SoundPage {
//    tabbar_state: Vec<(String, button::State)>,
//    current_tab_idx: usize,
//    sound_effect: SoundEffect,
//    sound_output: SoundOutput,
//    sound_input: SoundInput,
//    output_volumn_state: slider::State,
//    output_volumn_value: u8,
//    mute: bool,
//    show_volumn: bool,
// }

// impl SoundPage {
//    pub fn new() -> Self {
//       Self {
//          tabbar_state: vec![
//             ("  Sound Effects  ".to_string(), button::State::new()),
//             ("  Output  ".to_string(), button::State::new()),
//             ("  Input  ".to_string(), button::State::new()),
//          ],
//          current_tab_idx: 0,
//          sound_effect: SoundEffect::new(),
//          sound_output: SoundOutput::new(),
//          sound_input: SoundInput::new(),
//          output_volumn_state: slider::State::new(),
//          output_volumn_value: 27,
//          mute: false,
//          show_volumn: true,
//       }
//    }

//    pub fn update(&mut self, msg: SoundMessage) {
//       match msg {
//          SoundMessage::TabChanged(idx) => self.current_tab_idx = idx,
//          SoundMessage::SoundEffectDeviceChanged(device) => {
//             self.sound_effect.selected_sound_effect_device = device
//          }
//          SoundMessage::AlertVolumeChanged(val) => self.sound_effect.alert_volumn_value = val,
//          SoundMessage::TogglePlayStartup(val) => self.sound_effect.play_on_startup = val,
//          SoundMessage::TogglePlaySoundEffects(val) => self.sound_effect.play_sound_effects = val,
//          SoundMessage::TogglePlayFeedback(val) => self.sound_effect.play_feedback = val,
//          SoundMessage::OutputVolumeChanged(val) => {
//             if val == 0 {
//                self.mute = true;
//             } else {
//                self.mute = false;
//             }
//             self.output_volumn_value = val;
//          }
//          SoundMessage::ToggleMute(val) => {
//             self.mute = val;
//             self.output_volumn_value = if val { 0 } else { 50 };
//          }
//          SoundMessage::ToggleShowVolumn(val) => self.show_volumn = val,
//          SoundMessage::OutputBalanceChanged(val) => self.sound_output.balance_output_value = val,
//          SoundMessage::InputVolumeChanged(val) => self.sound_input.input_volume_value = val,
//       }
//    }

//    pub fn view(&mut self) -> Element<SoundMessage> {
//       let SoundPage {
//          tabbar_state,
//          current_tab_idx,
//          sound_effect,
//          sound_output,
//          sound_input,
//          output_volumn_state,
//          output_volumn_value,
//          mute,
//          show_volumn,
//       } = self;
//       // របារផ្ទាំង
//       let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
//       for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
//          let mut btn = Button::new(btn_state, Text::new(name.as_str()))
//             .padding(5)
//             .on_press(SoundMessage::TabChanged(idx));
//          if *current_tab_idx == idx {
//             btn = btn.style(CustomButton::SelectedTab);
//          } else {
//             btn = btn.style(CustomButton::Tab);
//          }
//          tabbar = tabbar.push(btn);
//       }
//       let tabbar_con = Container::new(tabbar)
//          .padding(2)
//          .center_x()
//          .style(CustomContainer::Segment);
//       let tabbar_section = Container::new(tabbar_con)
//          .padding(7)
//          .width(Length::Fill)
//          .center_x();

//       // ទិដ្ឋភាពទូទៅ
//       let tabview = match self.current_tab_idx {
//          0 => {
//             let SoundEffect {
//                alert_sound_scroll,
//                alert_idx,
//                sound_effect_device,
//                selected_sound_effect_device,
//                alert_volumn_state,
//                alert_volumn_value,
//                play_on_startup,
//                play_sound_effects,
//                play_feedback,
//                ..
//             } = sound_effect;
//             let label_alert_sound = Text::new("Select an alert sound:").size(14);
//             let mut device_pane_col = Scrollable::new(alert_sound_scroll).width(Length::Fill);

//             device_pane_col = AlertSound::ALL.iter().enumerate().fold(
//                device_pane_col,
//                |col, (idx, alert_sound)| {
//                   let mut alert_con = Container::new(
//                      Row::new()
//                         .padding(3)
//                         .align_items(Align::Center)
//                         .push(Space::with_width(Length::Units(7)))
//                         .push(Text::new(alert_sound.to_string())),
//                   )
//                   .width(Length::Fill);

//                   if *alert_idx == idx {
//                      alert_con = alert_con.style(CustomContainer::Background);
//                   }
//                   col.push(alert_con)
//                },
//             );

//             let device_pane = Container::new(
//                Column::new()
//                   .push(
//                      Container::new(Text::new("Names").size(12))
//                         .width(Length::Fill)
//                         .padding(7)
//                         .style(CustomContainer::Header),
//                   )
//                   .push(device_pane_col),
//             )
//             .height(Length::Units(150))
//             .style(CustomContainer::ForegroundWhite);

//             let lb_sound_effect = Text::new("Play sound effects through:");
//             let pl_sound_effect = PickList::new(
//                sound_effect_device,
//                &SoundEffectDevice::ALL[..],
//                Some(*selected_sound_effect_device),
//                SoundMessage::SoundEffectDeviceChanged,
//             )
//             .style(CustomSelect::Primary);
//             let sound_effect_device = Row::new()
//                .spacing(10)
//                .align_items(Align::Center)
//                .push(lb_sound_effect)
//                .push(pl_sound_effect)
//                .push(Space::with_width(Length::Units(180)));

//             let lb_alert_volumn = Text::new("Alert volume:");
//             let ic_volumn_down = Icon::new('\u{f027}')
//                .size(27)
//                .color(Color::from_rgb8(66, 66, 66));
//             let slider_alert_volumn = Slider::new(
//                alert_volumn_state,
//                0..=100,
//                *alert_volumn_value,
//                SoundMessage::AlertVolumeChanged,
//             )
//             .width(Length::Units(227))
//             .style(CustomSlider::Default);
//             let ic_volumn_up = Icon::new('\u{f028}')
//                .size(27)
//                .color(Color::from_rgb8(66, 66, 66));
//             let alert_volumn_section = Row::new()
//                .spacing(10)
//                .align_items(Align::Center)
//                .push(lb_alert_volumn)
//                .push(ic_volumn_down)
//                .push(slider_alert_volumn)
//                .push(ic_volumn_up);

//             let chk_play_on_startup = Checkbox::new(
//                *play_on_startup,
//                "Play sound on startup",
//                SoundMessage::TogglePlayStartup,
//             )
//             .spacing(10)
//             .style(CustomCheckbox::Default);
//             let chk_play_sound_effects = Checkbox::new(
//                *play_sound_effects,
//                "Play user interface sound effects",
//                SoundMessage::TogglePlaySoundEffects,
//             )
//             .spacing(10)
//             .style(CustomCheckbox::Default);
//             let chk_play_feedback = Checkbox::new(
//                *play_feedback,
//                "Play feedback when volume is changed",
//                SoundMessage::TogglePlayFeedback,
//             )
//             .spacing(10)
//             .style(CustomCheckbox::Default);

//             let container = Container::new(
//                Column::new()
//                   .width(Length::Fill)
//                   .spacing(10)
//                   .align_items(Align::Center)
//                   .push(sound_effect_device)
//                   .push(alert_volumn_section)
//                   .push(
//                      Row::new().push(Space::with_width(Length::Units(10))).push(
//                         Column::new()
//                            .spacing(10)
//                            .push(chk_play_on_startup)
//                            .push(chk_play_sound_effects)
//                            .push(chk_play_feedback),
//                      ),
//                   ),
//             )
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y();

//             Container::new(
//                Column::new()
//                   .spacing(10)
//                   .push(label_alert_sound)
//                   .push(device_pane)
//                   .push(container),
//             )
//          }
//          1 => {
//             let SoundOutput {
//                output_devices,
//                tb_devices_state,
//                balance_output_state,
//                balance_output_value,
//             } = sound_output;

//             let lb_output_device = Text::new("Select a device for sound output:");
//             let tb_columns = table_columns![("name", "Name"), ("type", "Type"),];
//             let tb_output_device =
//                Table::new(tb_devices_state, tb_columns, output_devices).width(Length::Fill);

//             let lb_selected_device = Text::new("Settings for the selected device:");
//             let lb_balance = Text::new("Balance:");
//             let slider_balance = Slider::new(
//                balance_output_state,
//                0..=100,
//                *balance_output_value,
//                SoundMessage::OutputBalanceChanged,
//             )
//             .width(Length::Units(175))
//             .style(CustomSlider::Default);
//             let balance_section = Container::new(
//                Row::new()
//                   .spacing(20)
//                   .align_items(Align::Center)
//                   .push(lb_balance)
//                   .push(
//                      Column::new().push(slider_balance).push(
//                         Row::new()
//                            .width(Length::Units(175))
//                            .push(Text::new("left").size(12))
//                            .push(Space::with_width(Length::Fill))
//                            .push(Text::new("right").size(12)),
//                      ),
//                   ),
//             )
//             .width(Length::Fill);

//             Container::new(
//                Column::new()
//                   .spacing(20)
//                   .push(
//                      Column::new()
//                         .spacing(7)
//                         .push(lb_output_device)
//                         .push(tb_output_device),
//                   )
//                   .push(
//                      Column::new()
//                         .spacing(15)
//                         .push(lb_selected_device)
//                         .push(balance_section),
//                   ),
//             )
//          }
//          2 => {
//             let SoundInput {
//                input_devices,
//                tb_devices_state,
//                input_volume_state,
//                input_volume_value,
//             } = sound_input;

//             let lb_input_device = Text::new("Select a device for sound input:");
//             let tb_columns = table_columns![("name", "Name"), ("type", "Type"),];
//             let tb_input_device =
//                Table::new(tb_devices_state, tb_columns, input_devices).width(Length::Fill);

//             let lb_selected_device = Text::new("Settings for the selected device:");
//             let lb_balance = Text::new("Input volume:");
//             let ic_volume_down = Icon::new('\u{f131}')
//                .size(27)
//                .color(Color::from_rgb8(66, 66, 66));
//             let slider_balance = Slider::new(
//                input_volume_state,
//                0..=100,
//                *input_volume_value,
//                SoundMessage::InputVolumeChanged,
//             )
//             .width(Length::Units(200))
//             .style(CustomSlider::Default);
//             let ic_volume_up = Icon::new('\u{f130}')
//                .size(27)
//                .color(Color::from_rgb8(66, 66, 66));
//             let input_vol_row = Row::new()
//                .spacing(5)
//                .align_items(Align::Center)
//                .push(lb_balance)
//                .push(ic_volume_down)
//                .push(slider_balance)
//                .push(ic_volume_up);
//             let input_vol_con = Container::new(input_vol_row).width(Length::Fill);

//             Container::new(
//                Column::new()
//                   .spacing(20)
//                   .push(
//                      Column::new()
//                         .spacing(7)
//                         .push(lb_input_device)
//                         .push(tb_input_device),
//                   )
//                   .push(
//                      Column::new()
//                         .spacing(15)
//                         .push(lb_selected_device)
//                         .push(input_vol_con),
//                   ),
//             )
//          }
//          _ => Container::new(Space::with_height(Length::Fill)),
//       };

//       // ផ្នែកខាងក្រោម
//       let lb_output_volumn = Text::new("Output volume:");
//       let ic_volumn_down = Icon::new('\u{f027}')
//          .size(27)
//          .color(Color::from_rgb8(66, 66, 66));
//       let slider_output_volumn = Slider::new(
//          output_volumn_state,
//          0..=100,
//          *output_volumn_value,
//          SoundMessage::OutputVolumeChanged,
//       )
//       .width(Length::Units(227))
//       .style(CustomSlider::Default);
//       let ic_volumn_up = Icon::new('\u{f028}')
//          .size(27)
//          .color(Color::from_rgb8(66, 66, 66));
//       let chk_mute = Checkbox::new(*mute, "Mute", SoundMessage::ToggleMute)
//          .spacing(10)
//          .style(CustomCheckbox::Default);
//       let chk_show_volumn = Checkbox::new(
//          *show_volumn,
//          "Show volume in menu bar",
//          SoundMessage::ToggleShowVolumn,
//       )
//       .spacing(10)
//       .style(CustomCheckbox::Default);
//       let output_volumn_row = Row::new()
//          .spacing(10)
//          .align_items(Align::Center)
//          .push(Space::with_width(Length::Units(45)))
//          .push(lb_output_volumn)
//          .push(ic_volumn_down)
//          .push(slider_output_volumn)
//          .push(ic_volumn_up)
//          .push(chk_mute);
//       let bottom_col = Column::new()
//          .spacing(10)
//          .align_items(Align::Center)
//          .push(output_volumn_row)
//          .push(
//             Row::new()
//                .push(chk_show_volumn)
//                .push(Space::with_width(Length::Units(60))),
//          );
//       let bottom_section = Container::new(bottom_col).padding(20).center_x();

//       // មាតិកា
//       let content = Column::new()
//          .align_items(Align::Center)
//          .push(tabbar_section)
//          .push(
//             tabview
//                .height(Length::Fill)
//                .padding(20)
//                .style(CustomContainer::ForegroundGray),
//          )
//          .push(bottom_section);

//       Container::new(content)
//          .padding(20)
//          .width(Length::FillPortion(15))
//          .height(Length::Fill)
//          .style(CustomContainer::Background)
//          .into()
//    }
// }

// #[derive(Debug, Clone, SmartDefault)]
// pub enum AlertSound {
//    #[default]
//    Boop,
//    Breeze,
//    Bubble,
//    Crystal,
//    Funcky,
//    Heroine,
//    Jump,
//    Mezzo,
//    Pebble,
//    Pluck,
//    Pong,
//    Sunami,
// }

// impl AlertSound {
//    const ALL: [AlertSound; 12] = [
//       AlertSound::Boop,
//       AlertSound::Breeze,
//       AlertSound::Bubble,
//       AlertSound::Crystal,
//       AlertSound::Funcky,
//       AlertSound::Heroine,
//       AlertSound::Jump,
//       AlertSound::Mezzo,
//       AlertSound::Pebble,
//       AlertSound::Pluck,
//       AlertSound::Pong,
//       AlertSound::Sunami,
//    ];
// }

// impl std::fmt::Display for AlertSound {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       write!(
//          f,
//          "{}",
//          match self {
//             AlertSound::Boop => "Boop",
//             AlertSound::Breeze => "Breeze",
//             AlertSound::Bubble => "Bubble",
//             AlertSound::Crystal => "Crystal",
//             AlertSound::Funcky => "Funcky",
//             AlertSound::Heroine => "Heroine",
//             AlertSound::Jump => "Jump",
//             AlertSound::Mezzo => "Mezzo",
//             AlertSound::Pebble => "Pebble",
//             AlertSound::Pluck => "Pluck",
//             AlertSound::Pong => "Pong",
//             AlertSound::Sunami => "Sunami",
//          }
//       )
//    }
// }

// #[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
// pub enum SoundEffectDevice {
//    #[default]
//    OutputDevice,
//    Speaker,
// }

// impl SoundEffectDevice {
//    const ALL: [SoundEffectDevice; 2] =
//       [SoundEffectDevice::OutputDevice, SoundEffectDevice::Speaker];
// }

// impl std::fmt::Display for SoundEffectDevice {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       write!(
//          f,
//          "{}",
//          match self {
//             SoundEffectDevice::OutputDevice => "Selected sound output device",
//             SoundEffectDevice::Speaker => "Koompi Pro Speakers",
//          }
//       )
//    }
// }

// #[derive(Debug, Clone, Default)]
// pub struct SoundEffect {
//    alert_sound_scroll: scrollable::State,
//    alert_idx: usize,
//    selected_alert_sound: AlertSound,
//    sound_effect_device: pick_list::State<SoundEffectDevice>,
//    selected_sound_effect_device: SoundEffectDevice,
//    alert_volumn_state: slider::State,
//    alert_volumn_value: u8,
//    play_on_startup: bool,
//    play_sound_effects: bool,
//    play_feedback: bool,
// }

// impl SoundEffect {
//    pub fn new() -> Self {
//       Self {
//          alert_idx: 0,
//          alert_volumn_value: 100,
//          play_on_startup: true,
//          play_sound_effects: true,
//          play_feedback: false,
//          ..Default::default()
//       }
//    }
// }

// #[derive(Debug, Clone, Default)]
// pub struct SoundOutput {
//    output_devices: Vec<SoundDevice>,
//    tb_devices_state: table::State,
//    balance_output_state: slider::State,
//    balance_output_value: u8,
// }

// impl SoundOutput {
//    pub fn new() -> Self {
//       Self {
//          output_devices: vec![
//             SoundDevice::new("Koompi Pro Speakers", "Built-in"),
//             SoundDevice::new("External Headphones", "Headphone"),
//          ],
//          balance_output_value: 50,
//          ..Default::default()
//       }
//    }
// }

// #[derive(Debug, Clone, Default)]
// pub struct SoundInput {
//    input_devices: Vec<SoundDevice>,
//    tb_devices_state: table::State,
//    input_volume_state: slider::State,
//    input_volume_value: u8,
// }

// impl SoundInput {
//    pub fn new() -> Self {
//       Self {
//          input_devices: vec![
//             SoundDevice::new("Koompi Pro Microphone", "Built-in"),
//             SoundDevice::new("External Microphone", "Microphone"),
//          ],
//          input_volume_value: 50,
//          ..Default::default()
//       }
//    }
// }

// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct SoundDevice {
//    name: String,
//    _type: String,
// }

// impl SoundDevice {
//    pub fn new(name: &str, _type: &str) -> Self {
//       Self {
//          name: name.to_string(),
//          _type: _type.to_string(),
//       }
//    }
// }

// impl TableData for SoundDevice {
//    fn get_field_value(&self, field_name: &str) -> TableResult<Value> {
//       let value = match field_name {
//          "name" => serde_json::to_value(&self.name),
//          "type" => serde_json::to_value(&self._type),
//          s => return Err(TableError::InvalidFieldName(s.to_owned())),
//       };
//       Ok(value.unwrap())
//    }
// }

use iced::{
   button, executor, pick_list, scrollable, slider, window, Align, Application, Button, Column,
   Command, Container, Element, Length, PickList, Row, Rule, Scrollable, Settings, Slider, Space,
   Subscription, Text,
};

use iced_custom_widget as icw;

use icw::components::Icon;
use icw::components::Tab;
use icw::components::Toggler;
use icw::styles::{
   buttons::ButtonStyle, containers::ContainerStyle, pick_list::PickListStyle, slider::SliderStyle,
};

use std::collections::HashMap;
// use rodio::Source;
use std::fmt;
use std::hash::Hash;
// use std::fs::File;
// use std::io::BufReader;

use std::ops::Index;
use std::path::PathBuf;
#[allow(non_snake_case)]
#[derive(Default, Debug, Clone)]
pub struct SoundPage {
   choice: Choice,
   scroll_content: scrollable::State,
   pick_out_dev: pick_list::State<OutputDevice>,
   pick_in_dev: pick_list::State<InputDevice>,
   selected_in_dev: InputDevice,
   selected_out_dev: OutputDevice,
   is_boost_sound: bool,
   is_muted: bool,
   enable_sound_effect: bool,
   is_sound_effect: bool,
   is_auto_noise_suppression: bool,
   is_in_muted: bool,
   sound_effecs: SettingsSoundEffect,
   sample_effects: Vec<(button::State, button::State, String)>,
   out_value: f32,
   effect_tick: usize,
   balance_val: f32,
   input_level: f32,
   input_val: f32,
   slider_output: slider::State,
   slider_input_level: slider::State,
   slider_input: slider::State,
   balace_state: slider::State,
   mute_out_sound: button::State,
   mute_in_sound: button::State,
   sound_effect_state: button::State,

   window_size: (u32, u32),
   FONT_SIZE: u16,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
   A,
   B,
   C,
}
impl Default for Choice {
   fn default() -> Self {
      Choice::A
   }
}

#[derive(Debug, Clone)]
pub enum SoundMessage {
   TabSelect(Choice),
   SeletedOut(OutputDevice),
   SeletedIn(InputDevice),
   EnableBoostSound(bool),
   SoundOutChanged(f32),
   SoundInChanged(f32),
   SoundEffect(bool),
   InputLevelChanged(f32),
   AutomatedSoundSuppression(bool),
   MutedSound,
   MuteSound,
   MutedInSound,
   EnableEffect(usize),
   TestSoundEffect(usize),
   BalanceChanged(f32),
   WindowResize((u32, u32)),
   CloseApp,
   Escape,
}
pub type SoundEffectErorr = Result<bool, std::io::Error>;

pub trait SoundEffect {
   fn play(&mut self, file: PathBuf) -> SoundEffectErorr;
   fn stop(&mut self, file: PathBuf) -> SoundEffectErorr;
   fn pause(&mut self, file: PathBuf) -> SoundEffectErorr;
   fn speed(&self) -> u32;
   fn volume(&self) -> u32;
}
#[derive(Debug, Default, Clone)]
pub struct SettingsSoundEffect {
   file: std::path::PathBuf,
   hash_sounds: HashMap<SoundEffectType, PathBuf>,
   effect_type: SoundEffectType,
   volume: u32,
   speed: u32,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SoundEffectType {
   Bootup,
   ShutDown,
   Logout,
   Wakeup,
   VolumnUpDown,
   // Notification,
   // LowBattery,
   // SendIconLauncher,
   // EmptyTrash,
   // Plugin,
   // Plugout,
   // RemoveDevConnected,
   // RemovableDevRemoved,
   // ErrorSound,
}
impl SoundEffectType {
   const ALL: [SoundEffectType; 5] = [
      SoundEffectType::Bootup,
      SoundEffectType::ShutDown,
      SoundEffectType::Logout,
      SoundEffectType::Wakeup,
      SoundEffectType::VolumnUpDown,
      // SoundEffectType::Notification,
      // SoundEffectType::LowBattery,
      // SoundEffectType::SendIconLauncher,
      // SoundEffectType::EmptyTrash,
      // SoundEffectType::Plugin,
      // SoundEffectType::Plugout,
      // SoundEffectType::RemoveDevConnected,
      // SoundEffectType::RemovableDevRemoved,
      // SoundEffectType::RemovableDevRemoved,
   ];
}
impl fmt::Display for SoundEffectType {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
         f,
         "{}",
         match self {
            SoundEffectType::Bootup => "Bootup",
            SoundEffectType::ShutDown => "Shutdown",
            SoundEffectType::Logout => "Log out",
            SoundEffectType::Wakeup => "Wake Up",
            SoundEffectType::VolumnUpDown => "Volume +/-",
            // SoundEffectType::Notification => "Notifications",
            // SoundEffectType::LowBattery => "Low battery",
            // SoundEffectType::SendIconLauncher => "Send icon in Launcher to Desktop",
            // SoundEffectType::EmptyTrash => "Empty Trash",
            // SoundEffectType::Plugin => "Plug in",
            // SoundEffectType::Plugout => "Plug out",
            // SoundEffectType::RemoveDevConnected => "Removable device connected",
            // SoundEffectType::RemovableDevRemoved => "Removable device removed",
            // SoundEffectType::ErrorSound => "Error",
         }
      )
   }
}
impl Default for SoundEffectType {
   fn default() -> Self {
      SoundEffectType::Bootup
   }
}
impl SettingsSoundEffect {
   pub fn new() -> Self {
      Self {
         file: dirs::config_dir().unwrap(),
         ..Default::default()
      }
   }
}
impl SoundEffect for SettingsSoundEffect {
   fn play(&mut self, file: PathBuf) -> SoundEffectErorr {
      unimplemented!("Function is unimplemented");
   }
   fn pause(&mut self, file: PathBuf) -> SoundEffectErorr {
      unimplemented!("Function is unimplemented");
   }
   fn stop(&mut self, file: PathBuf) -> SoundEffectErorr {
      unimplemented!("Function is unimplemented");
   }
   fn speed(&self) -> u32 {
      unimplemented!("Function is unimplemented");
   }
   fn volume(&self) -> u32 {
      unimplemented!("Function is unimplemented");
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputDevice {
   Internal,
   External,
}
impl Default for OutputDevice {
   fn default() -> Self {
      OutputDevice::Internal
   }
}
impl OutputDevice {
   const ALL: [OutputDevice; 2] = [OutputDevice::Internal, OutputDevice::External];
}

impl fmt::Display for OutputDevice {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
         f,
         "{}",
         match self {
            OutputDevice::Internal => "Internal (HDA Intel PCH)",
            OutputDevice::External => "External",
         }
      )
   }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDevice {
   Internal,
   External,
}
impl Default for InputDevice {
   fn default() -> Self {
      InputDevice::Internal
   }
}
impl InputDevice {
   const ALL: [InputDevice; 2] = [InputDevice::Internal, InputDevice::External];
}

impl fmt::Display for InputDevice {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(
         f,
         "{}",
         match self {
            InputDevice::Internal => "Internal (HDA Intel PCH)",
            InputDevice::External => "External",
         }
      )
   }
}

// impl Index<String> for SettingsSoundEffect {
//     type Output = PathBuf;
//     fn index(&self, key: String) -> &PathBuf {
//         &self.hash_sounds.index(&key)
//     }
// }
impl Index<&SoundEffectType> for SettingsSoundEffect {
   type Output = PathBuf;
   fn index(&self, key: &SoundEffectType) -> &Self::Output {
      &self.hash_sounds.index(&key)
   }
}
impl SoundPage {
   pub fn new() -> SoundPage {
      let str_con = |f: &str| -> String { f.to_string() };
      let mut vec_sounds: Vec<String> = vec![
         str_con("Bootup"),
         str_con("Shutdown"),
         str_con("Log out"),
         str_con("Wake Up"),
         str_con("Volume +/-"),
         // str_con("Notifications"),
         // str_con("Low battery"),
         // str_con("Send icon in Launcher to Desktop"),
         // str_con("Empty Trash"),
         // str_con("Plug in"),
         // str_con("Plug out"),
         // str_con("Removeable device connected"),
         // str_con("Removable device removed"),
         // str_con("Error"),
      ];
      let mut vec_tuple: Vec<(button::State, button::State, String)> = Vec::new();
      vec_sounds.iter_mut().for_each(|name| {
         vec_tuple.push((button::State::new(), button::State::new(), name.clone()))
      });
      let mut sound_effect_hash: HashMap<SoundEffectType, PathBuf> = HashMap::new();
      match playback::read_directory(
         std::path::PathBuf::new().join(standart_path::sys_data_dir().unwrap().join("syssettings")),
      ) {
         Ok(mut path) =>
         {
            #[allow(const_item_mutation)]
            for (i, j) in SoundEffectType::ALL[..].iter_mut().zip(path.iter_mut()) {
               sound_effect_hash.insert(*i, j.to_path_buf());
            }
         }
         Err(e) => println!("Error: {}", e),
      }
      for (i, j) in &sound_effect_hash {
         println!("key: {} value: {:?}", i, j);
      }
      println!(
         "Booup value: {:?}",
         sound_effect_hash.index(&SoundEffectType::Bootup)
      );

      Self {
         FONT_SIZE: 12,
         sound_effecs: SettingsSoundEffect {
            hash_sounds: sound_effect_hash,
            ..Default::default()
         },
         enable_sound_effect: true,
         sample_effects: vec_tuple,
         ..Default::default()
      }
   }

   pub fn update(&mut self, message: SoundMessage) {
      match message {
         SoundMessage::TabSelect(tab) => {
            self.choice = tab;
         }
         SoundMessage::SeletedOut(out_dev) => {
            self.selected_out_dev = out_dev;
         }
         SoundMessage::EnableBoostSound(is_enable) => {
            self.is_boost_sound = is_enable;
         }
         SoundMessage::SoundEffect(is_effectd) => {
            self.is_sound_effect = is_effectd;
         }
         SoundMessage::SoundOutChanged(val) => {
            self.out_value = val;
            SoundBackEnd::initialize();
         }
         SoundMessage::InputLevelChanged(val) => {
            self.input_level = val;
         }
         SoundMessage::SoundInChanged(val) => {
            self.input_val = val;
         }
         SoundMessage::TestSoundEffect(idx) => {
            let key = SoundEffectType::ALL[idx];
            let value = self.sound_effecs.hash_sounds.index(&key);
            match playback::run(&value) {
               Ok(()) => println!("sucesss"),
               Err(e) => println!("Error: {}", e),
            }
         }
         SoundMessage::CloseApp => {
            println!("Application clsoe with Ctrl+ W");
         }
         SoundMessage::Escape => {
            println!("Escape key press:");
         }
         SoundMessage::BalanceChanged(val) => {
            self.balance_val = val;
         }
         SoundMessage::EnableEffect(idx) => {
            self.effect_tick = idx;
         }
         SoundMessage::AutomatedSoundSuppression(is_auto) => {
            self.is_auto_noise_suppression = is_auto;
         }
         SoundMessage::MuteSound => {}
         SoundMessage::MutedSound => {
            self.is_muted = !self.is_muted;
         }
         _ => {}
      }
   }
   pub fn view(&mut self) -> Element<SoundMessage> {
      let effect_enable = self.enable_sound_effect;
      let current_tick = self.effect_tick;
      let row = Column::new()
         .width(Length::Fill)
         .align_items(Align::Center)
         .spacing(10)
         .push(
            Tab::new(
               Choice::A,
               Some(self.choice),
               SoundMessage::TabSelect,
               tab_content('\u{f028}', "Output"),
            )
            .width(Length::Fill)
            .height(Length::Units(50)),
         )
         .push(
            Tab::new(
               Choice::B,
               Some(self.choice),
               SoundMessage::TabSelect,
               tab_content('\u{f130}', "Input"),
            )
            .width(Length::Fill)
            .height(Length::Units(50)),
         )
         .push(
            Tab::new(
               Choice::C,
               Some(self.choice),
               SoundMessage::TabSelect,
               tab_content('\u{f5fd}', "SoundPage Effects"),
            )
            .width(Length::Fill)
            .height(Length::Units(50)),
         );
      let output_content = Column::new()
           .spacing(10)
           .push(Text::new("Output").size(self.FONT_SIZE + 12))
           .push(
               Container::new(
                   Row::new()
                       .align_items(Align::Center)
                       .spacing(10)
                       .push(Text::new("Output Device").size(self.FONT_SIZE + 10))
                       .push(
                           PickList::new(
                               &mut self.pick_out_dev,
                               &OutputDevice::ALL[..],
                               Some(self.selected_out_dev),
                               SoundMessage::SeletedOut,
                           ).text_size(self.FONT_SIZE + 2)
                           .style(PickListStyle {})
                           .width(Length::Fill),
                       ),
               )
               .width(Length::Fill)
               .padding(10)
               .style(ContainerStyle::LightGrayCircle),
           )
           .push(
               Container::new(
                   Column::new()
                       .spacing(10)
                       .push(
                           Row::new()
                               .push(Text::new("Output Volume").size(self.FONT_SIZE + 10))
                               .push(Space::with_width(Length::Fill))
                               .push(Text::new(&format!("{}%", self.out_value.to_string())).size(self.FONT_SIZE + 10)),
                       )
                       .push(
                           Row::new()
                               .align_items(Align::Center)
                               .spacing(4)
                               .push(
                                   Button::new(
                                       &mut self.mute_out_sound,
                                       Icon::new(if self.is_muted {
                                           '\u{f026}'
                                       } else {
                                           '\u{f028}'
                                       }),
                                   )
                                   .on_press(SoundMessage::MutedSound)
                                   .style(ButtonStyle::Transparent),
                               )
                               .push(
                                   Slider::new(
                                       &mut self.slider_output,
                                       0.0..=100.0,
                                       self.out_value,
                                       SoundMessage::SoundOutChanged,
                                   ).style(SliderStyle::Circle)
                                   .step(1.0)
                                   .width(Length::Fill),
                               )
                               .push(Icon::new('\u{f027}')),
                       ),
               )
               .width(Length::Fill)
               .padding(10)
               .style(ContainerStyle::LightGrayCircle),
           )
           .push(
               Container::new(
                   Row::new()
                       .align_items(Align::Center)
                       .spacing(10)
                       .push(Text::new("Volume Boost").size(self.FONT_SIZE + 10))
                       .push(Space::with_width(Length::Fill))
                       .push(Toggler::new(
                           self.is_boost_sound,
                           String::from(""),
                           SoundMessage::EnableBoostSound,
                       )),
               )
               .padding(10)
               .style(ContainerStyle::LightGrayCircle),
           ).push(if self.is_boost_sound {
               Container::new(Text::new("If the volume is lounder than 100%, it may distort audio and be harmdul to your speaker").size(self.FONT_SIZE + 8)).padding(10)
           }else {
               Container::new(Space::with_height(Length::Units(0)))
           })
           .push(
               Container::new(
                   Column::new()
                       .spacing(10)
                       .push(Text::new("Left/Right Balance").size(self.FONT_SIZE + 10))
                       .push(
                           Slider::new(
                               &mut self.balace_state,
                               0.0..=100.0,
                               self.balance_val,
                               SoundMessage::BalanceChanged,
                           ).style(SliderStyle::Default)
                           .step(1.0),
                       )
                       .push(
                           Row::new()
                               .push(Text::new("Left").size(self.FONT_SIZE + 8))
                               .push(Space::with_width(Length::Fill))
                               .push(Text::new("Right").size(self.FONT_SIZE + 8)),
                       ), // .push(),
               )
               .padding(10)
               .style(ContainerStyle::LightGrayCircle),
           );
      let input_content = Column::new()
         .push(Container::new(Text::new("Input").size(self.FONT_SIZE + 12)))
         .spacing(10)
         .push(
            Container::new(
               Row::new()
                  .width(Length::Fill)
                  .align_items(Align::Center)
                  .spacing(10)
                  .push(Text::new("Input Devices"))
                  .push(
                     PickList::new(
                        &mut self.pick_in_dev,
                        &InputDevice::ALL[..],
                        Some(self.selected_in_dev),
                        SoundMessage::SeletedIn,
                     )
                     .style(PickListStyle {}),
                  ),
            )
            .style(ContainerStyle::LightGrayCircle)
            .padding(10),
         )
         .push(
            Container::new(
               Column::new().spacing(10).push(
                  Row::new()
                     .align_items(Align::Center)
                     .spacing(4)
                     .push(Button::new(
                        &mut self.mute_in_sound,
                        Icon::new(if self.is_in_muted {
                           '\u{f026}'
                        } else {
                           '\u{f028}'
                        }),
                     ))
                     .push(
                        Slider::new(
                           &mut self.slider_input,
                           0.0..=100.0,
                           self.input_val,
                           SoundMessage::SoundInChanged,
                        )
                        .style(SliderStyle::Default)
                        .step(1.0),
                     )
                     .push(Icon::new('\u{f028}')),
               ),
            )
            .padding(10)
            .style(ContainerStyle::LightGrayCircle),
         )
         .push(
            Container::new(
               Column::new()
                  .spacing(10)
                  .push(Text::new("Input Level"))
                  .push(
                     Row::new()
                        .push(Icon::new('\u{f192}'))
                        .spacing(10)
                        .push(
                           Slider::new(
                              &mut self.slider_input_level,
                              0.0..=100.0,
                              self.input_level,
                              SoundMessage::InputLevelChanged,
                           )
                           .step(1.0)
                           .style(SliderStyle::Default),
                        )
                        .push(Icon::new('\u{f141}')),
                  ),
            )
            .padding(10)
            .style(ContainerStyle::LightGrayCircle),
         )
         .push(
            Container::new(Toggler::new(
               self.is_auto_noise_suppression,
               String::from("Automatic Noise Suppression"),
               SoundMessage::AutomatedSoundSuppression,
            ))
            .style(ContainerStyle::LightGrayCircle)
            .padding(10),
         );
      let sound_effects = Column::new()
         .spacing(10)
         .push(
            Container::new(
               Row::new()
                  .align_items(Align::Center)
                  .spacing(10)
                  .push(Text::new("SoundPage Effects"))
                  .push(Space::with_width(Length::Fill))
                  .push(Toggler::new(
                     self.is_sound_effect,
                     String::from(""),
                     SoundMessage::SoundEffect,
                  )),
            )
            .padding(10)
            .style(ContainerStyle::LightGrayCircle),
         )
         .push(if self.is_sound_effect {
            self.sample_effects.iter_mut().enumerate().fold(
               Column::new().spacing(10).align_items(Align::Center),
               |col_sound, (idx, (enable_state, state, name))| {
                  col_sound.push(
                     Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .width(Length::Fill)
                        .push(
                           Button::new(enable_state, Row::new().push(Text::new(name.as_str())))
                              .width(Length::Fill)
                              .style(ButtonStyle::Transparent)
                              .on_press(SoundMessage::TestSoundEffect(idx)),
                        )
                        // .push(Space::with_width(Length::Fill))
                        .push(
                           Button::new(
                              state,
                              Icon::new(if effect_enable && current_tick == idx {
                                 '\u{f058}'
                              } else {
                                 '\u{f111}'
                              }),
                           )
                           .padding(4)
                           .style(ButtonStyle::Transparent)
                           .on_press(SoundMessage::EnableEffect(idx)),
                        ),
                  )
               },
            )
         } else {
            Column::new()
         });
      // f058 tick-circle
      // f111 circle
      let contnet = Column::new()
         .height(Length::Fill)
         .align_items(Align::Center)
         .padding(20)
         .push(match self.choice {
            Choice::A => Container::new(output_content),
            Choice::B => Container::new(input_content),
            Choice::C => Container::new(sound_effects),
         });
      let netsidebar_scroll = Scrollable::new(&mut self.scroll_content)
         .push(row)
         .padding(10)
         .scrollbar_width(4)
         .scroller_width(4);
      let whole_content = Row::new()
         .width(Length::Fill)
         .height(Length::Fill)
         .push(
            Container::new(netsidebar_scroll.height(Length::Fill))
               .style(ContainerStyle::White)
               .width(Length::FillPortion(4))
               .height(Length::Fill),
         )
         .push(Rule::vertical(10))
         .push(
            Container::new(contnet.height(Length::Fill))
               .width(Length::FillPortion(9))
               .height(Length::Fill)
               .style(ContainerStyle::White), // .padding(10),
         );
      let container = Container::new(whole_content)
         .width(Length::Fill)
         .center_x()
         .center_y();
      Container::new(container)
         .style(ContainerStyle::LightGray)
         .width(Length::Fill)
         .height(Length::Fill)
         .padding(10)
         .center_x()
         .center_y()
         .into()
   }
}
fn tab_content<'l>(unicode: char, name: &str) -> Row<'l, SoundMessage> {
   Row::new()
      .push(Icon::new(unicode).size(24))
      .push(Text::new(name).size(16))
      .align_items(Align::Center)
      .spacing(8)
}

mod playback {

   use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
   use std::fs::read_dir;
   use std::path::PathBuf;
   use std::time::Duration;
   // NOTE: You probably want to investigate the
   // mixer feature for real use cases.
   struct Sound {
      data: Vec<u8>,
      volume: f32,
      pos: usize,
   }

   pub fn read_directory(in_path: std::path::PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
      let mut list_sounds: Vec<PathBuf> = Vec::new();
      let sound_dir = "sounds";
      if in_path.join(sound_dir).exists() {
         for path in read_dir(in_path.join(sound_dir))? {
            let dir = path?;
            list_sounds.push(dir.path());
         }
      } else {
         make_dir(&in_path, sound_dir)?;
         let paths = read_dir(in_path)?;
         paths.for_each(|val| {
            println!("Name: {:?}", val);
         });
      }
      Ok(list_sounds)
   }
   pub fn make_dir(in_path: &std::path::PathBuf, name: &str) -> Result<bool, std::io::Error> {
      std::fs::create_dir(in_path.join(name))?;
      Ok(true)
   }

   impl AudioCallback for Sound {
      type Channel = u8;

      fn callback(&mut self, out: &mut [u8]) {
         for dst in out.iter_mut() {
            // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
            // to both fill in the silence and scale the wav data accordingly. Filling the silence
            // once the wav is finished is trivial, applying the volume is more tricky. We need to:
            // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
            // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
            // * Move the resulting range to a range centered around the value 128, the final range
            //   is [128 - 128*volume, 128 + 127*volume] – scaled and correctly positioned
            //
            // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
            // would not give correct results.
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
         }
      }
   }
   pub fn run(path: &std::path::PathBuf) -> Result<(), String> {
      let sdl_context = sdl2::init().unwrap();
      let audio_subsystem = sdl_context.audio().unwrap();
      let desired_spec = AudioSpecDesired {
         freq: Some(44_100),
         channels: Some(1), // mono
         samples: None,     // default
      };
      let device = audio_subsystem
         .open_playback(None, &desired_spec, |spec| {
            let wav = AudioSpecWAV::load_wav(path).expect("Could not load test WAV file");
            let cvt = AudioCVT::new(
               wav.format,
               wav.channels,
               wav.freq,
               spec.format,
               spec.channels,
               spec.freq,
            )
            .expect("Could not convert WAV file");
            let data = cvt.convert(wav.buffer().to_vec());
            // initialize the audio callback
            Sound {
               data: data,
               volume: 0.50,
               pos: 0,
            }
         })
         .unwrap();
      // Start playback
      device.resume();
      // std::thread::spawn(|| {
      // Play for a second
      std::thread::sleep(Duration::from_millis(1_000));
      // });

      // Device is automatically closed when dropped

      Ok(())
   }
}

mod standart_path {
   use std::path::PathBuf;
   pub fn sys_data_dir() -> Option<PathBuf> {
      Some(PathBuf::new().join("/usr/share/"))
   }
}

mod SoundBackEnd {

   pub fn initialize() {}
   pub fn volume_up(level: u32) {}
   pub fn volumn_down(level: u32) {}
   pub fn mute_sound(is_mute: bool) {}
}

#[cfg(test)]
mod tests {
   #[test]
   fn sound_test() {
      assert_eq!(2 + 2, 3);
   }
}
