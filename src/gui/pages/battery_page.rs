use crate::helpers::ROOT_PATH;
use iced::{button, pick_list, slider, Align, Button, Checkbox, Column, Container, Element, Length, PickList, Row, Slider, Space, Svg, Text};
use iced_custom_widget::{number_input, Icon, NumberInput};

use super::super::styles::{CustomButton, CustomCheckbox, CustomContainer, CustomSelect, CustomSlider};
use chrono::prelude::*;
use libkoompi::system_settings::devices::Brightness;
use smart_default::SmartDefault;
#[derive(Debug, Clone)]
pub enum BatteryMessage {
   SidebarChanged(usize),
   InnerTabChanged(usize),
   ShowBatteryStatusToggled(bool),
   BrightNessChanged(u8),
   TurnDisplayOffBatteryChanged(u8),
   SlightlyDimDisplayToggled(bool),
   PowerNapWhileBatteryToggled(bool),
   OptVideoStreamToggled(bool),
   OptBatteryChargedToggled(bool),
   BatteryHealthClicked,
   RestoreDefaultBatteryClicked,
   TurnDisplayOffPowerChanged(u8),
   PreventFromSleepToggled(bool),
   WakeNetworkAccessToggled(bool),
   PowerNapWhilePowerToggled(bool),
   RestoreDefaultPowerClicked,
   StartUpToggled(bool),
   StartUpRepeatChanged(RepeatDays),
   StartUpHourChanged(u8),
   StartUpMinuteChanged(u8),
   SleepToggled(bool),
   SleepRepeatChanged(RepeatDays),
   SleepHourChanged(u8),
   SleepMinuteChanged(u8),
   RestorePrevClicked,
   ApplyScheduleClicked,
}

#[derive(Debug, Clone)]
pub struct BatteryPage {
   sidebar_state: Vec<(char, String, button::State)>,
   current_sidebar_tab_idx: usize,
   current_battery: u8,
   current_time: String,
   show_battery_status: bool,
   usage: Usage,
   battery_tab: BatteryTab,
   power_adapter: PowerAdapter,
   brightnessctl: Brightness,
   schedule: Schedule,
}

impl BatteryPage {
   pub fn new() -> Self {
      let now = chrono::Local::now();
      let (is_pm, hour) = now.time().hour12();
      Self {
         sidebar_state: vec![
            ('\u{f080}', "Usage".to_string(), button::State::new()),
            ('\u{f240}', "Battery".to_string(), button::State::new()),
            ('\u{f5ba}', "Power Adapter".to_string(), button::State::new()),
            ('\u{f133}', "Schedule".to_string(), button::State::new()),
         ],
         current_sidebar_tab_idx: 0,
         current_battery: 55,
         current_time: format!("Today, {:02}:{:02} {}", hour, now.minute(), if is_pm { "PM" } else { "AM" }),
         show_battery_status: true,
         usage: Usage::new(),
         battery_tab: BatteryTab::new(),
         power_adapter: PowerAdapter::new(),
         brightnessctl: Brightness::new(),
         schedule: Schedule::new(),
      }
   }

