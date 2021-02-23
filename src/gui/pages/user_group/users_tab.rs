mod add_user_page;
mod change_pwd_page;
mod user_info_page;
mod change_user_info_page;

use {
   user_info_page::{UserInfoPage, UserInfoMsg}, change_pwd_page::{ChangePwdPage, ChangePwdMsg}, 
   add_user_page::{AddUserPage, AddUserMsg}, change_user_info_page::{ChangeInfoPage, ChangeInfoMsg},
};
use std::{cell::RefCell, rc::Rc};
use libkoompi::system_settings::users_groups::{User, UsersGroupsManager, AccountType};
use iced::{
   button, scrollable, Scrollable, Button, Text, Container, Length, Column, Row, Align, Element, Image,
};
use iced_custom_widget::Icon;
use crate::gui::styles::{CustomButton, CustomContainer};

#[derive(Debug, Default)]
pub struct UsersTab {
   usrgrp_mn: RefCell<UsersGroupsManager>,
   curr_usr: RefCell<User>,
   ls_users: Vec<(User, button::State)>,
   selected_user: Option<usize>,
   scroll_users: scrollable::State,
   add_state: button::State,
   remove_state: button::State,

   // dynamic section
   content: ContentPage,
}

#[derive(Debug)]
pub enum ContentPage {
   UserInfo(UserInfoPage),
   ChangePwd(ChangePwdPage),
   ChangeInfo(ChangeInfoPage),
   AddUser(AddUserPage),
}

impl Default for ContentPage {
   fn default() -> Self {
      ContentPage::UserInfo(UserInfoPage::default())
   }
}

#[derive(Debug, Clone)]
pub enum UsersMsg {
   SelectedUsr(usize),
   AddClicked,
   RemoveClicked,
   UserInfoMSG(UserInfoMsg),
   ChangePwdMSG(ChangePwdMsg),
   AddUserMSG(AddUserMsg),
   ChangeInfoMSG(ChangeInfoMsg),
}

impl UsersTab {
   pub fn new(usrgrp_mn: Rc<RefCell<UsersGroupsManager>>) -> Self {
      let usrgrp_mn_ptr = Rc::into_raw(usrgrp_mn);
      let usrgrp_mn = unsafe { &*usrgrp_mn_ptr };
      let usrgrp_ref = usrgrp_mn.borrow();
      let list_users = usrgrp_ref.list_users();
      let curr_usr = list_users[0].clone();
      let selected_user = list_users.iter().position(|usr| usr.uid().eq(&usrgrp_ref.current_uid()));
      
      Self {
         usrgrp_mn: usrgrp_mn.clone(),
         curr_usr: RefCell::new(curr_usr.clone()),
         ls_users: list_users.iter().map(|&usr| (usr.clone(), button::State::new())).collect(),
         selected_user,
         content: ContentPage::UserInfo(UserInfoPage::new(&curr_usr, true, curr_usr.is_admin())),
         scroll_users: Default::default(),
         add_state: Default::default(),
         remove_state: Default::default(),
      }
   }

