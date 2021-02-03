use super::super::styles::{
   CustomButton, CustomCheckbox, CustomContainer, CustomSelect, CustomSlider,
};
use iced::{
   button, pick_list, scrollable, slider, Align, Button, Checkbox, Color, Column, Container,
   Element, Length, PickList, Row, Scrollable, Slider, Space, Text,
};
use iced_custom_widget::{table, Icon, Table, TableData, TableError, TableResult};
use iced_custom_widget::{table_column, table_columns};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smart_default::SmartDefault;

#[derive(Debug, Clone)]
pub enum SoundMessage {
   TabChanged(usize),
   SoundEffectDeviceChanged(SoundEffectDevice),
   AlertVolumeChanged(u8),
   TogglePlayStartup(bool),
   TogglePlaySoundEffects(bool),
   TogglePlayFeedback(bool),
   OutputVolumeChanged(u8),
   ToggleMute(bool),
   ToggleShowVolumn(bool),
   OutputBalanceChanged(u8),
   InputVolumeChanged(u8),
}

#[derive(Debug, Clone)]
pub struct SoundPage {
   tabbar_state: Vec<(String, button::State)>,
   current_tab_idx: usize,
   sound_effect: SoundEffect,
   sound_output: SoundOutput,
   sound_input: SoundInput,
   output_volumn_state: slider::State,
   output_volumn_value: u8,
   mute: bool,
   show_volumn: bool,
}