   pub fn update(&mut self, msg: BatteryMessage) {
      match msg {
         BatteryMessage::SidebarChanged(idx) => self.current_sidebar_tab_idx = idx,
         BatteryMessage::InnerTabChanged(idx) => self.usage.current_tab_idx = idx,
         BatteryMessage::ShowBatteryStatusToggled(is_checked) => self.show_battery_status = is_checked,
         BatteryMessage::TurnDisplayOffBatteryChanged(val) => self.battery_tab.turn_display_off_after_val = val,
         BatteryMessage::BrightNessChanged(val) => {
            self.battery_tab.brigthness_value = val;
            match self.brightnessctl.login1_set_brightness(self.battery_tab.brigthness_value as u32) {
               Ok(()) => {}
               Err(e) => eprint!("Error: {:?}", e),
            }
         }
         BatteryMessage::SlightlyDimDisplayToggled(is_checked) => self.battery_tab.slightly_dim_display = is_checked,
         BatteryMessage::PowerNapWhileBatteryToggled(is_checked) => self.battery_tab.enable_power_nap = is_checked,
         BatteryMessage::OptVideoStreamToggled(is_checked) => self.battery_tab.opt_video_stream = is_checked,
         BatteryMessage::OptBatteryChargedToggled(is_checked) => self.battery_tab.opt_battery_charging = is_checked,
         BatteryMessage::BatteryHealthClicked => self.battery_tab.is_battery_health_clicked = true,
         BatteryMessage::RestoreDefaultBatteryClicked => self.battery_tab = BatteryTab::new(),
         BatteryMessage::TurnDisplayOffPowerChanged(val) => self.power_adapter.turn_display_off_after_val = val,
         BatteryMessage::PreventFromSleepToggled(is_checked) => self.power_adapter.prevent_from_sleep = is_checked,
         BatteryMessage::WakeNetworkAccessToggled(is_checked) => self.power_adapter.wake_network_access = is_checked,
         BatteryMessage::PowerNapWhilePowerToggled(is_checked) => self.power_adapter.enable_power_nap = is_checked,
         BatteryMessage::RestoreDefaultPowerClicked => self.power_adapter = PowerAdapter::new(),
         BatteryMessage::StartUpToggled(is_checked) => {
            self.schedule.startup = is_checked;
            self.schedule.is_changed = true
         }
         BatteryMessage::StartUpRepeatChanged(val) => {
            self.schedule.startup_repeat_val = val;
            self.schedule.is_changed = true
         }
         BatteryMessage::StartUpHourChanged(val) => {
            self.schedule.startup_time_state.hour_val = val;
            self.schedule.is_changed = true
         }
         BatteryMessage::StartUpMinuteChanged(val) => {
            self.schedule.startup_time_state.minute_val = val;
            self.schedule.is_changed = true
         }
         BatteryMessage::SleepToggled(is_checked) => {
            self.schedule.sleep = is_checked;
            self.schedule.is_changed = true
         }
         BatteryMessage::SleepRepeatChanged(val) => {
            self.schedule.sleep_repeat_val = val;
            self.schedule.is_changed = true
         }
         BatteryMessage::SleepHourChanged(val) => {
            self.schedule.sleep_time_state.hour_val = val;
            self.schedule.is_changed = true
         }
         BatteryMessage::SleepMinuteChanged(val) => {
            self.schedule.sleep_time_state.minute_val = val;
            self.schedule.is_changed = true
         }
         BatteryMessage::RestorePrevClicked => self.schedule = Schedule::new(),
         BatteryMessage::ApplyScheduleClicked => self.schedule.is_changed = false,
      }
   }

