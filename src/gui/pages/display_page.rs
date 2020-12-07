use super::super::styles::{CustomButton, CustomContainer, CustomSlider, CustomCheckbox, CustomRadio};
use iced::{
   button, pick_list, scrollable, slider, Align, Length, Element, Space, Svg,
   PickList, Row, Scrollable, Slider, Text, Button, Checkbox, Column, Container, Radio
};
use smart_default::SmartDefault;

#[derive(Debug, Clone)]
pub enum DisplayMessage {
   TabChanged(usize),
   ResolutionChanged(Resolution),
   BrightnessChanged(u8),
   AutoAdjustBrightnessToggled(bool),
   TrueToneToggled(bool),
   ShowMirrorToggled(bool),
   MirrorDisplay(bool),
   DisplayPosChanged(DisplayPosition)
}

#[derive(Debug, Clone)]
pub struct DisplayPage {
   tabbar_state: Vec<(String, button::State)>,
   current_tab_idx: usize,
   display: Display,
   arrangement: Arrangement,
   color: Color,
   night_shift: NightShift,
   show_mirror: bool,
}

impl DisplayPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  Display  ".to_string(), button::State::new()),
            ("  Arrangement  ".to_string(), button::State::new()),
            ("  Color  ".to_string(), button::State::new()),
            ("  Night Shift  ".to_string(), button::State::new()),
         ],
         current_tab_idx: 0,
         display: Display::new(),
         arrangement: Arrangement::new(),
         color: Color::new(),
         night_shift: NightShift::new(),
         show_mirror: true,
      }
   }

   pub fn update(&mut self, msg: DisplayMessage) {
      match msg {
         DisplayMessage::TabChanged(idx) => self.current_tab_idx = idx,
         DisplayMessage::ResolutionChanged(val) => self.display.resolution = val,
         DisplayMessage::BrightnessChanged(val) => self.display.brightness_val = val,
         DisplayMessage::AutoAdjustBrightnessToggled(is_checked) => self.display.auto_adjust_brightness = is_checked,
         DisplayMessage::TrueToneToggled(is_checked) => self.display.true_tone = is_checked,
         DisplayMessage::ShowMirrorToggled(is_checked) => self.show_mirror = is_checked,
         DisplayMessage::MirrorDisplay(is_checked) => self.arrangement.mirror_display = is_checked,
         DisplayMessage::DisplayPosChanged(val) => self.arrangement.display_pos = val,
      }
   }

   pub fn view(&mut self) -> Element<DisplayMessage> {
      let DisplayPage {
         tabbar_state,
         current_tab_idx,
         display,
         arrangement,
         color,
         night_shift,
         show_mirror,
      } = self;

      // របារផ្ទាំង
      let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
      for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
         let mut btn = Button::new(btn_state, Text::new(name.as_str()))
            .padding(5)
            .on_press(DisplayMessage::TabChanged(idx));
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
            let Display {
               resolution,
               brightness_state,
               brightness_val,
               auto_adjust_brightness,
               true_tone,
            } = display;

            // ផ្ទាំងខាងឆ្វេង
            let logo = Svg::from_path(format!("{}/assets/images/laptop.svg",env!("CARGO_MANIFEST_DIR"))).width(Length::Units(150)).height(Length::Units(150));
            let left_pane = Container::new(logo).width(Length::FillPortion(4)).center_x();

            // ផ្ទាំងខាងស្ដាំ
            let lb_resolution = Text::new("Resolution:");
            let rd_resolution = Resolution::ALL.iter().fold(
               Column::new().spacing(10),
               |col, option| {
                  col.push(
                     Radio::new(*option, &format!("{:?}", option), Some(*resolution), DisplayMessage::ResolutionChanged).size(15).spacing(10).style(if *resolution == *option {CustomRadio::Active} else {CustomRadio::Disactive}),
                  )
               },
            );
            let resolution_row = Row::new().spacing(15)
               .push(lb_resolution)
               .push(rd_resolution);  
            
            let lb_brightness = Text::new("Brightness:");
            let slider_brightness = Slider::new(brightness_state, 0..=100, *brightness_val, DisplayMessage::BrightnessChanged).width(Length::Units(250)).style(CustomSlider::Default);
            let brightness_row = Row::new().spacing(15).align_items(Align::Center)
               .push(lb_brightness)
               .push(slider_brightness);

            let chb_auto_adjust_brightness = Checkbox::new(*auto_adjust_brightness, "Automatically adjust brightness", DisplayMessage::AutoAdjustBrightnessToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_true_tone = Checkbox::new(*true_tone, "True Tone", DisplayMessage::TrueToneToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_hint = Text::new("Automatically adapt display to make colors appear consistent in different ambient lighting conditions.").size(12);

            let right_pane = Container::new(
               Column::new().spacing(10)
               .push(resolution_row)
               .push(brightness_row)
               .push(
                  Row::new().push(Space::with_width(Length::Units(75)))
                  .push(
                     Column::new().spacing(10)
                     .push(chb_auto_adjust_brightness)
                     .push(chb_true_tone)
                     .push(txt_hint)
                  )
               )
            ).width(Length::FillPortion(6)).height(Length::Fill);

            Container::new(
               Row::new().spacing(15)
               .push(left_pane)
               .push(right_pane)
            ).padding(27).width(Length::Fill).height(Length::Fill)
         }
         1 => {
            let Arrangement {
               display_pos,
               mirror_display,
               btn_left,
               btn_top,
               btn_right,
               btn_bottom,
            } = arrangement;

            let txt_hint = Text::new("To rearrange the displays, drag them to desired position.\nTo relocate the menu bar, drag it to a different display.").size(12);
            let primary_screen = Container::new(Text::new("Primary")).width(Length::Units(100)).height(Length::Units(60)).center_x().center_y().style(CustomContainer::Primary);
            let secondary_screen = Container::new(Text::new("External")).width(Length::Units(150)).height(Length::Units(75)).center_x().center_y().style(CustomContainer::Primary);
            let display: Element<_> = match display_pos {
               DisplayPosition::Left | DisplayPosition::Right => {
                  let row = match display_pos {
                     DisplayPosition::Left => Row::new().push(secondary_screen).push(primary_screen),
                     DisplayPosition::Right => Row::new().push(primary_screen).push(secondary_screen),
                     _ => Row::new()
                  };

                  row.align_items(Align::Center).into()
               },
               DisplayPosition::Top | DisplayPosition::Bottom => {
                  let column = match display_pos {
                     DisplayPosition::Top => Column::new().push(secondary_screen).push(primary_screen),
                     DisplayPosition::Bottom => Column::new().push(primary_screen).push(secondary_screen),
                     _ => Column::new()
                  };

                  column.align_items(Align::Center).into()
               },
            };
            let btn_group = Column::new().spacing(10).align_items(Align::End)
               .push(Button::new(btn_left, Text::new("  Left Position  ")).on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Left)).style(CustomButton::Secondary))
               .push(Button::new(btn_top, Text::new("  Top Position  ")).on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Top)).style(CustomButton::Secondary))
               .push(Button::new(btn_right, Text::new("  Right Position  ")).on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Right)).style(CustomButton::Secondary))
               .push(Button::new(btn_bottom, Text::new("  Bottom Position  ")).on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Bottom)).style(CustomButton::Secondary));
            let display_con = Container::new(
               Row::new().padding(15)
               .push(
                  Container::new(display).width(Length::Fill).height(Length::Fill).center_x().center_y()
               )
               .push(
                  Container::new(btn_group).height(Length::Fill).center_y()
               )
            ).width(Length::Fill).height(Length::Fill).padding(15).center_x().center_y().style(CustomContainer::ForegroundWhite);
            let chb_mirror_display = Checkbox::new(*mirror_display, "Mirror Displays", DisplayMessage::MirrorDisplay).spacing(10).style(CustomCheckbox::Default);
           
            Container::new(
               Column::new().spacing(15)
               .push(txt_hint)
               .push(display_con)
               .push(chb_mirror_display)
            ).width(Length::Fill)
         }
         // 2 => {
         //    let SoundInput {
         //       input_devices,
         //       tb_devices_state,
         //       input_volume_state,
         //       input_volume_value,
         //    } = sound_input;

         // device_pane_col = AlertSound::ALL
         //    .iter()
         //    .enumerate()
         //    .fold(device_pane_col, |col, (idx, alert_sound)| {
         //       let mut alert_con = Container::new(
         //          Row::new()
         //             .padding(3)
         //             .align_items(Align::Center)
         //             .push(Space::with_width(Length::Units(7)))
         //             .push(Text::new(alert_sound.to_string())),
         //       )
         //       .width(Length::Fill);

         //       if *alert_idx == idx {
         //          alert_con = alert_con.style(CustomContainer::Background);
         //       }
         //       col.push(alert_con)
         //    });

         // let device_pane = Container::new(
         //    Column::new()
         //       .push(
         //          Container::new(Text::new("Names").size(12))
         //             .width(Length::Fill)
         //             .padding(7)
         //             .style(CustomContainer::Header),
         //       )
         //       .push(device_pane_col),
         // )
         // .height(Length::Units(150))
         // .style(CustomContainer::ForegroundWhite);

         // let lb_sound_effect = Text::new("Play sound effects through:");
         // let pl_sound_effect = PickList::new(
         //    sound_effect_device,
         //    &SoundEffectDevice::ALL[..],
         //    Some(*selected_sound_effect_device),
         //    DisplayMessage::SoundEffectDeviceChanged,
         // );

         //    let lb_input_device = Text::new("Select a device for sound input:");
         //    let tb_columns = table_columns![("name", "Name"), ("type", "Type"),];
         //    let tb_input_device = Table::new(tb_devices_state, tb_columns, input_devices).width(Length::Fill);

         //    let lb_selected_device = Text::new("Settings for the selected device:");
         //    let lb_balance = Text::new("Input volume:");
         //    let ic_volume_down = Icon::new('\u{f131}').size(27).color(Color::from_rgb8(66, 66, 66));
         //    let slider_balance = Slider::new(
         //       input_volume_state,
         //       0..=100,
         //       *input_volume_value,
         //       DisplayMessage::InputVolumeChanged,
         //    ).width(Length::Units(200)).style(CustomSlider::Default);
         //    let ic_volume_up = Icon::new('\u{f130}').size(27).color(Color::from_rgb8(66, 66, 66));
         //    let input_vol_row = Row::new()
         //       .spacing(5)
         //       .align_items(Align::Center)
         //       .push(lb_balance)
         //       .push(ic_volume_down)
         //       .push(slider_balance)
         //       .push(ic_volume_up);
         //    let input_vol_con = Container::new(input_vol_row).width(Length::Fill);

         //    Container::new(
         //       Column::new()
         //          .spacing(20)
         //          .push(Column::new().spacing(7).push(lb_input_device).push(tb_input_device))
         //          .push(Column::new().spacing(15).push(lb_selected_device).push(input_vol_con)),
         //    )
         // }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // ផ្នែកខាងក្រោម
      let chb_show_mirror = Checkbox::new(*show_mirror, "Show mirroring options in the menubar when available", DisplayMessage::ShowMirrorToggled).spacing(10).style(CustomCheckbox::Default);
      let bottom_section = Container::new(chb_show_mirror).padding(15);

      // មាតិកា
      let content = Column::new()   
         .push(tabbar_section)
         .push(
            tabview.height(Length::Fill).padding(20).style(CustomContainer::ForegroundGray),
         )
         .push(bottom_section);

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum Resolution {
   #[default]
   Default,
   Scaled,
}