impl SoundPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  Sound Effects  ".to_string(), button::State::new()),
            ("  Output  ".to_string(), button::State::new()),
            ("  Input  ".to_string(), button::State::new()),
         ],
         current_tab_idx: 0,
         sound_effect: SoundEffect::new(),
         sound_output: SoundOutput::new(),
         sound_input: SoundInput::new(),
         output_volumn_state: slider::State::new(),
         output_volumn_value: 27,
         mute: false,
         show_volumn: true,
      }
   }

   pub fn update(&mut self, msg: SoundMessage) {
      match msg {
         SoundMessage::TabChanged(idx) => self.current_tab_idx = idx,
         SoundMessage::SoundEffectDeviceChanged(device) => {
            self.sound_effect.selected_sound_effect_device = device
         }
         SoundMessage::AlertVolumeChanged(val) => self.sound_effect.alert_volumn_value = val,
         SoundMessage::TogglePlayStartup(val) => self.sound_effect.play_on_startup = val,
         SoundMessage::TogglePlaySoundEffects(val) => self.sound_effect.play_sound_effects = val,
         SoundMessage::TogglePlayFeedback(val) => self.sound_effect.play_feedback = val,
         SoundMessage::OutputVolumeChanged(val) => {
            if val == 0 {
               self.mute = true;
            } else {
               self.mute = false;
            }
            self.output_volumn_value = val;
         }
         SoundMessage::ToggleMute(val) => {
            self.mute = val;
            self.output_volumn_value = if val { 0 } else { 50 };
         }
         SoundMessage::ToggleShowVolumn(val) => self.show_volumn = val,
         SoundMessage::OutputBalanceChanged(val) => self.sound_output.balance_output_value = val,
         SoundMessage::InputVolumeChanged(val) => self.sound_input.input_volume_value = val,
      }
   }

   pub fn view(&mut self) -> Element<SoundMessage> {
      let SoundPage {
         tabbar_state,
         current_tab_idx,
         sound_effect,
         sound_output,
         sound_input,
         output_volumn_state,
         output_volumn_value,
         mute,
         show_volumn,
      } = self;
      // របារផ្ទាំង
      let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
      for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
         let mut btn = Button::new(btn_state, Text::new(name.as_str()))
            .padding(5)
            .on_press(SoundMessage::TabChanged(idx));
         if *current_tab_idx == idx {
            btn = btn.style(CustomButton::SelectedTab);
         } else {
            btn = btn.style(CustomButton::Tab);
         }
         tabbar = tabbar.push(btn);
      }
      let tabbar_con = Container::new(tabbar)
         .padding(2)
         .center_x()
         .style(CustomContainer::Segment);
      let tabbar_section = Container::new(tabbar_con)
         .padding(7)
         .width(Length::Fill)
         .center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let SoundEffect {
               alert_sound_scroll,
               alert_idx,
               sound_effect_device,
               selected_sound_effect_device,
               alert_volumn_state,
               alert_volumn_value,
               play_on_startup,
               play_sound_effects,
               play_feedback,
               ..
            } = sound_effect;
            let label_alert_sound = Text::new("Select an alert sound:").size(14);
            let mut device_pane_col = Scrollable::new(alert_sound_scroll).width(Length::Fill);

            device_pane_col = AlertSound::ALL.iter().enumerate().fold(
               device_pane_col,
               |col, (idx, alert_sound)| {
                  let mut alert_con = Container::new(
                     Row::new()
                        .padding(3)
                        .align_items(Align::Center)
                        .push(Space::with_width(Length::Units(7)))
                        .push(Text::new(alert_sound.to_string())),
                  )
                  .width(Length::Fill);

                  if *alert_idx == idx {
                     alert_con = alert_con.style(CustomContainer::Background);
                  }
                  col.push(alert_con)
               },
            );

            let device_pane = Container::new(
               Column::new()
                  .push(
                     Container::new(Text::new("Names").size(12))
                        .width(Length::Fill)
                        .padding(7)
                        .style(CustomContainer::Header),
                  )
                  .push(device_pane_col),
            )
            .height(Length::Units(150))
            .style(CustomContainer::ForegroundWhite);

            let lb_sound_effect = Text::new("Play sound effects through:");
            let pl_sound_effect = PickList::new(
               sound_effect_device,
               &SoundEffectDevice::ALL[..],
               Some(*selected_sound_effect_device),
               SoundMessage::SoundEffectDeviceChanged,
            )
            .style(CustomSelect::Primary);
            let sound_effect_device = Row::new()
               .spacing(10)
               .align_items(Align::Center)
               .push(lb_sound_effect)
               .push(pl_sound_effect)
               .push(Space::with_width(Length::Units(180)));

            let lb_alert_volumn = Text::new("Alert volume:");
            let ic_volumn_down = Icon::new('\u{f027}')
               .size(27)
               .color(Color::from_rgb8(66, 66, 66));
            let slider_alert_volumn = Slider::new(
               alert_volumn_state,
               0..=100,
               *alert_volumn_value,
               SoundMessage::AlertVolumeChanged,
            )
            .width(Length::Units(227))
            .style(CustomSlider::Default);
            let ic_volumn_up = Icon::new('\u{f028}')
               .size(27)
               .color(Color::from_rgb8(66, 66, 66));
            let alert_volumn_section = Row::new()
               .spacing(10)
               .align_items(Align::Center)
               .push(lb_alert_volumn)
               .push(ic_volumn_down)
               .push(slider_alert_volumn)
               .push(ic_volumn_up);

            let chk_play_on_startup = Checkbox::new(
               *play_on_startup,
               "Play sound on startup",
               SoundMessage::TogglePlayStartup,
            )
            .spacing(10)
            .style(CustomCheckbox::Default);
            let chk_play_sound_effects = Checkbox::new(
               *play_sound_effects,
               "Play user interface sound effects",
               SoundMessage::TogglePlaySoundEffects,
            )
            .spacing(10)
            .style(CustomCheckbox::Default);
            let chk_play_feedback = Checkbox::new(
               *play_feedback,
               "Play feedback when volume is changed",
               SoundMessage::TogglePlayFeedback,
            )
            .spacing(10)
            .style(CustomCheckbox::Default);

            let container = Container::new(
               Column::new()
                  .width(Length::Fill)
                  .spacing(10)
                  .align_items(Align::Center)
                  .push(sound_effect_device)
                  .push(alert_volumn_section)
                  .push(
                     Row::new().push(Space::with_width(Length::Units(10))).push(
                        Column::new()
                           .spacing(10)
                           .push(chk_play_on_startup)
                           .push(chk_play_sound_effects)
                           .push(chk_play_feedback),
                     ),
                  ),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

            Container::new(
               Column::new()
                  .spacing(10)
                  .push(label_alert_sound)
                  .push(device_pane)
                  .push(container),
            )
         }
         1 => {
            let SoundOutput {
               output_devices,
               tb_devices_state,
               balance_output_state,
               balance_output_value,
            } = sound_output;

            let lb_output_device = Text::new("Select a device for sound output:");
            let tb_columns = table_columns![("name", "Name"), ("type", "Type"),];
            let tb_output_device =
               Table::new(tb_devices_state, tb_columns, output_devices).width(Length::Fill);

            let lb_selected_device = Text::new("Settings for the selected device:");
            let lb_balance = Text::new("Balance:");
            let slider_balance = Slider::new(
               balance_output_state,
               0..=100,
               *balance_output_value,
               SoundMessage::OutputBalanceChanged,
            )
            .width(Length::Units(175))
            .style(CustomSlider::Default);
            let balance_section = Container::new(
               Row::new()
                  .spacing(20)
                  .align_items(Align::Center)
                  .push(lb_balance)
                  .push(
                     Column::new().push(slider_balance).push(
                        Row::new()
                           .width(Length::Units(175))
                           .push(Text::new("left").size(12))
                           .push(Space::with_width(Length::Fill))
                           .push(Text::new("right").size(12)),
                     ),
                  ),
            )
            .width(Length::Fill);

            Container::new(
               Column::new()
                  .spacing(20)
                  .push(
                     Column::new()
                        .spacing(7)
                        .push(lb_output_device)
                        .push(tb_output_device),
                  )
                  .push(
                     Column::new()
                        .spacing(15)
                        .push(lb_selected_device)
                        .push(balance_section),
                  ),
            )
         }
         2 => {
            let SoundInput {
               input_devices,
               tb_devices_state,
               input_volume_state,
               input_volume_value,
            } = sound_input;

            let lb_input_device = Text::new("Select a device for sound input:");
            let tb_columns = table_columns![("name", "Name"), ("type", "Type"),];
            let tb_input_device =
               Table::new(tb_devices_state, tb_columns, input_devices).width(Length::Fill);

            let lb_selected_device = Text::new("Settings for the selected device:");
            let lb_balance = Text::new("Input volume:");
            let ic_volume_down = Icon::new('\u{f131}')
               .size(27)
               .color(Color::from_rgb8(66, 66, 66));
            let slider_balance = Slider::new(
               input_volume_state,
               0..=100,
               *input_volume_value,
               SoundMessage::InputVolumeChanged,
            )
            .width(Length::Units(200))
            .style(CustomSlider::Default);
            let ic_volume_up = Icon::new('\u{f130}')
               .size(27)
               .color(Color::from_rgb8(66, 66, 66));
            let input_vol_row = Row::new()
               .spacing(5)
               .align_items(Align::Center)
               .push(lb_balance)
               .push(ic_volume_down)
               .push(slider_balance)
               .push(ic_volume_up);
            let input_vol_con = Container::new(input_vol_row).width(Length::Fill);

            Container::new(
               Column::new()
                  .spacing(20)
                  .push(
                     Column::new()
                        .spacing(7)
                        .push(lb_input_device)
                        .push(tb_input_device),
                  )
                  .push(
                     Column::new()
                        .spacing(15)
                        .push(lb_selected_device)
                        .push(input_vol_con),
                  ),
            )
         }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // ផ្នែកខាងក្រោម
      let lb_output_volumn = Text::new("Output volume:");
      let ic_volumn_down = Icon::new('\u{f027}')
         .size(27)
         .color(Color::from_rgb8(66, 66, 66));
      let slider_output_volumn = Slider::new(
         output_volumn_state,
         0..=100,
         *output_volumn_value,
         SoundMessage::OutputVolumeChanged,
      )
      .width(Length::Units(227))
      .style(CustomSlider::Default);
      let ic_volumn_up = Icon::new('\u{f028}')
         .size(27)
         .color(Color::from_rgb8(66, 66, 66));
      let chk_mute = Checkbox::new(*mute, "Mute", SoundMessage::ToggleMute)
         .spacing(10)
         .style(CustomCheckbox::Default);
      let chk_show_volumn = Checkbox::new(
         *show_volumn,
         "Show volume in menu bar",
         SoundMessage::ToggleShowVolumn,
      )
      .spacing(10)
      .style(CustomCheckbox::Default);
      let output_volumn_row = Row::new()
         .spacing(10)
         .align_items(Align::Center)
         .push(Space::with_width(Length::Units(45)))
         .push(lb_output_volumn)
         .push(ic_volumn_down)
         .push(slider_output_volumn)
         .push(ic_volumn_up)
         .push(chk_mute);
      let bottom_col = Column::new()
         .spacing(10)
         .align_items(Align::Center)
         .push(output_volumn_row)
         .push(
            Row::new()
               .push(chk_show_volumn)
               .push(Space::with_width(Length::Units(60))),
         );
      let bottom_section = Container::new(bottom_col).padding(20).center_x();

      // មាតិកា
      let content = Column::new()
         .align_items(Align::Center)
         .push(tabbar_section)
         .push(
            tabview
               .height(Length::Fill)
               .padding(20)
               .style(CustomContainer::ForegroundGray),
         )
         .push(bottom_section);

      Container::new(content)
         .padding(20)
         .width(Length::FillPortion(15))
         .height(Length::Fill)
         .style(CustomContainer::Background)
         .into()
   }
}

