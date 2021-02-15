use iced::{
   pick_list, button, scrollable, Element, Align, Space, Length, 
   Container, Checkbox, Row, Text, Button, Column, Scrollable, PickList,
};
use iced_custom_widget::{Icon, IconBrand, number_input, NumberInput};
use super::super::styles::{CustomButton, CustomContainer, CustomCheckbox, CustomSelect};
use smart_default::SmartDefault;

#[derive(Debug, Clone)]
pub enum PrivacyMessage {
   TabChanged(usize),
   ChangePW(bool),
   ReqPWToggled(bool),
   ReqPWDurChanged(ReqPWDuration),
   ShowMsgScreenLockToggled(bool),
   LogoutAfterToggled(bool),
   LogoutAfterDurChanged(u8),
   ReqAdminPWToggled(bool),
   PrivacyTabSelected(usize),
   EnablePrivacyToggled(bool),
   AppsPrivacyToggled(usize, bool),
   TurnFirewallClicked,
   BlockAllInConToggled(bool),
   AutoAllowBuiltInToggled(bool),
   AutoAllowDownSignedToggled(bool),
   AdvancedToggled(bool),
}

#[derive(Debug, Clone)]
pub struct PrivacyPage {
   tabbar_state: Vec<(&'static str, button::State)>,
   current_tab_idx: usize,
   general_tab: GeneralTab,
   privacy_tab: PrivacyTab,
   firewall_tab: FirewallTab,
   advanced_state: button::State,
   is_advanced: bool
}

impl PrivacyPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  General  ", button::State::new()),
            ("  Privacy  ", button::State::new()),
            ("  Firewall  ", button::State::new()),
         ],
         current_tab_idx: 0,
         general_tab: GeneralTab::new(),
         privacy_tab: PrivacyTab::new(),
         firewall_tab: FirewallTab::new(),
         advanced_state: button::State::new(),
         is_advanced: false
      }
   }

   pub fn update(&mut self, msg: PrivacyMessage) {
      use PrivacyMessage::*;
      match msg {
         TabChanged(idx) => self.current_tab_idx = idx,
         ChangePW(is_checked) => self.general_tab.is_change_pw = is_checked,
         ReqPWToggled(is_checked) => self.general_tab.req_pw = is_checked,
         ReqPWDurChanged(val) => self.general_tab.req_pw_dur_val = val,
         ShowMsgScreenLockToggled(is_checked) => self.general_tab.show_msg_screen_lock = is_checked,
         LogoutAfterToggled(is_checked) => self.general_tab.logout_after = is_checked,
         LogoutAfterDurChanged(val) => self.general_tab.logout_after_dur_state.dur_val = val,
         ReqAdminPWToggled(is_checked) => self.general_tab.req_admin_pw_sys_pref = is_checked,
         PrivacyTabSelected(idx) => self.privacy_tab.selected_privacy_tab = idx,
         EnablePrivacyToggled(is_checked) => {
            let (_, enable, _) = self.privacy_tab.privacy_tab_map.get_mut(self.privacy_tab.selected_privacy_tab).unwrap();
            if let Some(enable) = enable {
               enable.0 = is_checked;
            }
         },
         AppsPrivacyToggled(idx, is_checked) => {
            self.privacy_tab.selected_app_privacy = idx;
            self.privacy_tab.privacy_tab_map.get_mut(self.privacy_tab.selected_privacy_tab).unwrap().2.get_mut(idx).unwrap().0 = is_checked;
         },
         TurnFirewallClicked => self.firewall_tab.is_turn_on_firewall = !self.firewall_tab.is_turn_on_firewall,
         BlockAllInConToggled(is_checked) => self.firewall_tab.block_all_in_con = is_checked,
         AutoAllowBuiltInToggled(is_checked) => self.firewall_tab.auto_allow_built_in_software = is_checked,
         AutoAllowDownSignedToggled(is_checked) => self.firewall_tab.auto_allow_down_signed_software = is_checked,
         AdvancedToggled(is_checked) => self.is_advanced = is_checked,
      }
   }

   pub fn view(&mut self) -> Element<PrivacyMessage> {
      let PrivacyPage {
         tabbar_state,
         current_tab_idx,
         general_tab,
         privacy_tab,
         firewall_tab,
         advanced_state,
         is_advanced,
      } = self;

      // របារផ្ទាំង
      let tabbar = tabbar_state.iter_mut().enumerate().fold(Row::new().spacing(2).align_items(Align::Center), |tabbar, (idx, (title, state))| {
         let btn = Button::new(state, Text::new(*title)).padding(5).on_press(PrivacyMessage::TabChanged(idx)).style(if *current_tab_idx == idx {CustomButton::SelectedTab} else {CustomButton::Tab});
         tabbar.push(btn)
      }); 
      let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
      let tabbar_section = Container::new(tabbar_con).padding(7).width(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let GeneralTab {
               change_pw_state,
               is_change_pw,
               req_pw,
               req_pw_dur_state,
               req_pw_dur_val,
               show_msg_screen_lock,
               set_lock_msg_state,
               logout_after,
               logout_after_dur_state: DurationState {
                  dur_state,
                  dur_val, 
               },
               req_admin_pw_sys_pref,
            } = general_tab;

            let lb_login_pw = Text::new("A login password has been set for this user");
            let btn_change_pw = Button::new(change_pw_state, Text::new("  Change Password  ")).on_press(PrivacyMessage::ChangePW(!(*is_change_pw))).style(CustomButton::Default);
            let change_pw_sec = Container::new(
               Row::new().spacing(10)
               .push(lb_login_pw)
               .push(btn_change_pw)
            );
            let chb_req_pw = Checkbox::new(*req_pw, "Require password", PrivacyMessage::ReqPWToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_req_pw_dur = PickList::new(req_pw_dur_state, &ReqPWDuration::ALL[..], Some(*req_pw_dur_val), PrivacyMessage::ReqPWDurChanged).style(CustomSelect::Primary);
            let lb_after_sleep = Text::new("after sleep or screen saver begin.");
            let req_pw_sec = Container::new(
               Row::new().spacing(10).align_items(Align::Center)
               .push(chb_req_pw)
               .push(pl_req_pw_dur)
               .push(lb_after_sleep)
            );
            let chb_show_msg_screen_lock = Checkbox::new(*show_msg_screen_lock, "Show a message when the screen is locked", PrivacyMessage::ShowMsgScreenLockToggled).spacing(10).style(CustomCheckbox::Default);
            let btn_set_lock_msg = Button::new(set_lock_msg_state, Text::new("  Set Lock Message  ")).style(CustomButton::Default);
            let show_msg_sec = Container::new(
               Row::new().spacing(10).align_items(Align::Center)
               .push(chb_show_msg_screen_lock)
               .push(btn_set_lock_msg)
            );
            let chb_logout_after = Checkbox::new(*logout_after, "Log out after", PrivacyMessage::LogoutAfterToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_logout_after_dur = NumberInput::new(dur_state, *dur_val, 100, PrivacyMessage::LogoutAfterDurChanged).width(Length::Units(50));
            let lb_min_inactivity = Text::new("minutes of inactivity");
            let logout_after_dur_sec = Container::new(
               Row::new().spacing(10).align_items(Align::Center)
               .push(chb_logout_after)
               .push(pl_logout_after_dur)
               .push(lb_min_inactivity)
            );
            let chb_req_admin = Checkbox::new(*req_admin_pw_sys_pref, "Require an administrator password to access system preferences", PrivacyMessage::ReqAdminPWToggled).spacing(10).style(CustomCheckbox::Default);
            
            Container::new(
               Column::new().width(Length::Fill).spacing(10).align_items(Align::Start)
               .push(change_pw_sec)
               .push(req_pw_sec)
               .push(show_msg_sec)
               .push(logout_after_dur_sec)
               .push(chb_req_admin)
            ).width(Length::Fill).height(Length::Fill)
         },
         1 => {
            let PrivacyTab {
               privacy_tabs,
               selected_privacy_tab,
               privacy_tab_map,
               selected_app_privacy,
               left_pane_scroll,
               right_pane_scroll,
            } = privacy_tab;

            // ផ្ទាំងខាងឆ្វេង
            let left_tab_col = privacy_tabs.iter_mut().enumerate().fold(Scrollable::new(left_pane_scroll).height(Length::Fill).padding(7).spacing(4), |scrollable, (idx, (icon, title, state))| {
               let btn = Button::new(state, Row::new().spacing(7).align_items(Align::Center).push(Icon::new(*icon).size(30)).push(Text::new(*title))).width(Length::Fill).on_press(PrivacyMessage::PrivacyTabSelected(idx)).style(if *selected_privacy_tab == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar});
               scrollable.push(btn)
            });
            
            let left_pane = Container::new(left_tab_col).width(Length::FillPortion(4)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្ទាំងខាងស្ដាំ
            let (hint, enable, ls_apps) = privacy_tab_map.get_mut(*selected_privacy_tab).unwrap();
            let mut main_view_col = Column::new().spacing(10);
            if let Some((is_checked, title)) = enable {
               main_view_col = main_view_col.push(Checkbox::new(*is_checked, *title, PrivacyMessage::EnablePrivacyToggled));
            }
            main_view_col = main_view_col.push(Text::new(*hint));
            let apps_list_view = ls_apps.iter_mut().enumerate().fold(Scrollable::new(right_pane_scroll).width(Length::Fill).height(Length::Fill).padding(7).spacing(4), |col, (idx, (is_checked, icon, title))| {
               let row = Row::new().width(Length::Fill).align_items(Align::Center).padding(4)
                  .push(Checkbox::new(*is_checked, *title, move |is| PrivacyMessage::AppsPrivacyToggled(idx, is)).spacing(10).style(CustomCheckbox::Default))
                  // .push(Space::with_width(Length::Fill))
                  .push(IconBrand::new(*icon).size(30));

               col.push(Container::new(row).width(Length::Fill).style(if *selected_app_privacy == idx {CustomContainer::Hovered} else {CustomContainer::ForegroundWhite}))
            });
            let app_view_sec = Container::new(apps_list_view).height(Length::Fill).style(CustomContainer::ForegroundWhite);
            main_view_col = main_view_col.push(app_view_sec);
            let main_view_sec = Container::new(main_view_col).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundGray);

            Container::new(
               Row::new().spacing(10)
               .push(left_pane)
               .push(main_view_sec)
            ).width(Length::Fill).height(Length::Fill)
         },
         2 => {
            let FirewallTab {
               turn_firewall_state,
               is_turn_on_firewall,
               block_all_in_con,
               auto_allow_built_in_software,
               auto_allow_down_signed_software,
            } = firewall_tab;

            let firewall_state = |is_on| if is_on {"On"} else {"Off"};
            let firewall_status = Container::new(Space::with_width(Length::Units(10))).height(Length::Units(10)).style(if *is_turn_on_firewall {CustomContainer::Success} else {CustomContainer::Warning});
            let txt_firewall = Text::new(format!("Firewall: {}", firewall_state(*is_turn_on_firewall))).size(14);
            let btn_turn_firewall = Button::new(turn_firewall_state, Text::new(format!("  Turn {} Firewall  ", firewall_state(!(*is_turn_on_firewall))))).on_press(PrivacyMessage::TurnFirewallClicked).style(CustomButton::Default);
            let txt_firewall_hint = Text::new("The firewall is turned on and set up to prevent unauthorized applications, programs and services from accepting incoming connections.").size(12);
            let chb_block_all_in_con = Checkbox::new(*block_all_in_con, "Block all incoming connections", PrivacyMessage::BlockAllInConToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_block_all_hint = Text::new("Blocks all incoming connections except those required for basic Internet Services, such as DHCP.").size(12);
            let chb_auto_allow_built_in_software = Checkbox::new(*auto_allow_built_in_software, "Automatically allow built-in software to recieve incoming connections", PrivacyMessage::AutoAllowBuiltInToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_auto_allow_down_signed_software = Checkbox::new(*auto_allow_down_signed_software, "Automatically allow downloaded signed software to recieve incoming connections", PrivacyMessage::AutoAllowDownSignedToggled).spacing(10).style(CustomCheckbox::Default);

            Container::new(
               Column::new().spacing(10)
               .push(
                  Row::new().spacing(10).align_items(Align::Center)
                  .push(firewall_status)
                  .push(txt_firewall)
                  .push(Space::with_width(Length::Fill))
                  .push(btn_turn_firewall)
               )
               .push(txt_firewall_hint)
               .push(
                  Column::new()
                  .push(chb_block_all_in_con)
                  .push(
                     Row::new().push(Space::with_width(Length::Units(30))).push(txt_block_all_hint)
                  )
               )
               .push(chb_auto_allow_built_in_software)
               .push(chb_auto_allow_down_signed_software)
            ).width(Length::Fill).height(Length::Fill)
         }, 
         _ => Container::new(Space::with_height(Length::Fill))
      };

      // ផ្នែកខាងក្រោម
      let bottom_row = Row::new().padding(15).spacing(20).align_items(Align::Center)
         .push(Text::new(if *is_advanced {"Sorry, It's just UI"} else {""}))
         .push(Button::new(advanced_state, Text::new("  Advanced  ")).style(CustomButton::Default).on_press(PrivacyMessage::AdvancedToggled(!self.is_advanced)));
      let bottom_section = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);

      // មាតិកា   
      let content = Column::new().width(Length::Fill).align_items(Align::Center)
         .push(tabbar_section)
         .push(tabview.height(Length::Fill).padding(20).style(CustomContainer::ForegroundGray))
         .push(bottom_section);

      Container::new(content).width(Length::FillPortion(15)).padding(20).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone, Default)]
