use super::{
   users_tab::{UsersTab, UsersMsg}, groups_tab::{GroupsTab, GroupsMsg}
};
use crate::gui::addon_widgets::tabbar;
use crate::gui::styles::{CustomContainer, CustomButton};
use iced::{
   button, Button, Text, Container, Length, Column, Row, Align, Element, Space
};

pub struct UserGroupPage {
   tabbar_state: Vec<(&'static str, button::State)>,
   curr_tab_idx: usize,
   users_tab: UsersTab,
   groups_tab: GroupsTab,
   btn_option_state: button::State   
}

#[derive(Debug, Clone)]
pub enum UserGroupMsg {
   TabChanged(usize),
   UsersMSG(UsersMsg),
   GroupsMSG(GroupsMsg),
   OptionClicked,
}

impl UserGroupPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("Users", button::State::new()),
            ("Groups", button::State::new()),
         ],
         curr_tab_idx: 0,
         users_tab: UsersTab::new(),
         groups_tab: GroupsTab::new(),
         btn_option_state: button::State::new(),
      }
   }

   pub fn update(&mut self, msg: UserGroupMsg) {
      use UserGroupMsg::*;
      let Self {
         users_tab,
         groups_tab,
         ..
      } = self;

      match msg {
         TabChanged(idx) => self.curr_tab_idx = idx,
         UsersMSG(users_msg) => users_tab.update(users_msg),
         GroupsMSG(groups_msg) => groups_tab.update(groups_msg),
         OptionClicked => {} 
      }
   }

   pub fn view(&mut self) -> Element<UserGroupMsg> {
      use UserGroupMsg::*;
      let Self {
         tabbar_state,
         users_tab,
         groups_tab,
         btn_option_state,   
         ..
      } = self;

      // របារផ្ទាំង
      let tabbar_sec = tabbar(tabbar_state, self.curr_tab_idx, |idx| TabChanged(idx));

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.curr_tab_idx {
         0 => users_tab.view().map(|msg| UsersMSG(msg)),
         1 => groups_tab.view().map(|msg| GroupsMSG(msg)),
         _ => Row::new().into()
      };

      // ផ្នែកខាងក្រោម
      let btn_opt = Button::new(btn_option_state, Text::new("Options")).on_press(OptionClicked).style(CustomButton::Default);

      let bottom_sec = Container::new(
         Row::new().padding(15).spacing(10).align_items(Align::Center)
         .push(Space::with_width(Length::Fill))
         .push(btn_opt)
      ).width(Length::Fill).align_x(Align::End);

      // មាតិកា
      let content = Column::new().width(Length::Fill)
         .push(tabbar_sec)
         .push(Container::new(tabview).width(Length::Fill).height(Length::Fill).padding(10).style(CustomContainer::ForegroundGray))
         .push(bottom_sec);

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}