   pub fn view(&mut self) -> Element<BatteryMessage> {
      let BatteryPage {
         sidebar_state,
         current_sidebar_tab_idx,
         current_battery,
         current_time,
         show_battery_status,
         usage,
         battery_tab,
         power_adapter,
         schedule,
         brightnessctl,
      } = self;

      // របារចំហៀង
      let icon = Svg::from_path(format!("{}/assets/images/battery.svg", ROOT_PATH())).height(Length::Units(127));
      let icon_con = Container::new(icon).padding(27).width(Length::Fill).center_x();
      let battery_level = Text::new(format!("Current Level: {}%", current_battery)).size(15);
      let last_charged = Text::new("Last charged to 100%").size(12);
      let last_charged_date = Text::new(current_time.as_str()).size(12);
      let sidebar_tabs = sidebar_state.iter_mut().enumerate().fold(Column::new().spacing(4), |col, (idx, (icon, name, state))| {
         col.push(
            Button::new(state, Row::new().spacing(4).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(name.as_str())))
               .width(Length::Fill)
               .on_press(BatteryMessage::SidebarChanged(idx))
               .style(if *current_sidebar_tab_idx == idx { CustomButton::SelectedSidebar } else { CustomButton::Sidebar }),
         )
      });
      let sidebar_col = Column::new()
         .spacing(20)
         .align_items(Align::Center)
         .push(icon_con)
         .push(Column::new().spacing(7).align_items(Align::Center).push(battery_level).push(last_charged).push(last_charged_date))
         .push(sidebar_tabs);
      let sidebar = Container::new(sidebar_col).padding(7).width(Length::FillPortion(2)).height(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match current_sidebar_tab_idx {
         0 => {
            let Usage { tabbar_state, current_tab_idx } = usage;

            let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
            for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
               let mut btn = Button::new(btn_state, Text::new(name.as_str())).padding(5).on_press(BatteryMessage::InnerTabChanged(idx));
               if *current_tab_idx == idx {
                  btn = btn.style(CustomButton::SelectedTab);
               } else {
                  btn = btn.style(CustomButton::Tab);
               }
               tabbar = tabbar.push(btn);
            }
            let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
            let tabbar_section = Container::new(tabbar_con).padding(7).width(Length::Fill).center_x();

            let inner_tabview = match current_tab_idx {
               0 => {
                  let lb_battery_level = Text::new("Battery Level");
                  let line_chart_1 = Svg::from_path(format!("{}/assets/images/line-chart.svg", ROOT_PATH())).height(Length::Fill);
                  let battery_level_sec = Container::new(Column::new().spacing(10).push(lb_battery_level).push(line_chart_1)).height(Length::FillPortion(5));

                  let lb_screen_on = Text::new("Screen On Usage");
                  let line_chart_2 = Svg::from_path(format!("{}/assets/images/line-chart.svg", ROOT_PATH())).height(Length::Fill);
                  let screen_on_sec = Container::new(Column::new().spacing(10).push(lb_screen_on).push(line_chart_2)).height(Length::FillPortion(5));
                  Container::new(Column::new().spacing(20).push(battery_level_sec).push(screen_on_sec)).height(Length::Fill)
               }
               1 => {
                  let lb_enery_usage = Text::new("Energy Usage");
                  let bar_chart_1 = Svg::from_path(format!("{}/assets/images/bar-chart.svg", ROOT_PATH())).height(Length::Fill);
                  let energy_usage_sec = Container::new(Column::new().spacing(10).push(lb_enery_usage).push(bar_chart_1)).height(Length::FillPortion(5));

                  let lb_screen_on = Text::new("Screen On Usage");
                  let bar_chart_2 = Svg::from_path(format!("{}/assets/images/bar-chart.svg", ROOT_PATH())).height(Length::Fill);
                  let screen_on_sec = Container::new(Column::new().spacing(10).push(lb_screen_on).push(bar_chart_2)).height(Length::FillPortion(5));
                  Container::new(Column::new().spacing(20).push(energy_usage_sec).push(screen_on_sec)).height(Length::Fill)
               }
               _ => Container::new(Space::with_width(Length::Fill)),
            };

            Container::new(
               Column::new()
                  .width(Length::Fill)
                  .push(tabbar_section)
                  .push(inner_tabview.padding(15).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundWhite)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
         }
         1 => {
            let BatteryTab {
               turn_display_off_after_state,
               turn_display_off_after_val,
               brightness_control,
               brigthness_value,
               slightly_dim_display,
               enable_power_nap,
               opt_video_stream,
               opt_battery_charging,
               battery_health_state,
               is_battery_health_clicked,
               restore_defaults,
               ..
            } = battery_tab;

            let chb_show_battery = Checkbox::new(*show_battery_status, "Show battery status in menu bar", BatteryMessage::ShowBatteryStatusToggled).spacing(10).style(CustomCheckbox::Default);
            let lb_turn_display_off = Text::new("Turn display off after: ").size(12);
            let lb_brightness = Text::new("Dispaly Brightness").size(12);
            let slider_turn_display_off = Slider::new(turn_display_off_after_state, 1..=181, *turn_display_off_after_val, BatteryMessage::TurnDisplayOffBatteryChanged)
               .width(Length::FillPortion(4))
               .style(CustomSlider::Default);
            *brigthness_value = self.brightnessctl.get_percent() as u8;
            let slider_brightness = Slider::new(brightness_control, 1..=100, *brigthness_value, BatteryMessage::BrightNessChanged).width(Length::FillPortion(4)).style(CustomSlider::Default);
            let turn_display_off_sec = Column::new()
               .push(Row::new().push(lb_brightness).push(Space::with_width(Length::Fill)).push(Text::new(format!("{}%", self.brightnessctl.get_percent().to_string()))))
               .push(slider_brightness)
               .spacing(4)
               .push(lb_turn_display_off)
               .push(
                  Column::new().push(slider_turn_display_off).push(
                     Row::new()
                        .width(Length::Fill)
                        .push(Text::new("1 min").size(12))
                        .push(Space::with_width(Length::FillPortion(4)))
                        .push(Text::new("15 min").size(12))
                        .push(Space::with_width(Length::FillPortion(5)))
                        .push(Text::new("1 hr").size(12))
                        .push(Space::with_width(Length::FillPortion(5)))
                        .push(Text::new("3 hrs").size(12))
                        .push(Space::with_width(Length::FillPortion(1)))
                        .push(Text::new("never").size(12)),
                  ),
               );

            let chb_slightly_dim = Checkbox::new(*slightly_dim_display, "Slightly dim the display while on battery power", BatteryMessage::SlightlyDimDisplayToggled)
               .spacing(10)
               .style(CustomCheckbox::Default);
            let chb_power_nap = Checkbox::new(*enable_power_nap, "Enable Power Nap while on battery power", BatteryMessage::PowerNapWhileBatteryToggled)
               .spacing(10)
               .style(CustomCheckbox::Default);
            let txt_hint = Text::new("While sleeping, your computer can periodically check for new email, calendar and more.").size(12);
            let chb_opt_video_stream = Checkbox::new(*opt_video_stream, "Optimize video streaming while on battery", BatteryMessage::OptVideoStreamToggled)
               .spacing(10)
               .style(CustomCheckbox::Default);
            let chb_opt_battery_charging = Checkbox::new(*opt_battery_charging, "Optimize video streaming while on battery", BatteryMessage::OptBatteryChargedToggled)
               .spacing(10)
               .style(CustomCheckbox::Default);
            let txt_charging_hint = Text::new("To reduce battery aging, your computer learns from your daily charging routine, so it can wait to finish charging past 80% until you need to use it on battery. ").size(12);

            let top_sec = Container::new(
               Column::new()
                  .spacing(20)
                  .push(chb_show_battery)
                  .push(turn_display_off_sec)
                  .push(
                     Column::new()
                        .spacing(10)
                        .push(chb_slightly_dim)
                        .push(Column::new().push(chb_power_nap).push(Row::new().push(Space::with_width(Length::Units(30))).push(txt_hint))),
                  )
                  .push(
                     Column::new()
                        .spacing(10)
                        .push(chb_opt_video_stream)
                        .push(Column::new().push(chb_opt_battery_charging).push(Row::new().push(Space::with_width(Length::Units(30))).push(txt_charging_hint))),
                  ),
            )
            .height(Length::Fill);

            // ផ្នែកខាងក្រោម
            let btn_battery_health = Button::new(battery_health_state, Text::new("  Battery Health...  ")).on_press(BatteryMessage::BatteryHealthClicked).style(CustomButton::Default);
            let btn_restore = Button::new(restore_defaults, Text::new("  Defaults  ")).on_press(BatteryMessage::RestoreDefaultBatteryClicked).style(CustomButton::Default);
            let bottom_row = Row::new()
               .spacing(15)
               .align_items(Align::Center)
               .push(btn_restore)
               .push(Space::with_width(Length::Fill))
               .push(Text::new(if *is_battery_health_clicked { "Battery Condition: Normal" } else { "" }))
               .push(btn_battery_health);
            let bottom_sec = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);

            Container::new(Column::new().push(top_sec).push(bottom_sec))
         }
         2 => {
            let PowerAdapter {
               turn_display_off_after_state,
               turn_display_off_after_val,
               prevent_from_sleep,
               wake_network_access,
               enable_power_nap,
               restore_defaults,
            } = power_adapter;

            let chb_show_battery = Checkbox::new(*show_battery_status, "Show battery status in menu bar", BatteryMessage::ShowBatteryStatusToggled).spacing(10).style(CustomCheckbox::Default);
            let lb_turn_display_off = Text::new("Turn display off after: ").size(12);
            let slider_turn_display_off = Slider::new(turn_display_off_after_state, 1..=181, *turn_display_off_after_val, BatteryMessage::TurnDisplayOffPowerChanged)
               .width(Length::FillPortion(4))
               .style(CustomSlider::Default);
            let turn_display_off_sec = Column::new().spacing(4).push(lb_turn_display_off).push(
               Column::new().push(slider_turn_display_off).push(
                  Row::new()
                     .width(Length::Fill)
                     .push(Text::new("1 min").size(12))
                     .push(Space::with_width(Length::FillPortion(4)))
                     .push(Text::new("15 min").size(12))
                     .push(Space::with_width(Length::FillPortion(5)))
                     .push(Text::new("1 hr").size(12))
                     .push(Space::with_width(Length::FillPortion(5)))
                     .push(Text::new("3 hrs").size(12))
                     .push(Space::with_width(Length::FillPortion(1)))
                     .push(Text::new("never").size(12)),
               ),
            );

            let chb_prevent_from_sleep = Checkbox::new(*prevent_from_sleep, "Prevent computer from sleeping automatically when the display is off", BatteryMessage::PreventFromSleepToggled)
               .spacing(10)
               .style(CustomCheckbox::Default);
            let chb_wake_network_access = Checkbox::new(*wake_network_access, "Wake for network access", BatteryMessage::WakeNetworkAccessToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_power_nap = Checkbox::new(*enable_power_nap, "Enable Power Nap while plugged into a power adapter", BatteryMessage::PowerNapWhilePowerToggled)
               .spacing(10)
               .style(CustomCheckbox::Default);
            let txt_hint = Text::new("While sleeping, your computer can backup using Time Machine and periodically check for email, calendar and more.").size(12);

            let top_sec = Container::new(
               Column::new().spacing(20).push(chb_show_battery).push(turn_display_off_sec).push(
                  Column::new()
                     .spacing(10)
                     .push(chb_prevent_from_sleep)
                     .push(chb_wake_network_access)
                     .push(Column::new().push(chb_power_nap).push(Row::new().push(Space::with_width(Length::Units(30))).push(txt_hint))),
               ),
            )
            .height(Length::Fill);
            // ផ្នែកខាងក្រោម
            let btn_restore = Button::new(restore_defaults, Text::new("  Defaults  ")).on_press(BatteryMessage::RestoreDefaultPowerClicked).style(CustomButton::Default);
            let bottom_row = Row::new().spacing(15).align_items(Align::Center).push(btn_restore);
            let bottom_sec = Container::new(bottom_row).width(Length::Fill);

            Container::new(Column::new().push(top_sec).push(bottom_sec))
         }
         3 => {
            let Schedule {
               startup,
               startup_repeat_state,
               startup_repeat_val,
               startup_time_state,
               sleep,
               sleep_repeat_state,
               sleep_repeat_val,
               sleep_time_state,
               restore_prev_state,
               appply_state,
               is_changed,
            } = schedule;

            let chb_startup = Checkbox::new(*startup, "Start up", BatteryMessage::StartUpToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_startup_repeat = PickList::new(startup_repeat_state, &RepeatDays::ALL[..], Some(*startup_repeat_val), BatteryMessage::StartUpRepeatChanged).style(CustomSelect::Primary);
            let sp_startup_hour = NumberInput::new(&mut startup_time_state.hour_state, startup_time_state.hour_val, 24, BatteryMessage::StartUpHourChanged).width(Length::Units(50));
            let sp_startup_minute = NumberInput::new(&mut startup_time_state.minute_state, startup_time_state.minute_val, 60, BatteryMessage::StartUpMinuteChanged).width(Length::Units(50));
            let txt_startup_hint = Text::new("Scheduled start up will only occur when a power adapter is connected to your computer.");
            let startup_hint_con = Container::new(if *startup { txt_startup_hint } else { Text::new("") }).height(Length::Units(35));
            let startup_con = Container::new(
               Row::new().push(Space::with_width(Length::Units(30))).push(
                  Column::new()
                     .spacing(7)
                     .push(
                        Row::new()
                           .spacing(7)
                           .align_items(Align::Center)
                           .push(pl_startup_repeat)
                           .push(Text::new("at").size(12))
                           .push(sp_startup_hour)
                           .push(Text::new(":").size(12))
                           .push(sp_startup_minute),
                     )
                     .push(startup_hint_con),
               ),
            );

            let chb_sleep = Checkbox::new(*sleep, "Sleep", BatteryMessage::SleepToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_sleep_repeat = PickList::new(sleep_repeat_state, &RepeatDays::ALL[..], Some(*sleep_repeat_val), BatteryMessage::SleepRepeatChanged).style(CustomSelect::Primary);
            let sp_sleep_hour = NumberInput::new(&mut sleep_time_state.hour_state, sleep_time_state.hour_val, 24, BatteryMessage::SleepHourChanged).width(Length::Units(50));
            let sp_sleep_minute = NumberInput::new(&mut sleep_time_state.minute_state, sleep_time_state.minute_val, 60, BatteryMessage::SleepMinuteChanged).width(Length::Units(50));
            let sleep_con = Container::new(
               Row::new()
                  .spacing(7)
                  .align_items(Align::Center)
                  .push(Space::with_width(Length::Units(23)))
                  .push(pl_sleep_repeat)
                  .push(Text::new("at").size(12))
                  .push(sp_sleep_hour)
                  .push(Text::new(":").size(12))
                  .push(sp_sleep_minute),
            );

            let top_sec = Container::new(Column::new().push(chb_startup).push(startup_con).push(chb_sleep).push(sleep_con)).height(Length::Fill);
            // ផ្នែកខាងក្រោម
            let mut btn_restore = Button::new(restore_prev_state, Text::new("  Restore Previous Settings  ")).style(CustomButton::Default);
            let mut btn_apply = Button::new(appply_state, Text::new("  Apply  ")).style(CustomButton::Primary);
            if *is_changed {
               btn_restore = btn_restore.on_press(BatteryMessage::RestorePrevClicked);
               btn_apply = btn_apply.on_press(BatteryMessage::ApplyScheduleClicked);
            }
            let bottom_row = Row::new().spacing(15).align_items(Align::Center).push(btn_restore).push(btn_apply);
            let bottom_sec = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);

            Container::new(Column::new().push(top_sec).push(bottom_sec))
         }
         _ => Container::new(Space::with_height(Length::Fill)),
      };

      // មាតិកា
      let content = Row::new()
         .width(Length::Fill)
         .push(sidebar)
         .push(Container::new(tabview.padding(27).style(CustomContainer::ForegroundGray)).width(Length::FillPortion(6)).height(Length::Fill).padding(20));

      Container::new(content).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone, Default)]
pub struct Usage {
   tabbar_state: Vec<(String, button::State)>,
   current_tab_idx: usize,
}

impl Usage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![("  Last 24 Hours  ".to_string(), button::State::new()), ("  Last 7 Days  ".to_string(), button::State::new())],
         current_tab_idx: 0,
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct BatteryTab {
   turn_display_off_after_state: slider::State,
   turn_display_off_after_val: u8,
   brightness_control: slider::State,
   brigthness_value: u8,
   slightly_dim_display: bool,
   enable_power_nap: bool,
   opt_video_stream: bool,
   opt_battery_charging: bool,
   battery_health_state: button::State,
   is_battery_health_clicked: bool,
   restore_defaults: button::State,
}

impl BatteryTab {
   pub fn new() -> Self {
      Self {
         turn_display_off_after_val: 2,
         slightly_dim_display: true,
         enable_power_nap: true,
         opt_video_stream: false,
         opt_battery_charging: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct PowerAdapter {
   turn_display_off_after_state: slider::State,
   turn_display_off_after_val: u8,
   prevent_from_sleep: bool,
   wake_network_access: bool,
   enable_power_nap: bool,
   restore_defaults: button::State,
}

impl PowerAdapter {
   pub fn new() -> Self {
      Self {
         turn_display_off_after_val: 10,
         prevent_from_sleep: false,
         wake_network_access: true,
         enable_power_nap: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct Schedule {
   startup: bool,
   startup_repeat_state: pick_list::State<RepeatDays>,
   startup_repeat_val: RepeatDays,
   startup_time_state: TimeState,
   sleep: bool,
   sleep_repeat_state: pick_list::State<RepeatDays>,
   sleep_repeat_val: RepeatDays,
   sleep_time_state: TimeState,
   restore_prev_state: button::State,
   appply_state: button::State,
   is_changed: bool,
}

#[derive(Debug, Clone, Default)]
struct TimeState {
   hour_state: number_input::State,
   hour_val: u8,
   minute_state: number_input::State,
   minute_val: u8,
}

impl Schedule {
   pub fn new() -> Self {
      Self {
         startup_time_state: TimeState { hour_val: 12, ..Default::default() },
         sleep_time_state: TimeState { hour_val: 12, ..Default::default() },
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum RepeatDays {
   Weekdays,
   Weekends,
   #[default]
   EveryDay,
   Mon,
   Tue,
   Wed,
   Thu,
   Fri,
   Sat,
   Sun,
}

impl RepeatDays {
   const ALL: [RepeatDays; 10] = [
      RepeatDays::Weekdays,
      RepeatDays::Weekends,
      RepeatDays::EveryDay,
      RepeatDays::Mon,
      RepeatDays::Tue,
      RepeatDays::Wed,
      RepeatDays::Thu,
      RepeatDays::Fri,
      RepeatDays::Sat,
      RepeatDays::Sun,
   ];
}

impl std::fmt::Display for RepeatDays {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            RepeatDays::Weekdays => "Weekdays",
            RepeatDays::Weekends => "Weekends",
            RepeatDays::EveryDay => "Every Day",
            RepeatDays::Mon => "Monday",
            RepeatDays::Tue => "Tuesday",
            RepeatDays::Wed => "Wednesday",
            RepeatDays::Thu => "Thursday",
            RepeatDays::Fri => "Friday",
            RepeatDays::Sat => "Saturday",
            RepeatDays::Sun => "Sunday",
         }
      )
   }
}