   pub fn update(&mut self, msg: UsersMsg) {
      use UsersMsg::*;
      use ContentPage::*;

      let Self {
         usrgrp_mn,
         ls_users,
         content,
         curr_usr,
         ..
      } = self;

      let curr_usr = curr_usr.borrow();

      match msg {
         SelectedUsr(idx) => {
            self.selected_user = Some(idx);
            if let Some((user, _)) = ls_users.get(idx) {
               if let UserInfo(user_info_page) = content {
                  user_info_page.with_user(user, curr_usr.uid().eq(&user.uid()));
               }
            }
         },
         AddClicked => {
            if curr_usr.is_admin() {
               self.content = AddUser(AddUserPage::new());
            }
         },
         RemoveClicked => {
            if curr_usr.is_admin() {
               if let Some(selected) = self.selected_user {
                  if let Some((user, _)) = ls_users.get(selected) {
                     match usrgrp_mn.borrow_mut().delete_user(user.username(), false) {
                        Ok(is_ok) => if is_ok {
                           ls_users.remove(selected);
                        } else {
                           println!("can not delete group");
                        },
                        Err(err) => eprintln!("{:?}", err)
                     }
                  }
               }
               self.selected_user = None;
            }
         },
         UserInfoMSG(usr_info_msg) => {
            if let UserInfo(user_info_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get_mut(idx) {
                     use UserInfoMsg::*;
                     match usr_info_msg {
                        ChangePwdClicked => self.content = ChangePwd(ChangePwdPage::new(curr_usr.uid().eq(&user.uid()))),
                        ChangeInfoClicked => self.content = ChangeInfo(ChangeInfoPage::new(user, usrgrp_mn.borrow().all_groups().iter().find(|grp| grp.gid() == user.gid()).map(|grp| grp.name()), usrgrp_mn.borrow().login_shells().to_vec())),
                        AllowUsrAdminToggled(is_checked) => if curr_usr.is_admin() {
                           match user.change_account_type(if is_checked {AccountType::Admin} else {AccountType::default()}) {
                              Ok(()) => user_info_page.with_user(user, curr_usr.uid().eq(&user.uid())),
                              Err(err) => eprintln!("{:?}", err),
                           }
                        },
                     }
                  }
               }
            }
         },
         ChangePwdMSG(change_pwd_msg) => {
            if let ChangePwd(change_pwd_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get(idx) {
                     use ChangePwdMsg::*;
                     match change_pwd_msg {
                        CancelClicked => self.content = UserInfo(UserInfoPage::new(&user, curr_usr.uid().eq(&user.uid()), curr_usr.is_admin())),
                        ChangeClicked(old_pwd, new_pwd, verify_pwd) => {
                           if curr_usr.uid().eq(&user.uid()) {
                              match usrgrp_mn.borrow_mut().change_user_password(user.username(), &old_pwd, &new_pwd, &verify_pwd) {
                                 Ok(is_ok) => if is_ok {
                                    println!("Change password Success")
                                 } else {
                                    println!("can not Change password")
                                 }, 
                                 Err(err) => eprintln!("{:?}", err),
                              }
                           } else {
                              if curr_usr.is_admin() {
                                 match usrgrp_mn.borrow_mut().reset_user_password(user.username(), &new_pwd, &verify_pwd) {
                                    Ok(is_ok) => if is_ok {
                                       println!("Reset password Success")
                                    } else {
                                       println!("can not Reset password")
                                    },
                                    Err(err) => eprintln!("{:?}", err),
                                 }
                              }
                           }
                           self.content = UserInfo(UserInfoPage::new(&user, curr_usr.uid().eq(&user.uid()), curr_usr.is_admin()));
                        },
                        _ => change_pwd_page.update(change_pwd_msg)
                     }
                  }
               }
            }
         },
         AddUserMSG(add_user_msg) => {
            if curr_usr.is_admin() {
               if let AddUser(add_user_page) = content {
                  use AddUserMsg::*;
                  match add_user_msg {
                     CreateClicked(user) => {
                        match usrgrp_mn.borrow_mut().create_user(user.fullname, user.username.clone(), user.acc_type, user.pwd, user.pwd_verify) {
                           Ok(is_user) => {
                              if is_user {
                                 if let Some(user) = usrgrp_mn.borrow().user_from_name(user.username.as_str()) {
                                    ls_users.push((user.clone(), button::State::new()));
                                    self.selected_user = ls_users.iter().map(|(usr, _)| usr).position(|usr| usr.uid() == user.uid());
                                    self.content = UserInfo(UserInfoPage::new(&user, curr_usr.uid().eq(&user.uid()), curr_usr.is_admin()));
                                 }
                              } else {
                                 self.selected_user = Some(0);
                                 self.content = UserInfo(UserInfoPage::new(&curr_usr, true, curr_usr.is_admin()));
                              };
                           },
                           Err(err) => eprintln!("{:?}", err)
                        }
                     },
                     CancelClicked => {
                        let user = usrgrp_mn.borrow().list_users()[self.selected_user.unwrap_or(0)].clone();
                        self.content = UserInfo(UserInfoPage::new(&user, curr_usr.uid().eq(&user.uid()), curr_usr.is_admin()));
                     },
                     _ => add_user_page.update(add_user_msg)
                  }
               }
            }
         },
         ChangeInfoMSG(change_info_msg) => {
            if let ChangeInfo(change_info_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get(idx) {
                     use ChangeInfoMsg::*;
                     match change_info_msg {
                        CancelClicked => self.content = UserInfo(UserInfoPage::new(&user, curr_usr.uid().eq(&user.uid()), curr_usr.is_admin())),
                        OkayClicked(usernew) => {
                           let (uid, usrname, home_dir) = if curr_usr.uid().ne(&user.uid()) {
                              (Some(usernew.uid.to_string()), Some(usernew.username), Some(usernew.home_dir))
                           } else {
                              (None, None, None)
                           };
                           match usrgrp_mn.borrow_mut().change_user_info(user.username().to_string(), uid, usernew.gname, usernew.fullname, usrname, usernew.login_shell, home_dir) {
                              Ok(is_ok) => if is_ok {
                                 println!("edit user success");
                              } else {
                                 println!("can not edit user");
                              },
                              Err(err) => eprintln!("{:?}", err)
                           }
                           self.content = UserInfo(UserInfoPage::new(&user, curr_usr.uid().eq(&user.uid()), curr_usr.is_admin()))
                        },
                        _ => change_info_page.update(change_info_msg)
                     }
                  }
               }
            }
         }
      }
   }

   pub fn view(&mut self) -> Element<UsersMsg> {
      use UsersMsg::*;
      use ContentPage::*;
      let Self {
         ls_users, curr_usr, selected_user, scroll_users, add_state, remove_state, content, ..
      } = self;

      let is_admin = curr_usr.borrow().is_admin();
      let scrollable_users = ls_users.iter_mut().enumerate().fold(Scrollable::new(scroll_users).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (user, state))| {
         // let mut profile_path = user.home_dir().join(".face");
         // if !profile_path.exists() {
         //    profile_path = ;
         // }
         let row = Row::new().spacing(10).align_items(Align::Center)
            .push(Container::new(Image::new(user.home_dir().join(".face").to_path_buf()).width(Length::Units(30)).height(Length::Units(30))).width(Length::Units(30)).height(Length::Units(30)).style(CustomContainer::Header))
            .push(
               Column::new().spacing(5)
               .push(Text::new(user.fullname()))
               .push(Text::new(&user.account_type().to_string()).size(11))
            );
         let mut btn = Button::new(state, row).width(Length::Fill).style(
            if let Some(selected) = *selected_user {
               if selected == idx {CustomButton::Selected}
               else {CustomButton::Text}
            }
            else {CustomButton::Text}
         );
         if let UserInfo(_) = content {
            btn = btn.on_press(SelectedUsr(idx));
         }
         scrollable.push(btn)
      });
      let mut btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).style(CustomButton::Text);
      let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
      if let Some(idx) = *selected_user {
         if idx.ne(&0) && is_admin {
            btn_remove = btn_remove.on_press(RemoveClicked);
         }
      }
      if is_admin {
         btn_add = btn_add.on_press(AddClicked);
      }
      let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
      let users_pane = Container::new(
         Column::new()
         .push(Container::new(Text::new("Users")).width(Length::Fill).padding(7).style(CustomContainer::Header))
         .push(scrollable_users)
         .push(btn_group)
      ).height(Length::Fill).width(Length::FillPortion(3)).style(CustomContainer::ForegroundWhite);

      let right_sec = match content {
         AddUser(add_user_page) => add_user_page.view().map(|msg| AddUserMSG(msg)),
         ChangePwd(change_pwd_page) => change_pwd_page.view().map(|msg| ChangePwdMSG(msg)),
         ChangeInfo(change_info_page) => change_info_page.view().map(|msg| ChangeInfoMSG(msg)),
         UserInfo(user_info_page) => user_info_page.view().map(|msg| UserInfoMSG(msg))
      };

      Container::new(
         Row::new().width(Length::Fill).spacing(10)
         .push(users_pane)
         .push(right_sec)
      ).width(Length::Fill).height(Length::Fill).into()
   }
}