#[derive(Debug, Clone, SmartDefault)]
pub enum AlertSound {
   #[default]
   Boop,
   Breeze,
   Bubble,
   Crystal,
   Funcky,
   Heroine,
   Jump,
   Mezzo,
   Pebble,
   Pluck,
   Pong,
   Sunami,
}

impl AlertSound {
   const ALL: [AlertSound; 12] = [
      AlertSound::Boop,
      AlertSound::Breeze,
      AlertSound::Bubble,
      AlertSound::Crystal,
      AlertSound::Funcky,
      AlertSound::Heroine,
      AlertSound::Jump,
      AlertSound::Mezzo,
      AlertSound::Pebble,
      AlertSound::Pluck,
      AlertSound::Pong,
      AlertSound::Sunami,
   ];
}

impl std::fmt::Display for AlertSound {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            AlertSound::Boop => "Boop",
            AlertSound::Breeze => "Breeze",
            AlertSound::Bubble => "Bubble",
            AlertSound::Crystal => "Crystal",
            AlertSound::Funcky => "Funcky",
            AlertSound::Heroine => "Heroine",
            AlertSound::Jump => "Jump",
            AlertSound::Mezzo => "Mezzo",
            AlertSound::Pebble => "Pebble",
            AlertSound::Pluck => "Pluck",
            AlertSound::Pong => "Pong",
            AlertSound::Sunami => "Sunami",
         }
      )
   }
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum SoundEffectDevice {
   #[default]
   OutputDevice,
   Speaker,
}