pub(self) struct GeneralTab {
   change_pw_state: button::State,
   is_change_pw: bool,
   req_pw: bool,
   req_pw_dur_state: pick_list::State<ReqPWDuration>,
   req_pw_dur_val: ReqPWDuration,
   show_msg_screen_lock: bool,
   set_lock_msg_state: button::State,
   logout_after: bool,
   logout_after_dur_state: DurationState,
   req_admin_pw_sys_pref: bool,
}

impl GeneralTab {
   pub fn new() -> Self {
      Self {
         req_pw: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub(self) struct PrivacyTab {
   privacy_tabs: Vec<(char, &'static str, button::State)>,
   selected_privacy_tab: usize,
   privacy_tab_map: Vec<(&'static str, Option<(bool, &'static str)>, Vec<(bool, char, &'static str)>)>,
   selected_app_privacy: usize,
   left_pane_scroll: scrollable::State,
   right_pane_scroll: scrollable::State,
}

impl PrivacyTab {
   pub fn new() -> Self {
      Self {
         privacy_tabs: vec![
            ('\u{f124}', "Location & Services", button::State::new()), 
            ('\u{f2b9}', "Contacts", button::State::new()),
            ('\u{f133}', "Calendars", button::State::new()), 
            ('\u{f87c}', "Photos", button::State::new()), 
            ('\u{f030}', "Camera", button::State::new()), 
            ('\u{f130}', "Microphone", button::State::new()), 
            ('\u{f29a}', "Accessibility", button::State::new()), 
            ('\u{f111}', "Bluetooth", button::State::new()), 
         ],
         privacy_tab_map: vec![
            (
               "Allow the apps and services below to determine your location.",
               Some((true, "Enable Location Services")),
               vec![
                  (true, '\u{f289}', "Weather"),
                  (false, '\u{f268}', "Google Chrome"),
                  (true, '\u{f392}', "Discord"),
               ]
            ),
            (
               "Allow the apps below to access your contacts.",
               None,
               vec![
                  (true, '\u{f41a}', "Terminal"),
               ]
            ),
            (
               "Allow the apps below to access your calendars.",
               None,
               vec![
                  (true, '\u{f41a}', "Terminal"),
               ]
            ),
            (
               "Allow the apps below to access your photos.",
               None,
               vec![
                  (true, '\u{f41a}', "Terminal"),
               ]
            ),
            (
               "Allow the apps below to access your camera.",
               None,
               vec![
                  (true, '\u{f268}', "Google Chrome"),
               ]
            ),
            (
               "Allow the apps below to access your microphone.",
               None,
               vec![
                  (true, '\u{f268}', "Google Chrome"),
                  (true, '\u{f2c6}', "Telegram"),
               ]
            ),
            (
               "Allow the apps below to control your computer.",
               None,
               Vec::new()
            ),
            (
               "Allow the apps below to use Bluetooth.",
               None,
               Vec::new()
            ),
         ],
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub(self) struct FirewallTab {
   turn_firewall_state: button::State,
   is_turn_on_firewall: bool,
   block_all_in_con: bool,
   auto_allow_built_in_software: bool,
   auto_allow_down_signed_software: bool,
}

impl FirewallTab {
   pub fn new() -> Self {
      Self {
         is_turn_on_firewall: true,
         auto_allow_built_in_software: true,
         auto_allow_down_signed_software: true,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum ReqPWDuration {
   #[default]
   Immediately,
   _5s,
   _1m,
   _5m,
   _15m,
   _1h,
}

impl ReqPWDuration {
   const ALL: [ReqPWDuration; 6] = [
      ReqPWDuration::Immediately,
      ReqPWDuration::_5s,
      ReqPWDuration::_1m,
      ReqPWDuration::_5m,
      ReqPWDuration::_15m,
      ReqPWDuration::_1h
   ];
}

impl std::fmt::Display for ReqPWDuration {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      use ReqPWDuration::*;
      write!(f, "{}", match self {
            Immediately => "Immediately",
            _5s => "5 secs",
            _1m => "1 min",
            _5m => "5 mins", 
            _15m => "15 mins",
            _1h => "1 hour"
         }
      )
   }
}

#[derive(Debug, Clone, SmartDefault)]
pub struct DurationState {
   dur_state: number_input::State,
   #[default(60)]
   dur_val: u8,
}