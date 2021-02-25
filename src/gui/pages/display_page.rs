use super::super::styles::{
   CustomButton, CustomCheckbox, CustomContainer, CustomRadio, CustomSlider,
};

use iced::{
   button, pick_list, scrollable, slider, Align, Button, Checkbox, Column, Container, Element,
   Length, PickList, Radio, Row, Scrollable, Slider, Space, Svg, Text,
};
use smart_default::SmartDefault;
use crate::{gui::styles::CustomSelect, helpers::ROOT_PATH};
#[derive(Debug, Clone)]
pub enum DisplayMessage {
   TabChanged(usize),
   ResolutionChanged(Resolution),
   BrightnessChanged(u8),
   AutoAdjustBrightnessToggled(bool),
   TrueToneToggled(bool),
   ShowMirrorToggled(bool),
   MirrorDisplay(bool),
   DisplayPosChanged(DisplayPosition),
   DisplayProfileChanged(usize),
   ShowProfilesToggled(bool),
   BtnCreateClicked,
   BtnOpenClicked(usize),
   BtnDeleteClicked(usize),
   ScheduleChanged(Schedule),
   TurnNightShiftTmr(bool),
   ColorTempChanged(u8),
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
         DisplayMessage::AutoAdjustBrightnessToggled(is_checked) => {
            self.display.auto_adjust_brightness = is_checked
         }
         DisplayMessage::TrueToneToggled(is_checked) => self.display.true_tone = is_checked,
         DisplayMessage::ShowMirrorToggled(is_checked) => self.show_mirror = is_checked,
         DisplayMessage::MirrorDisplay(is_checked) => self.arrangement.mirror_display = is_checked,
         DisplayMessage::DisplayPosChanged(val) => self.arrangement.display_pos = val,
         DisplayMessage::DisplayProfileChanged(idx) => self.color.selected_profile = Some(idx),
         DisplayMessage::ShowProfilesToggled(is_checked) => self.color.show_profiles = is_checked,
         DisplayMessage::BtnCreateClicked => self
            .color
            .display_profiles
            .push(("New Profile".to_string(), button::State::new())),
         DisplayMessage::BtnOpenClicked(selected_idx) => {
            self.color.opened_display = format!("Open Profile Index: {}", selected_idx)
         }
         DisplayMessage::BtnDeleteClicked(selected_idx) => {
            self.color.display_profiles.remove(selected_idx);
            self.color.selected_profile = None;
         }
         DisplayMessage::ScheduleChanged(val) => self.night_shift.selected_schedule = val,
         DisplayMessage::TurnNightShiftTmr(is_checked) => self.night_shift.turn_on_tmr = is_checked,
         DisplayMessage::ColorTempChanged(val) => self.night_shift.color_temp_val = val,
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
            let Display {
               resolution,
               brightness_state,
               brightness_val,
               auto_adjust_brightness,
               true_tone,
            } = display;

            // ផ្ទាំងខាងឆ្វេង
            let logo = Svg::from_path(format!("{}/assets/images/laptop.svg", ROOT_PATH()))
               .width(Length::Units(150))
               .height(Length::Units(150));
            let left_pane = Container::new(logo)
               .width(Length::FillPortion(4))
               .center_x();

            // ផ្ទាំងខាងស្ដាំ
            let lb_resolution = Text::new("Resolution:");
            let rd_resolution =
               Resolution::ALL
                  .iter()
                  .fold(Column::new().spacing(10), |col, option| {
                     col.push(
                        Radio::new(
                           *option,
                           &format!("{:?}", option),
                           Some(*resolution),
                           DisplayMessage::ResolutionChanged,
                        )
                        .size(15)
                        .spacing(10)
                        .style(if *resolution == *option {
                           CustomRadio::Active
                        } else {
                           CustomRadio::Disactive
                        }),
                     )
                  });
            let resolution_row = Row::new()
               .spacing(15)
               .push(lb_resolution)
               .push(rd_resolution);
            let lb_brightness = Text::new("Brightness:");
            let slider_brightness = Slider::new(
               brightness_state,
               0..=100,
               *brightness_val,
               DisplayMessage::BrightnessChanged,
            )
            .width(Length::Units(250))
            .style(CustomSlider::Default);
            let brightness_row = Row::new()
               .spacing(15)
               .align_items(Align::Center)
               .push(lb_brightness)
               .push(slider_brightness);

            let chb_auto_adjust_brightness = Checkbox::new(
               *auto_adjust_brightness,
               "Automatically adjust brightness",
               DisplayMessage::AutoAdjustBrightnessToggled,
            )
            .spacing(10)
            .style(CustomCheckbox::Default);
            let chb_true_tone =
               Checkbox::new(*true_tone, "True Tone", DisplayMessage::TrueToneToggled)
                  .spacing(10)
                  .style(CustomCheckbox::Default);
            let txt_hint = Text::new("Automatically adapt display to make colors appear consistent in different ambient lighting conditions.").size(12);