impl SoundEffectDevice {
   const ALL: [SoundEffectDevice; 2] =
      [SoundEffectDevice::OutputDevice, SoundEffectDevice::Speaker];
}

impl std::fmt::Display for SoundEffectDevice {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            SoundEffectDevice::OutputDevice => "Selected sound output device",
            SoundEffectDevice::Speaker => "Koompi Pro Speakers",
         }
      )
   }
}

#[derive(Debug, Clone, Default)]
pub struct SoundEffect {
   alert_sound_scroll: scrollable::State,
   alert_idx: usize,
   selected_alert_sound: AlertSound,
   sound_effect_device: pick_list::State<SoundEffectDevice>,
   selected_sound_effect_device: SoundEffectDevice,
   alert_volumn_state: slider::State,
   alert_volumn_value: u8,
   play_on_startup: bool,
   play_sound_effects: bool,
   play_feedback: bool,
}

impl SoundEffect {
   pub fn new() -> Self {
      Self {
         alert_idx: 0,
         alert_volumn_value: 100,
         play_on_startup: true,
         play_sound_effects: true,
         play_feedback: false,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct SoundOutput {
   output_devices: Vec<SoundDevice>,
   tb_devices_state: table::State,
   balance_output_state: slider::State,
   balance_output_value: u8,
}

impl SoundOutput {
   pub fn new() -> Self {
      Self {
         output_devices: vec![
            SoundDevice::new("Koompi Pro Speakers", "Built-in"),
            SoundDevice::new("External Headphones", "Headphone"),
         ],
         balance_output_value: 50,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct SoundInput {
   input_devices: Vec<SoundDevice>,
   tb_devices_state: table::State,
   input_volume_state: slider::State,
   input_volume_value: u8,
}

impl SoundInput {
   pub fn new() -> Self {
      Self {
         input_devices: vec![
            SoundDevice::new("Koompi Pro Microphone", "Built-in"),
            SoundDevice::new("External Microphone", "Microphone"),
         ],
         input_volume_value: 50,
         ..Default::default()
      }
   }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SoundDevice {
   name: String,
   _type: String,
}

impl SoundDevice {
   pub fn new(name: &str, _type: &str) -> Self {
      Self {
         name: name.to_string(),
         _type: _type.to_string(),
      }
   }
}

impl TableData for SoundDevice {
   fn get_field_value(&self, field_name: &str) -> TableResult<Value> {
      let value = match field_name {
         "name" => serde_json::to_value(&self.name),
         "type" => serde_json::to_value(&self._type),
         s => return Err(TableError::InvalidFieldName(s.to_owned())),
      };
      Ok(value.unwrap())
   }
}
