use iced::{
   pick_list, slider, button, scrollable, Element, Align, Space, Length, Svg,
   Container, Checkbox, Row, Text, Button, Column, Scrollable, PickList, Slider, Radio,
};
use iced_custom_widget::Icon;
use vedas_core::svg;
use super::super::styles::{CustomButton, CustomContainer, CustomSlider, CustomCheckbox, CustomRadio};
use smart_default::SmartDefault;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub enum BatteryMessage {
   SidebarChanged(usize),
   InnerTabChanged(usize),
   ShowBatteryStatusToggled(bool),
   TurnDisplayOffChanged(u8),
   SlightlyDimDisplayToggled(bool),
   PowerNapToggled(bool),
   OptVideoStreamToggled(bool),
   OptBatteryChargedToggled(bool),
   BatteryHealthClicked,
   RestoreDefaultBatteryClicked,
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
         current_time: format!("Today, {:02}:{:02} {}", hour, now.minute(), if is_pm {"PM"} else {"AM"}),
         show_battery_status: true,
         usage: Usage::new(),
         battery_tab: BatteryTab::new(),
         power_adapter: PowerAdapter::new(),
         schedule: Schedule::new(),
      }
   }

   pub fn update(&mut self, msg: BatteryMessage) {
      match msg {
         BatteryMessage::SidebarChanged(idx) => self.current_sidebar_tab_idx = idx,
         BatteryMessage::InnerTabChanged(idx) => self.usage.current_tab_idx = idx,
         BatteryMessage::ShowBatteryStatusToggled(is_checked) => self.show_battery_status = is_checked,
         BatteryMessage::TurnDisplayOffChanged(val) => self.battery_tab.turn_display_off_after_val = val,
         BatteryMessage::SlightlyDimDisplayToggled(is_checked) => self.battery_tab.slightly_dim_display = is_checked,
         BatteryMessage::PowerNapToggled(is_checked) => self.battery_tab.enable_power_nap = is_checked,
         BatteryMessage::OptVideoStreamToggled(is_checked) => self.battery_tab.opt_video_stream = is_checked,
         BatteryMessage::OptBatteryChargedToggled(is_checked) => self.battery_tab.opt_battery_charging = is_checked,
         BatteryMessage::BatteryHealthClicked => self.battery_tab.is_battery_health_clicked = true,
         BatteryMessage::RestoreDefaultBatteryClicked => self.battery_tab = BatteryTab::new(),
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
      } = self;

      // របារចំហៀង
      let icon = Svg::from_path("assets/images/battery.svg").height(Length::Units(127));
      let icon_con = Container::new(icon).padding(27).width(Length::Fill).center_x();
      let battery_level = Text::new(format!("Current Level: {}%", current_battery));
      let last_charged = Text::new("Last charged to 100%").size(12);
      let last_charged_date = Text::new(current_time.as_str()).size(12);
      let sidebar_tabs = sidebar_state.iter_mut().enumerate().fold(Column::new().spacing(4), |col, (idx, (icon, name, state))| {
         col.push(
            Button::new(state, Row::new().spacing(4).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(name.as_str()))).width(Length::Fill).on_press(BatteryMessage::SidebarChanged(idx)).style(if *current_sidebar_tab_idx == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
         )
      });
      let sidebar_col = Column::new().spacing(20).align_items(Align::Center)
         .push(icon_con)
         .push(
            Column::new().spacing(7).align_items(Align::Center)
            .push(battery_level)
            .push(last_charged)
            .push(last_charged_date)
         )
         .push(sidebar_tabs);
      let sidebar = Container::new(sidebar_col).padding(7).width(Length::FillPortion(2)).height(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match current_sidebar_tab_idx {
         0 => {
            let Usage {
               tabbar_state,
               current_tab_idx,
            } = usage;

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
                  let line_chart_1 = Svg::from_path("assets/images/line-chart.svg").height(Length::Units(127));
                  let battery_level_sec = Container::new(
                     Column::new().spacing(5)
                     .push(lb_battery_level)
                     .push(line_chart_1)
                  );

                  let lb_screen_on = Text::new("Screen On Usage");
                  let line_chart_2 = Svg::from_path("assets/images/line-chart.svg").height(Length::Units(127));
                  let screen_on_sec = Container::new(
                     Column::new().spacing(5)
                     .push(lb_screen_on)
                     .push(line_chart_2)
                  );
                  
                  Container::new(
                     Column::new().spacing(20)
                     .push(battery_level_sec)
                     .push(screen_on_sec)
                  )
               }
               1 => {
                  let lb_enery_usage = Text::new("Energy Usage");
                  let bar_chart_1 = Svg::from_path("assets/images/bar-chart.svg").height(Length::Units(127));
                  let energy_usage_sec = Container::new(
                     Column::new().spacing(5)
                     .push(lb_enery_usage)
                     .push(bar_chart_1)
                  );

                  let lb_screen_on = Text::new("Screen On Usage");
                  let bar_chart_2 = Svg::from_path("assets/images/bar-chart.svg").height(Length::Units(127));
                  let screen_on_sec = Container::new(
                     Column::new().spacing(5)
                     .push(lb_screen_on)
                     .push(bar_chart_2)
                  );
                  
                  Container::new(
                     Column::new().spacing(20)
                     .push(energy_usage_sec)
                     .push(screen_on_sec)
                  )
               }
               _ => Container::new(Space::with_width(Length::Fill))
            };

            Container::new(
               Column::new().width(Length::Fill)
               .push(tabbar_section)
               .push(inner_tabview.padding(15).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundWhite))
            ).width(Length::Fill).height(Length::Fill)
         },
         1 => {
            let BatteryTab {
               turn_display_off_after_state,
               turn_display_off_after_val,
               slightly_dim_display,
               enable_power_nap,
               opt_video_stream,
               opt_battery_charging,
               battery_health_state,
               is_battery_health_clicked,
               restore_defaults,
            } = battery_tab;

            let chb_show_battery = Checkbox::new(*show_battery_status, "Show battery status in menu bar", BatteryMessage::ShowBatteryStatusToggled).spacing(10).style(CustomCheckbox::Default);
            let lb_turn_display_off = Text::new("Turn display off after: ").size(12);
            let slider_turn_display_off = Slider::new(turn_display_off_after_state, 1..=181, *turn_display_off_after_val, BatteryMessage::TurnDisplayOffChanged).style(CustomSlider::Default);
            let turn_display_off_sec = Column::new().spacing(4)
               .push(lb_turn_display_off)
               .push(slider_turn_display_off);
            
            let chb_slightly_dim = Checkbox::new(*slightly_dim_display, "Slightly dim the display while on battery power", BatteryMessage::SlightlyDimDisplayToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_power_nap = Checkbox::new(*enable_power_nap, "Enable Power Nap while on battery power", BatteryMessage::PowerNapToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_hint = Text::new("While sleeping, your computer can periodically check for new email, calendar and more.").size(12);
            let chb_opt_video_stream = Checkbox::new(*opt_video_stream, "Optimize video streaming while on battery", BatteryMessage::OptVideoStreamToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_opt_battery_charging = Checkbox::new(*opt_battery_charging, "Optimize video streaming while on battery", BatteryMessage::OptBatteryChargedToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_charging_hint = Text::new("To reduce battery aging, your computer learns from your daily charging routine, so it can wait to finish charging past 80% until you need to use it on battery. ").size(12);

            let top_sec = Container::new(
               Column::new().spacing(20)
               .push(chb_show_battery)
               .push(turn_display_off_sec)
               .push(
                  Column::new().spacing(10)
                  .push(chb_slightly_dim)
                  .push(chb_power_nap)
                  .push(txt_hint)
               )
               .push(
                  Column::new().spacing(10)
                  .push(chb_opt_video_stream)
                  .push(chb_opt_battery_charging)
                  .push(txt_charging_hint)
               )
            ).height(Length::Fill);
            
            // ផ្នែកខាងក្រោម
            let btn_battery_health = Button::new(battery_health_state, Text::new("  Battery Health...  ")).on_press(BatteryMessage::BatteryHealthClicked).style(CustomButton::Default);
            let btn_restore = Button::new(restore_defaults, Text::new("  Restore Defaults  ")).on_press(BatteryMessage::RestoreDefaultBatteryClicked).style(CustomButton::Default);
            let bottom_row = Row::new().spacing(15).align_items(Align::Center).push(Text::new(if *is_battery_health_clicked {"Battery Condition: Normal"} else {""})).push(btn_battery_health).push(btn_restore);
            let bottom_sec = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);
            
            Container::new(
               Column::new()
               .push(top_sec)
               .push(bottom_sec)
            )
         },
         // 2 => {
         //    let InputSources {
         //       btn_add_state, 
         //       btn_remove_state, 
         //       input_sources_tab,
         //       input_sources_selected,
         //       show_input_menu,
         //       auto_switch,
         //       left_pane_scroll,
         //       right_pane_scroll,
         //    } = input_sources;

         //    // ផ្ទាំងខាងឆ្វេង
         //    let tab_len = input_sources_tab.len();
         //    let left_tab_col = input_sources_tab.iter_mut().enumerate().fold(Scrollable::new(left_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (icon, title, state))| {
         //       col.push(
         //          if let Some(selected_idx) = input_sources_selected {
         //             Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(title.as_str()))).width(Length::Fill).on_press(BatteryMessage::InputSourceLeftTabSelected(idx)).style(if *selected_idx == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
         //          } else {
         //             Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Icon::new(*icon).size(18)).push(Text::new(title.as_str()))).width(Length::Fill).on_press(BatteryMessage::InputSourceLeftTabSelected(idx)).style(CustomButton::Sidebar)
         //          }
         //       )
         //    });
         //    let btn_add = Button::new(btn_add_state, Icon::new('\u{f0fe}').size(27)).padding(0).on_press(BatteryMessage::BtnAddClicked).style(CustomButton::Text);
         //    let mut btn_remove = Button::new(btn_remove_state, Icon::new('\u{f146}').size(27)).padding(0).style(CustomButton::Text);
         //    if input_sources_selected.is_some() && tab_len > 1 {
         //       btn_remove = btn_remove.on_press(BatteryMessage::BtnRemoveClicked);
         //    }
         //    let btn_group = Container::new(
         //       Row::new().push(btn_add).push(btn_remove)
         //    ).width(Length::Fill).style(CustomContainer::Header);
         //    let left_pane = Container::new(
         //       Column::new()
         //       .push(left_tab_col)
         //       .push(btn_group)
         //    ).width(Length::FillPortion(4)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

         //    // ផ្ទាំងខាងស្ដាំ
         //    let keyboard_image_con = match input_sources_selected {
         //       Some(idx) => match idx {
         //          0 => {
         //             let en_keyboard = svg!("assets/images/keyboard.svg").height(Length::Units(250));
         //             Container::new(
         //                Row::new().push(Space::with_width(Length::FillPortion(1))).push(en_keyboard).push(Space::with_width(Length::FillPortion(1)))
         //             ).width(Length::Fill).center_x().center_y()
         //          },
         //          1 => {
         //             let kh_keyboard = svg!("assets/images/keyboard.svg").height(Length::Units(250));
         //             Container::new(
         //                Row::new().push(Space::with_width(Length::FillPortion(1))).push(kh_keyboard).push(Space::with_width(Length::FillPortion(1)))
         //             ).width(Length::Fill).center_x().center_y()
         //          },
         //          _ => Container::new(Space::with_width(Length::Fill))
         //       }
         //       None => Container::new(Space::with_width(Length::Fill))
         //    };

         //    let right_pane = Container::new(
         //       Scrollable::new(right_pane_scroll).push(keyboard_image_con)
         //    ).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

         //    // ផ្នែកខាងក្រោម
         //    let chb_show_input_menu = Checkbox::new(*show_input_menu, "Show Input menu in menu bar", BatteryMessage::ShowInputMenuToggled).spacing(10).style(CustomCheckbox::Default);
         //    let chb_auto_switch = Checkbox::new(*auto_switch, "Automatically switch to a document's input source", BatteryMessage::AutoSwitchToggled).spacing(10).style(CustomCheckbox::Default);
         //    let bottom_right_col = Column::new().spacing(10)
         //       .push(chb_show_input_menu)
         //       .push(chb_auto_switch);

         //    let bottom_row = Row::new().spacing(15).width(Length::Fill)
         //       .push(Space::with_width(Length::FillPortion(4)))
         //       .push(Container::new(bottom_right_col).width(Length::FillPortion(6)));
            
         //    Container::new(
         //       Column::new().spacing(10)
         //       .push(
         //          Container::new(
         //             Row::new().spacing(15)
         //             .push(left_pane)
         //             .push(right_pane)
         //          ).height(Length::FillPortion(11))
         //       )
         //       .push(bottom_row)
         //    ).width(Length::Fill).height(Length::Fill)
         // }, 
         // 3 => {
         //    let Dictation {
         //       btn_about, 
         //       turn_on_dict,
         //       language_state,
         //       language_val,
         //       shortcut_state,
         //       shortcut_val,
         //    } = dictation;

         //    // ផ្ទាំងខាងឆ្វេង
         //    let mic_image = svg!("assets/images/mic.svg").height(Length::Units(150));
         //    let mic_con = Container::new(mic_image).width(Length::FillPortion(4)).center_x();

         //    // ផ្ទាំងខាងស្ដាំ
         //    let txt_dictation = Text::new("Use dictation wherever you can type text. To start dictating,\nuse the shortcut or select Start Dictation from the Edit menu.");
         //    let lb_dictation = Text::new("Dictation:");
         //    let rd_dictaion_on = Radio::new(true, "On", Some(*turn_on_dict), BatteryMessage::DictationToggled).size(15).spacing(10).style(if *turn_on_dict {CustomRadio::Active} else {CustomRadio::Disactive});
         //    let rd_dictaion_off = Radio::new(false, "Off", Some(*turn_on_dict), BatteryMessage::DictationToggled).size(15).spacing(10).style(if !(*turn_on_dict) {CustomRadio::Active} else {CustomRadio::Disactive});
         //    let dictation_section = Row::new().spacing(10).align_items(Align::Center)
         //       .push(lb_dictation)
         //       .push(rd_dictaion_on)
         //       .push(rd_dictaion_off);

         //    let lb_language = Text::new("Language:");
         //    let pl_language = PickList::new(language_state, &Language::ALL[..], Some(*language_val), BatteryMessage::LanguageChanged);
         //    let language_section = Row::new().spacing(10).align_items(Align::Center)
         //       .push(lb_language)
         //       .push(pl_language);

         //    let lb_shortcut = Text::new("Shortcut:");
         //    let pl_shortcut = PickList::new(shortcut_state, &ShortcutDict::ALL[..], Some(*shortcut_val), BatteryMessage::ShortcutChanged);
         //    let shortcut_section = Row::new().spacing(10).align_items(Align::Center)
         //       .push(lb_shortcut)
         //       .push(pl_shortcut);
            
         //    let right_con = Container::new(
         //       Column::new().spacing(20)
         //       .push(txt_dictation)
         //       .push(
         //          Column::new().spacing(10)
         //          .push(dictation_section)
         //          .push(language_section)
         //          .push(shortcut_section)
         //       )
         //    ).width(Length::FillPortion(6)).height(Length::Fill);
         
         //    Container::new(
         //       Column::new().spacing(10)
         //       .push(
         //          Container::new(
         //             Row::new().spacing(15)
         //             .push(mic_con)
         //             .push(right_con)
         //          ).height(Length::FillPortion(11))
         //       )
         //       .push(
         //          Container::new(
         //             Button::new(btn_about, Text::new("  About Dictation & Privacy  ")).on_press(BatteryMessage::AboutClicked).style(CustomButton::Default)
         //          ).width(Length::Fill).align_x(Align::End)
         //       )
         //    ).width(Length::Fill).height(Length::Fill)
         // },
         _ => Container::new(Space::with_height(Length::Fill))
      };

      // មាតិកា   
      let content = Row::new().width(Length::Fill)
         .push(sidebar)
         .push(
            Container::new(
               tabview.height(Length::Fill).padding(15).style(CustomContainer::ForegroundGray)
            ).width(Length::FillPortion(6)).height(Length::Fill).padding(20)
         );

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
         tabbar_state: vec![
            ("  Last 24 Hours  ".to_string(), button::State::new()),
            ("  Last 7 Days  ".to_string(), button::State::new()),
         ],
         current_tab_idx: 0
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct BatteryTab {
   turn_display_off_after_state: slider::State,
   turn_display_off_after_val: u8,
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
   is_apply_clicked: bool,
}

#[derive(Debug, Clone, Default)]
struct TimeState {
   inc_hour_state: button::State,
   hour_val: f32,
   dec_hour_state: button::State,
   inc_minute_state: button::State,
   minute_val: f32,
   dec_minute_state: button::State,
}

impl Schedule {
   pub fn new() -> Self {
      Self {
         startup_time_state: TimeState {
            hour_val: 12.0,
            ..Default::default()
         },
         sleep_time_state: TimeState {
            hour_val: 12.0,
            ..Default::default()
         },
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
   Sun
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
      RepeatDays::Sun
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
            RepeatDays::Sun => "Sunday"
         }
      )
   }
}