impl Resolution {
   const ALL: [Resolution; 2] = [Resolution::Default, Resolution::Scaled];
}

#[derive(Debug, Clone, Default)]
pub struct Display {
   resolution: Resolution,
   brightness_state: slider::State,
   brightness_val: u8,
   auto_adjust_brightness: bool,
   true_tone: bool,
}

impl Display {
   pub fn new() -> Self {
      Self {
         brightness_val: 50,
         auto_adjust_brightness: false,
         true_tone: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, SmartDefault)]
pub enum DisplayPosition {
   Left,
   Right, 
   #[default]
   Top,
   Bottom
}

#[derive(Debug, Clone, Default)]
pub struct Arrangement {
   display_pos: DisplayPosition,
   mirror_display: bool,
   btn_left: button::State,
   btn_top: button::State,
   btn_right: button::State,
   btn_bottom: button::State,
}

impl Arrangement {
   pub fn new() -> Self {
      Self::default()
   }
}

#[derive(Debug, Clone, Default)]
pub struct Color {
   display_profiles: Vec<(String, button::State)>,
   selected_profile: Option<usize>,
   btn_create: button::State,
   btn_open: button::State,
   is_opened: bool,
   btn_delete: button::State,
   show_profiles: bool,
   scroll: scrollable::State
}

impl Color {
   pub fn new() -> Self {
      Self {
         display_profiles: vec![
            ("Color LCD".to_string(), button::State::new())
         ],
         show_profiles: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct NightShift {
   scedule_state: pick_list::State<Schedule>,
   selected_scedule: Schedule,
   turn_on_tmr: bool,
   color_temp_state: slider::State,
   color_temp_val: u8,
}

impl NightShift {
   pub fn new() -> Self {
      Self {
         color_temp_val: 27,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, SmartDefault)]
pub enum Schedule {
   #[default]
   Off,
   Custom,
   SunsetToSunrise,
}

impl Schedule {
   const ALL: [Schedule; 3] = [
      Schedule::Off,
      Schedule::Custom,
      Schedule::SunsetToSunrise,
   ];
}

impl std::fmt::Display for Schedule {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            Schedule::Off => "Off",
            Schedule::Custom => "Custom",
            Schedule::SunsetToSunrise => "Sunset to Sunrise",
         }
      )
   }
}