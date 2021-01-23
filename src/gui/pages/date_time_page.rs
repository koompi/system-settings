use chrono::prelude::*;
use chrono::{DateTime, Local};
use super::super::styles::{CustomButton, CustomContainer, CustomCheckbox, CustomTextInput, HOVERED, ERROR};
use iced::{
   canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
   button, scrollable, text_input, time, Align, Length, Space, Point, Rectangle, Subscription, Vector, 
   Button, Checkbox, Color, Column, Container, Element, Row, TextInput, Scrollable, Text,
};
use libkoompi::system_settings::datetime::DateTimeManager;

const DATE_FMT: &'static str = "%d/%m/%Y";
const TIME_FMT: &'static str = "%r";
const TZ_FMT: &'static str = "%z";
const DATETIME_FMT: &'static str = "%F %T";

#[derive(Debug, Clone)]
pub enum DateTimeMessage {
   TabChanged(usize),
   AutoDateTimeToggled(bool),
   TxtTimeChanged(String),
   TxtDateChanged(String),
   Tick(DateTime<Local>),
   DefaultsClicked,
   ApplyClicked,
   AutoTZToggled(bool),
   SearchTZChanged(String),
   TZSelected(usize),
}

#[derive(Debug, Default)]
pub struct DateTimePage {
   datetime_manager: DateTimeManager,
   tabbar_state: Vec<(String, button::State)>,
   current_tab_idx: usize,
   datetime_tab: DateTimeTab,
   timezone_tab: TimeZoneTab,
   selected_tz: Option<String>,
   defaults_state: button::State,
   appply_state: button::State,
   is_changed: bool,
}

impl DateTimePage {
   pub fn new() -> Self {
      let tabs = vec![
         ("  Date & Time  ".to_string(), button::State::new()),
         ("  Time Zone  ".to_string(), button::State::new()),
      ];

      match DateTimeManager::new() {
         Ok(dt_mn) => {
            let ls_tz = dt_mn.list_timezones().iter().map(|tz| (tz.to_string(), button::State::new())).collect::<Vec<(String, button::State)>>();

            Self {
               tabbar_state: tabs,
               selected_tz: Some(dt_mn.timezone().to_string()),
               datetime_tab: DateTimeTab::default(),
               timezone_tab: TimeZoneTab {
                  tz_ls: ls_tz.clone(),
                  filtered_tz_ls: ls_tz,
                  ..TimeZoneTab::default()
               },
               datetime_manager: dt_mn,
               ..Self::default()
            }
         },
         Err(err) => {
            eprintln!("{}", err); // error handling here
            Self {
               tabbar_state: tabs,
               ..Self::default()
            }
         }
      }
   }

   pub fn update(&mut self, msg: DateTimeMessage) {
      use DateTimeMessage::*;
      match msg {
         TabChanged(idx) => self.current_tab_idx = idx,
         AutoDateTimeToggled(is_checked) => {
            self.set_ntp(is_checked);
            if !(*self.datetime_manager.ntp()) {
               self.datetime_tab.temp_date_val = self.datetime_tab.clock.now.format(DATE_FMT).to_string();
               self.datetime_tab.temp_time_val = self.datetime_tab.clock.now.format(TIME_FMT).to_string();
            }
         },
         TxtTimeChanged(val) => {
            self.datetime_tab.is_time_change = true;
            self.datetime_tab.temp_time_val = val;
            self.is_changed = true;
         },
         TxtDateChanged(val) => {
            self.datetime_tab.is_date_change = true;
            self.datetime_tab.temp_date_val = val;
            self.is_changed = true;
         },
         Tick(now) => {
            if now != self.datetime_tab.clock.now {
               self.datetime_tab.clock.now = now;
               self.datetime_tab.clock.clock.clear();
            }
         }
         DefaultsClicked => {
            let current_tab = self.current_tab_idx;
            *self = Self::new();
            self.current_tab_idx = current_tab;
         },
         ApplyClicked => {
            match self.current_tab_idx {
               0 => {
                  let timezone = self.datetime_tab.clock.now.format(TZ_FMT).to_string();
                  let datetime = format!("{} {} {}", self.datetime_tab.temp_date_val, self.datetime_tab.temp_time_val, timezone);
                  match DateTime::parse_from_str(datetime.as_str(), format!("{} {} {}", DATE_FMT, TIME_FMT, TZ_FMT).as_str()) {
                     Ok(now) => {
                        self.datetime_tab.clock.now = now.into();
                        match self.datetime_manager.set_datetime(&self.datetime_tab.clock.now.format(DATETIME_FMT).to_string()) {
                           Ok(res) => {
                              if res {
                                 self.datetime_tab.is_date_change = false;
                                 self.datetime_tab.is_time_change = false;
                                 println!("Set Datetime success");
                              } else {
                                 eprintln!("Fail to set Datetime");
                              }
                           },
                           Err(err) => eprintln!("{}", err)
                        }
                     },
                     Err(err) => eprintln!("{}", err)
                  }
               },
               1 => {
                  if let Some(selected_tz) = &self.selected_tz {
                     match self.datetime_manager.set_timezone(selected_tz) {
                        Ok(res) => {
                           if res {
                              println!("Set timezone success");
                           } else {
                              eprintln!("Fail to set timezone");
                           }
                        },
                        Err(err) => eprintln!("{}", err)
                     }
                  }
               },
               _ => {}
            }
            
            self.is_changed = false;
         },
         AutoTZToggled(is_checked) => self.set_ntp(is_checked),
         SearchTZChanged(text) => {
            self.timezone_tab.search_val = text;
            self.timezone_tab.filtered_tz_ls = self.timezone_tab.tz_ls.iter()
            .filter(|tz| tz.0.to_lowercase().contains(&self.timezone_tab.search_val.to_lowercase()))
            .cloned()
            .collect();
         },
         TZSelected(idx) => {
            self.selected_tz = Some(self.timezone_tab.filtered_tz_ls[idx].0.clone());
            self.is_changed = true;
         },
      }
   }