            let right_pane = Container::new(
               Column::new()
                  .spacing(10)
                  .push(resolution_row)
                  .push(brightness_row)
                  .push(
                     Row::new().push(Space::with_width(Length::Units(75))).push(
                        Column::new()
                           .spacing(10)
                           .push(chb_auto_adjust_brightness)
                           .push(chb_true_tone)
                           .push(txt_hint),
                     ),
                  ),
            )
            .width(Length::FillPortion(6))
            .height(Length::Fill);

            Container::new(Row::new().spacing(15).push(left_pane).push(right_pane))
               .padding(27)
               .width(Length::Fill)
               .height(Length::Fill)
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
            let primary_screen = Container::new(Text::new("Primary"))
               .width(Length::Units(100))
               .height(Length::Units(60))
               .center_x()
               .center_y()
               .style(CustomContainer::Primary);
            let secondary_screen = Container::new(Text::new("External"))
               .width(Length::Units(150))
               .height(Length::Units(75))
               .center_x()
               .center_y()
               .style(CustomContainer::Primary);
            let display: Element<_> = match display_pos {
               DisplayPosition::Left | DisplayPosition::Right => {
                  let row = match display_pos {
                     DisplayPosition::Left => {
                        Row::new().push(secondary_screen).push(primary_screen)
                     }
                     DisplayPosition::Right => {
                        Row::new().push(primary_screen).push(secondary_screen)
                     }
                     _ => Row::new(),
                  };

                  row.align_items(Align::Center).into()
               }
               DisplayPosition::Top | DisplayPosition::Bottom => {
                  let column = match display_pos {
                     DisplayPosition::Top => {
                        Column::new().push(secondary_screen).push(primary_screen)
                     }
                     DisplayPosition::Bottom => {
                        Column::new().push(primary_screen).push(secondary_screen)
                     }
                     _ => Column::new(),
                  };

                  column.align_items(Align::Center).into()
               }
            };
            let btn_group = Column::new()
               .spacing(10)
               .align_items(Align::End)
               .push(
                  Button::new(btn_left, Text::new("  Left Position  "))
                     .on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Left))
                     .style(CustomButton::Text),
               )
               .push(
                  Button::new(btn_top, Text::new("  Top Position  "))
                     .on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Top))
                     .style(CustomButton::Text),
               )
               .push(
                  Button::new(btn_right, Text::new("  Right Position  "))
                     .on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Right))
                     .style(CustomButton::Text),
               )
               .push(
                  Button::new(btn_bottom, Text::new("  Bottom Position  "))
                     .on_press(DisplayMessage::DisplayPosChanged(DisplayPosition::Bottom))
                     .style(CustomButton::Text),
               );
            let display_con = Container::new(
               Row::new()
                  .padding(15)
                  .push(
                     Container::new(display)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x()
                        .center_y(),
                  )
                  .push(Container::new(btn_group).height(Length::Fill).center_y()),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(15)
            .center_x()
            .center_y()
            .style(CustomContainer::ForegroundWhite);
            let chb_mirror_display = Checkbox::new(
               *mirror_display,
               "Mirror Displays",
               DisplayMessage::MirrorDisplay,
            )
            .spacing(10)
            .style(CustomCheckbox::Default);
            Container::new(
               Column::new()
                  .spacing(15)
                  .push(txt_hint)
                  .push(display_con)
                  .push(chb_mirror_display),
            )
            .width(Length::Fill)
         }
         2 => {
            let Color {
               display_profiles,
               selected_profile,
               btn_create_state,
               btn_open_state,
               btn_delete_state,
               show_profiles,
               scroll,
               opened_display,
            } = color;

            // ផ្ទាំងខាងឆ្វេង
            let lb_display_profile = Text::new("Display profile:");
            let profile_pane = display_profiles.iter_mut().enumerate().fold(
               Scrollable::new(scroll).spacing(4).scroller_width(4).scrollbar_width(4),
               |scrollable, (idx, (name, state))| {
                  let mut profile = Button::new(state, Text::new(name.to_string()))
                     .width(Length::Fill)
                     .on_press(DisplayMessage::DisplayProfileChanged(idx));
                  profile = if let Some(selected_idx) = selected_profile {
                     if *selected_idx == idx {
                        profile.style(CustomButton::Selected)
                     } else {
                        profile.style(CustomButton::Text)
                     }
                  } else {
                     profile.style(CustomButton::Text)
                  };
                  scrollable.push(profile)
               },
            );
            let chb_show_profile = Checkbox::new(
               *show_profiles,
               "Show profiles for this display only",
               DisplayMessage::ShowProfilesToggled,
            )
            .spacing(10)
            .style(CustomCheckbox::Default);
            let left_pane = Container::new(
               Column::new()
                  .spacing(15)
                  .push(lb_display_profile)
                  .push(
                     Container::new(profile_pane)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .padding(7)
                        .style(CustomContainer::ForegroundWhite),
                  )
                  .push(chb_show_profile),
            )
            .width(Length::FillPortion(5))
            .height(Length::Fill);

            // ផ្ទាំងខាងស្ដាំ
            let btn_create = Button::new(btn_create_state, Text::new("  New Profile  "))
               .on_press(DisplayMessage::BtnCreateClicked)
               .style(CustomButton::Default);
            let mut btn_open = Button::new(btn_open_state, Text::new("  Open Profile  "))
               .style(CustomButton::Default);
            let mut btn_delete = Button::new(btn_delete_state, Text::new("  Delete Profile  "))
               .style(CustomButton::Default);
            if let Some(selected_idx) = selected_profile {
               btn_open = btn_open.on_press(DisplayMessage::BtnOpenClicked(*selected_idx));
               btn_delete = btn_delete.on_press(DisplayMessage::BtnDeleteClicked(*selected_idx));
            }

            let btn_group = Column::new()
               .spacing(15)
               .align_items(Align::End)
               .push(btn_create)
               .push(
                  Row::new()
                     .spacing(15)
                     .align_items(Align::Center)
                     .push(Text::new(opened_display.as_str()))
                     .push(btn_open),
               )
               .push(btn_delete);
            let right_pane = Container::new(btn_group)
               .width(Length::FillPortion(5))
               .height(Length::Fill)
               .align_x(Align::End)
               .center_y();

            Container::new(Row::new().spacing(15).push(left_pane).push(right_pane))
         }
         3 => {
            let NightShift {
               schedule_state,
               selected_schedule,
               turn_on_tmr,
               color_temp_state,
               color_temp_val,
            } = night_shift;

            let txt_hint = Text::new("Night Shift automatically shifts the colors of display to warmer end of the color spectrum after dark. This may help you get a better night's sleep.");

            let lb_schedule = Text::new("Schedule:");

            let pl_schedule = PickList::new(schedule_state, &Schedule::ALL[..], Some(*selected_schedule), DisplayMessage::ScheduleChanged).style(CustomSelect::Primary);
            let schedule_row = Row::new().spacing(15).align_items(Align::Center)
               .push(lb_schedule)
               .push(pl_schedule);


            let lb_manual = Text::new("Manual:");
            let chb_manual = Checkbox::new(
               *turn_on_tmr,
               "Turn on until Tomorrow",
               DisplayMessage::TurnNightShiftTmr,
            )
            .style(CustomCheckbox::Default);
            let manual_row = Row::new()
               .spacing(15)
               .align_items(Align::Center)
               .push(lb_manual)
               .push(chb_manual);

            let lb_color_temp = Text::new("Color Temperature:");

            let slider_color_temp = Slider::new(
               color_temp_state,
               0..=100,
               *color_temp_val,
               DisplayMessage::ColorTempChanged,
            )
            .width(Length::Units(250))
            .style(CustomSlider::Default);
            let color_temp_row = Row::new()
               .spacing(15)
               .align_items(Align::Center)
               .push(lb_color_temp)
               .push(slider_color_temp);
            Container::new(
               Row::new()
                  .push(Space::with_width(Length::FillPortion(2)))
                  .push(
                     Container::new(
                        Column::new().spacing(30).push(txt_hint).push(
                           Column::new()
                              .spacing(10)
                              .align_items(Align::Center)
                              .push(schedule_row)
                              .push(
                                 Row::new()
                                    .push(Space::with_width(Length::Units(40)))
                                    .push(manual_row),
                              )
                              .push(
                                 Row::new()
                                    .push(Space::with_width(Length::Units(80)))
                                    .push(color_temp_row),
                              ),
                        ),

                     )
                     .width(Length::FillPortion(6)),
                  )
                  .push(Space::with_width(Length::FillPortion(2))),
            )
            .center_x()
         }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // ផ្នែកខាងក្រោម
      let chb_show_mirror = Checkbox::new(
         *show_mirror,
         "Show mirroring options in the menubar when available",
         DisplayMessage::ShowMirrorToggled,
      )
      .spacing(10)
      .style(CustomCheckbox::Default);
      let bottom_section = Container::new(chb_show_mirror).padding(15);

      // មាតិកា
      let content = Column::new()
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
   Bottom,
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
   btn_create_state: button::State,
   btn_open_state: button::State,
   btn_delete_state: button::State,
   show_profiles: bool,
   scroll: scrollable::State,
   opened_display: String,
}

impl Color {
   pub fn new() -> Self {
      Self {
         display_profiles: vec![("Color LCD".to_string(), button::State::new())],
         show_profiles: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct NightShift {
   schedule_state: pick_list::State<Schedule>,
   selected_schedule: Schedule,
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

#[derive(Debug, Clone, Copy, SmartDefault, Eq, PartialEq)]
pub enum Schedule {
   #[default]
   Off,
   Custom,
   SunsetToSunrise,
}

impl Schedule {
   const ALL: [Schedule; 3] = [Schedule::Off, Schedule::Custom, Schedule::SunsetToSunrise];
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
