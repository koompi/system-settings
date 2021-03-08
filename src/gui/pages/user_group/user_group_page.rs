use super::{
   users_tab::{UsersTab, UsersMsg}, 
   groups_tab::{GroupsTab, GroupsMsg}
};
use libkoompi::system_settings::users_groups::{UsersGroupsManager, User, Group};
use crate::gui::addon_widgets::tabbar;
use crate::gui::styles::CustomContainer;
use iced::{
   button, Container, Length, Column, Text, Element
};

#[derive(Debug, Default)]
pub struct UserGroupPage {
   usrgrp_mn: UsersGroupsManager,
   tabbar_state: Vec<(&'static str, button::State)>,
   curr_tab_idx: usize,
   ls_users: Vec<User>,
   ls_grps: Vec<Group>,
   curr_usr: Option<User>,
   content: ContentPage,
}

#[derive(Debug)]
pub enum ContentPage {
   Users(UsersTab),
   Groups(GroupsTab),
   Empty
}

impl Default for ContentPage {
   fn default() -> Self {
      Self::Empty
   }
}

#[derive(Debug, Clone)]
pub enum UserGroupMsg {
   TabChanged(usize),
   UsersMSG(UsersMsg),
   GroupsMSG(GroupsMsg),
}

impl UserGroupPage {
   pub fn new() -> Self {
      use ContentPage::*;
      let tabs = vec![
         ("Users", button::State::new()),
         ("Groups", button::State::new()),
      ];

      match UsersGroupsManager::new() {
         Ok(mut usrgrp_mn) => {
            let mut ls_users: Vec<User> = usrgrp_mn.list_users().to_owned();
            let ls_grps: Vec<Group> = usrgrp_mn.list_groups().to_owned();
            let mut curr_usr = usrgrp_mn.current_user().map(ToOwned::to_owned);

            Self {
               tabbar_state: tabs,
               content: Users(UsersTab::new(&mut usrgrp_mn, &mut ls_users, &mut curr_usr)),
               ls_users,
               ls_grps,
               curr_usr,
               usrgrp_mn,
               ..Self::default()
            }
         },
         Err(err) => {
            eprintln!("{:?}", err);
            Self::default()
         }
      }
   }

   pub fn update(&mut self, msg: UserGroupMsg) {
      use UserGroupMsg::*;
      use ContentPage::*;
      let Self {
         usrgrp_mn, ls_users, ls_grps, curr_usr, content, ..
      } = self;

      match msg {
         TabChanged(idx) => {
            self.curr_tab_idx = idx;
            match idx {
               0 => self.content = Users(UsersTab::new(usrgrp_mn, ls_users, curr_usr)),
               1 => self.content = Groups(GroupsTab::new(usrgrp_mn, ls_grps, ls_users, curr_usr.as_ref().map(|usr| usr.is_admin()).unwrap_or(false))),
               _ => self.content = Empty,
            }
         },
         UsersMSG(users_msg) => {
            if let Users(users_tab) = content {
               users_tab.update(users_msg);
            }
         },
         GroupsMSG(groups_msg) => {
            if let Groups(groups_tab) = content {
               groups_tab.update(groups_msg);
            }
         }, 
      }
   }

   pub fn view(&mut self) -> Element<UserGroupMsg> {
      use ContentPage::*;
      use UserGroupMsg::*;
      let Self {
         tabbar_state,
         content,
         ..
      } = self;

      // របារផ្ទាំង
      let tabbar_sec = tabbar(tabbar_state, self.curr_tab_idx, TabChanged);

      // ទិដ្ឋភាពទូទៅ
      let tabview = match content {
         Users(users_tab) => users_tab.view().map(|msg| UsersMSG(msg)),
         Groups(groups_tab) => groups_tab.view().map(|msg| GroupsMSG(msg)),
         Empty => Container::new(
            Text::new("There is no content available")
         ).width(Length::Fill).height(Length::Fill).style(CustomContainer::Header).center_x().center_y().into()
      };

      // មាតិកា
      let content = Column::new().width(Length::Fill)
         .push(tabbar_sec)
         .push(Container::new(tabview).width(Length::Fill).height(Length::Fill).padding(15).style(CustomContainer::ForegroundGray));

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}