   pub fn subscription(&self) -> Subscription<DateTimeMessage> {
      if *self.datetime_manager.ntp() || !self.datetime_tab.is_time_change {
         time::every(std::time::Duration::from_millis(250)).map(|_| DateTimeMessage::Tick(Local::now()))
      } else {
         Subscription::none()
      }
   }

   pub fn view(&mut self) -> Element<DateTimeMessage> {
      let DateTimePage {
         datetime_manager,
         tabbar_state,
         current_tab_idx,
         datetime_tab,
         timezone_tab, 
         selected_tz,
         defaults_state,
         appply_state,
         is_changed,
      } = self;

      // របារផ្ទាំង
      let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
      for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
         let mut btn = Button::new(btn_state, Text::new(name.as_str())).padding(5).on_press(DateTimeMessage::TabChanged(idx));
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
            let DateTimeTab {
               clock,
               txt_time_state,
               temp_time_val,
               is_time_change,
               txt_date_state,
               temp_date_val, 
               is_date_change,
            } = datetime_tab;

            let chb_auto_datetime = Checkbox::new(*datetime_manager.ntp(), "Set date and time automatically", DateTimeMessage::AutoDateTimeToggled).spacing(10).style(CustomCheckbox::Default);
            let date = clock.now.date().format(DATE_FMT).to_string();
            let txt_date: Element<_> = if *datetime_manager.ntp() {
               Text::new(date).into()
            } else {
               TextInput::new(txt_date_state, "", if *is_date_change {temp_date_val} else {&date}, DateTimeMessage::TxtDateChanged).padding(7).width(Length::Units(70)).style(CustomTextInput::Default).into()
            };
            let time = clock.now.time().format(TIME_FMT).to_string();
            let txt_time: Element<_> = if *datetime_manager.ntp() {
               Text::new(time).size(14).into()
            } else {
               TextInput::new(txt_time_state, "", if *is_time_change {temp_time_val} else {&time}, DateTimeMessage::TxtTimeChanged).padding(7).width(Length::Units(75)).style(CustomTextInput::Default).into()
            };
            let calendar_con = Container::new(Text::new("Calendar")).width(Length::Units(127)).height(Length::Units(127)).center_x().center_y().style(CustomContainer::ForegroundWhite);
            let calendar_sec = Container::new(
               Column::new().spacing(20).align_items(Align::Center)
               .push(calendar_con)
               .push(txt_date)
            );

            let canvas_clock = Canvas::new(clock).width(Length::Units(127)).height(Length::Units(127));
            let time_sec = Container::new(
               Column::new().spacing(20).align_items(Align::Center)
               .push(canvas_clock)
               .push(txt_time)
            );
            let datetime_sec = Container::new(
               Row::new().spacing(75).align_items(Align::Center)
               .push(calendar_sec)
               .push(time_sec)
            ).width(Length::Fill).height(Length::Fill).center_x().center_y();

            let txt_format_hint = Text::new("To set date and time formats, use Language & Region preferences in the sidebar.");

            Container::new(
               Column::new().spacing(20)
               .push(chb_auto_datetime)
               .push(datetime_sec)
               .push(txt_format_hint)
            ).width(Length::Fill).height(Length::Fill)
         }
         1 => {
            let TimeZoneTab {
               search_state,
               search_val,
               filtered_tz_ls,
               scroll,
               ..
            } = timezone_tab;

            let chb_auto_tz = Checkbox::new(*datetime_manager.ntp(), "Set time zone automatically using current location", DateTimeMessage::AutoTZToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_tz_hint = Text::new("To change the local time zone, select your area from the list below then click Apply.");
            let txt_current_tz = Text::new(format!("Current local time zone: {}", datetime_manager.timezone()));

            let input_search_tz = TextInput::new(search_state, "Search time zone...", &search_val, DateTimeMessage::SearchTZChanged).padding(10).style(CustomTextInput::Default);
            let scroll_tz = filtered_tz_ls.iter_mut().enumerate().fold(Scrollable::new(scroll).height(Length::Fill).padding(7).spacing(4), |scrollable, (idx, (tz, state))| {
               let mut btn = Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Text::new(tz.clone()))).width(Length::Fill).style(
                  if datetime_manager.timezone() == tz {CustomButton::Selected} 
                  else if let Some(selected_tz) = selected_tz {
                     if selected_tz == tz {CustomButton::Hovered}
                     else {CustomButton::Text}
                  }
                  else {CustomButton::Text}
               );
               if !(*datetime_manager.ntp()) {
                  btn = btn.on_press(DateTimeMessage::TZSelected(idx));
               }
               scrollable.push(btn)
            });
            let tz_pane = Container::new(
               Column::new()
                  .push(
                     Container::new(Text::new("Time Zones").size(12)).width(Length::Fill).padding(7).style(CustomContainer::Header),
                  )
                  .push(scroll_tz),
            ).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            Container::new(
               Column::new().spacing(15)
               .push(chb_auto_tz)
               .push(
                  Column::new().spacing(10)
                  .push(txt_tz_hint)
                  .push(txt_current_tz)
               )
               .push(input_search_tz)
               .push(tz_pane)
            ).width(Length::Fill)
         }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // ផ្នែកខាងក្រោម
      let btn_defaults = Button::new(defaults_state, Text::new("  Defaults  ")).on_press(DateTimeMessage::DefaultsClicked).style(CustomButton::Default);
      let mut btn_apply = Button::new(appply_state, Text::new("  Apply  ")).style(CustomButton::Primary);
      if *is_changed {
         btn_apply = btn_apply.on_press(DateTimeMessage::ApplyClicked);
      }
      let bottom_row = Row::new().padding(15).spacing(20).align_items(Align::Center)
         .push(btn_defaults)
         .push(Space::with_width(Length::Fill))
         .push(btn_apply);
      let bottom_section = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);

      // មាតិកា
      let content = Column::new().width(Length::Fill).align_items(Align::Center)
         .push(tabbar_section)
         .push(tabview.height(Length::Fill).padding(25).style(CustomContainer::ForegroundGray))
         .push(bottom_section);

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }

   fn set_ntp(&mut self, ntp: bool) {
      match self.datetime_manager.set_ntp(ntp) {
         Ok(res) => {
            if res {
               println!("Set NTP success");
            } else {
               eprintln!("Fail to set NTP");
            }
         },
         Err(err) => eprintln!("{}", err)
      }
   }
}

#[derive(Debug, Default)]
pub struct DateTimeTab {
   // auto_datetime: bool,
   clock: Clock,
   txt_time_state: text_input::State,
   temp_time_val: String,
   is_time_change: bool,
   txt_date_state: text_input::State,
   temp_date_val: String,
   is_date_change: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TimeZoneTab {
   // auto_tz: bool,
   search_state: text_input::State,
   search_val: String,
   tz_ls: Vec<(String, button::State)>,
   filtered_tz_ls: Vec<(String, button::State)>,
   scroll: scrollable::State,
}

#[derive(Debug)]
struct Clock {
   now: DateTime<Local>,
   clock: Cache,
}

impl Default for Clock {
   fn default() -> Self {
      Self {
         now: Local::now(),
         clock: Default::default(),
      }
   }
}

impl canvas::Program<DateTimeMessage> for Clock {
   fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
      let clock = self.clock.draw(bounds.size(), |frame| {
         let center = frame.center();
         let radius = frame.width().min(frame.height()) / 2.0;

         let background = Path::circle(center, radius);
         let foreground = Path::circle(center, radius * 0.9);
         frame.fill(&background, Color {a: 0.7, ..HOVERED});
         frame.fill(&foreground, Color::WHITE);
         let circle_center = Path::circle(center, radius * 0.05);
         frame.fill(&circle_center, Color::BLACK);

         let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
         let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));
         let thin_stroke = Stroke {
            width: 1.0,
            color: ERROR,
            line_cap: LineCap::Round,
            ..Stroke::default()
         };
         let wide_stroke = Stroke {
            width: thin_stroke.width * 2.5,
            color: Color::BLACK,
            ..thin_stroke
         };
         frame.translate(Vector::new(center.x, center.y));

         frame.with_save(|frame| {
            frame.rotate(hand_rotation(self.now.hour(), 12));
            frame.stroke(&short_hand, wide_stroke);
         });

         frame.with_save(|frame| {
            frame.rotate(hand_rotation(self.now.minute(), 60));
            frame.stroke(&long_hand, wide_stroke);
         });

         frame.with_save(|frame| {
            frame.rotate(hand_rotation(self.now.second(), 60));
            frame.stroke(&long_hand, thin_stroke);
         });
      });

      vec![clock]
   }
}

fn hand_rotation(n: u32, total: u32) -> f32 {
   let turns = n as f32 / total as f32;

   2.0 * std::f32::consts::PI * turns
}