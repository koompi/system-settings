mod add_user_page;
mod change_groups_page;
mod change_pwd_page;
mod change_user_info_page;
mod user_info_page;

use crate::gui::styles::{CustomButton, CustomContainer};
use iced::{button, scrollable, Align, Button, Column, Container, Element, Image, Length, Row, Scrollable, Space, Text};
use iced_custom_widget::Icon;
use libkoompi::system_settings::users_groups::{AccountType, User, UsersGroupsManager};
use std::cell::RefCell;
use {
   add_user_page::{AddUserMsg, AddUserPage},
   change_groups_page::{ChangeGroupsMsg, ChangeGroupsPage},
   change_pwd_page::{ChangePwdMsg, ChangePwdPage},
   change_user_info_page::{ChangeInfoMsg, ChangeInfoPage},
   user_info_page::{UserInfoMsg, UserInfoPage},
};

#[derive(Debug, Default)]
pub struct UsersTab {
   usrgrp_mn: RefCell<UsersGroupsManager>,
   curr_usr: RefCell<User>,
   ls_users: Vec<(RefCell<User>, button::State)>,
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
   ChangeGroups(ChangeGroupsPage),
   AddUser(AddUserPage),
   Empty,
}

impl Default for ContentPage {
   fn default() -> Self {
      Self::Empty
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
   ChangeGroupsMSG(ChangeGroupsMsg),
}

impl UsersTab {
   pub fn new(usrgrp_mn: &mut UsersGroupsManager, ls_users: &mut Vec<User>, curr_usr: &mut Option<User>) -> Self {
      use ContentPage::*;
      let (curr_usr, selected_user, content) = match curr_usr {
         Some(usr) => (usr.to_owned(), Some(0), UserInfo(UserInfoPage::new(usr, true, usr.is_admin()))),
         None => (User::default(), None, Empty),
      };

      Self {
         usrgrp_mn: RefCell::new(usrgrp_mn.to_owned()),
         curr_usr: RefCell::new(curr_usr),
         ls_users: ls_users.iter().map(|usr| (RefCell::new(usr.to_owned()), button::State::new())).collect(),
         selected_user,
         content,
         scroll_users: Default::default(),
         add_state: Default::default(),
         remove_state: Default::default(),
      }
   }

   pub fn update(&mut self, msg: UsersMsg) {
      use ContentPage::*;
      use UsersMsg::*;

      let Self { usrgrp_mn, ls_users, content, curr_usr, .. } = self;

      let curr_usr = curr_usr.borrow();
      let curr_is_admin = curr_usr.is_admin();
      let is_curr_usr = |uid: u16| curr_usr.uid().eq(&uid);

      match msg {
         SelectedUsr(idx) => {
            self.selected_user = Some(idx);
            if let Some((user, _)) = ls_users.get(idx) {
               let usr = user.borrow();
               let is_curr_usr = is_curr_usr(usr.uid());

               match content {
                  UserInfo(user_info_page) => user_info_page.with_user(&usr, is_curr_usr),
                  ChangeInfo(change_user_info_page) => {
                     let usrgrp_ref = usrgrp_mn.borrow();
                     let groupname = usrgrp_ref.all_groups().iter().find(|grp| grp.gid() == usr.gid()).map(|grp| grp.name());
                     change_user_info_page.with_user(&usr, is_curr_usr, groupname);
                  }
                  ChangeGroups(change_groups_page) => change_groups_page.with_user(&usr),
                  _ => self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin)),
               }
            }
         }
         AddClicked => {
            if curr_is_admin {
               self.content = AddUser(AddUserPage::new());
            }
         }
         RemoveClicked => {
            if curr_is_admin {
               if let Some(selected) = self.selected_user {
                  if let Some((user, _)) = ls_users.get(selected) {
                     let usr = user.borrow();

                     match usrgrp_mn.borrow_mut().delete_user(usr.username(), false) {
                        Ok(is_ok) => {
                           if is_ok {
                              std::mem::drop(usr);
                              ls_users.remove(selected);
                              self.selected_user = Some(0);
                              self.content = UserInfo(UserInfoPage::new(&curr_usr, true, curr_is_admin));
                           } else {
                              println!("can not delete group");
                           }
                        }
                        Err(err) => eprintln!("{:?}", err),
                     }
                  }
               }
            }
         }
         UserInfoMSG(usr_info_msg) => {
            if let UserInfo(user_info_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get_mut(idx) {
                     let mut usr = user.borrow_mut();
                     let is_curr_usr = is_curr_usr(usr.uid());

                     use UserInfoMsg::*;
                     match usr_info_msg {
                        ChangePwdClicked => self.content = ChangePwd(ChangePwdPage::new(is_curr_usr)),
                        ChangeInfoClicked => {
                           let usrgrp_ref = usrgrp_mn.borrow();
                           let groupname = usrgrp_ref.all_groups().iter().find(|grp| grp.gid() == usr.gid()).map(|grp| grp.name());
                           let login_shells = usrgrp_ref.login_shells().to_vec();
                           self.content = ChangeInfo(ChangeInfoPage::new(&usr, is_curr_usr, groupname, login_shells));
                        }
                        AllowUsrAdminToggled(is_checked) => {
                           if curr_is_admin {
                              match usrgrp_mn.borrow_mut().change_user_type(usr.username(), if is_checked { AccountType::Admin } else { AccountType::default() }) {
                                 Ok(newusr) => {
                                    if let Some(newusr) = newusr {
                                       println!("Change user type Success");
                                       *usr = newusr;
                                       user_info_page.with_user(&usr, is_curr_usr);
                                    } else {
                                       println!("can not Change user type");
                                    }
                                 }
                                 Err(err) => eprintln!("{:?}", err),
                              }
                           }
                        }
                        ChangeGroupClicked => {
                           let usrgrp_ref = usrgrp_mn.borrow();
                           let all_groups = usrgrp_ref.all_groups();
                           self.content = ChangeGroups(ChangeGroupsPage::new(&usr, all_groups));
                        }
                     }
                  }
               }
            }
         }
         ChangePwdMSG(change_pwd_msg) => {
            if let ChangePwd(change_pwd_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get_mut(idx) {
                     let mut usr = user.borrow_mut();
                     let is_curr_usr = is_curr_usr(usr.uid());

                     use ChangePwdMsg::*;
                     match change_pwd_msg {
                        CancelClicked => self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin)),
                        ChangeClicked(old_pwd, new_pwd, verify_pwd) => {
                           if is_curr_usr {
                              match usrgrp_mn.borrow_mut().change_user_password(usr.username(), &old_pwd, &new_pwd, &verify_pwd) {
                                 Ok(newusr) => {
                                    if let Some(newusr) = newusr {
                                       println!("Change password Success");
                                       *usr = newusr;
                                    } else {
                                       println!("can not Change password")
                                    }
                                 }
                                 Err(err) => eprintln!("{:?}", err),
                              }
                           } else if curr_is_admin {
                              match usrgrp_mn.borrow_mut().reset_user_password(usr.username(), &new_pwd, &verify_pwd) {
                                 Ok(newusr) => {
                                    if let Some(newusr) = newusr {
                                       println!("Reset password Success");
                                       *usr = newusr;
                                    } else {
                                       println!("can not Reset password")
                                    }
                                 }
                                 Err(err) => eprintln!("{:?}", err),
                              }
                           }
                           self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin));
                        }
                        _ => change_pwd_page.update(change_pwd_msg),
                     }
                  }
               }
            }
         }
         AddUserMSG(add_user_msg) => {
            if curr_is_admin {
               if let AddUser(add_user_page) = content {
                  use AddUserMsg::*;
                  match add_user_msg {
                     CreateClicked(user) => match usrgrp_mn.borrow_mut().create_user(user.fullname, user.username.clone(), user.acc_type, user.pwd, user.pwd_verify) {
                        Ok(newuser) => {
                           if let Some(newuser) = newuser {
                              ls_users.push((RefCell::new(newuser.to_owned()), button::State::new()));
                              self.selected_user = Some(ls_users.len() - 1);
                              self.content = UserInfo(UserInfoPage::new(&newuser, is_curr_usr(newuser.uid()), curr_is_admin));
                           } else {
                              self.selected_user = Some(0);
                              self.content = UserInfo(UserInfoPage::new(&curr_usr, true, curr_is_admin));
                           };
                        }
                        Err(err) => eprintln!("{:?}", err),
                     },
                     CancelClicked => {
                        let user = ls_users.iter().map(|(usr, _)| usr).collect::<Vec<&RefCell<User>>>()[self.selected_user.unwrap_or(0)];
                        let usr = user.borrow();
                        self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr(usr.uid()), curr_is_admin));
                     }
                     _ => add_user_page.update(add_user_msg),
                  }
               }
            }
         }
         ChangeInfoMSG(change_info_msg) => {
            if let ChangeInfo(change_info_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get_mut(idx) {
                     let mut usr = user.borrow_mut();
                     let is_curr_usr = is_curr_usr(usr.uid());

                     use ChangeInfoMsg::*;
                     match change_info_msg {
                        CancelClicked => self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin)),
                        OkayClicked(usernew) => {
                           match usrgrp_mn.borrow_mut().change_user_info(usr.username().to_string(), usernew.uid.to_string(), usernew.gname, usernew.fullname, usernew.username, usernew.login_shell, usernew.home_dir)
                           {
                              Ok(newusr) => {
                                 if let Some(newusr) = newusr {
                                    println!("edit user success");
                                    *usr = newusr;
                                 } else {
                                    println!("can not edit user");
                                 }
                              }
                              Err(err) => eprintln!("{:?}", err),
                           }
                           self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin))
                        }
                        _ => change_info_page.update(change_info_msg),
                     }
                  }
               }
            }
         }
         ChangeGroupsMSG(change_groups_msg) => {
            if let ChangeGroups(change_groups_page) = content {
               if let Some(idx) = self.selected_user {
                  if let Some((user, _)) = ls_users.get_mut(idx) {
                     let mut usr = user.borrow_mut();
                     let is_curr_usr = is_curr_usr(usr.uid());

                     use ChangeGroupsMsg::*;
                     match change_groups_msg {
                        CancelClicked => self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin)),
                        OkayClicked(ls_groups) => {
                           match usrgrp_mn.borrow_mut().change_user_groups(usr.username(), ls_groups.iter().map(|grp| grp.as_str()).collect::<Vec<&str>>()) {
                              Ok(newusr) => {
                                 if let Some(newusr) = newusr {
                                    println!("change user groups success");
                                    *usr = newusr;
                                 } else {
                                    println!("can not change user groups");
                                 }
                              }
                              Err(err) => eprintln!("{:?}", err),
                           }
                           self.content = UserInfo(UserInfoPage::new(&usr, is_curr_usr, curr_is_admin))
                        }
                        _ => change_groups_page.update(change_groups_msg),
                     }
                  }
               }
            }
         }
      }
   }

   pub fn view(&mut self) -> Element<UsersMsg> {
      use ContentPage::*;
      use UsersMsg::*;
      let Self {
         ls_users,
         curr_usr,
         selected_user,
         scroll_users,
         add_state,
         remove_state,
         content,
         ..
      } = self;

      let is_admin = curr_usr.borrow().is_admin();
      let scrollable_users = ls_users
         .iter_mut()
         .enumerate()
         .fold(Scrollable::new(scroll_users).height(Length::Fill).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (usr, state))| {
            let user = usr.borrow();
            let profile_pic: Element<_> = if user.profile_path().exists() {
               Image::new(user.profile_path()).width(Length::Units(30)).height(Length::Units(30)).into()
            } else {
               Container::new(Row::new()).width(Length::Units(30)).height(Length::Units(30)).style(CustomContainer::Header).into()
            };
            let row = Row::new()
               .spacing(10)
               .align_items(Align::Center)
               .push(profile_pic)
               .push(Column::new().spacing(5).push(Text::new(user.fullname())).push(Text::new(&user.account_type().to_string()).size(11)));
            let mut btn = Button::new(state, row).width(Length::Fill).style(if let Some(selected) = *selected_user {
               if selected == idx {
                  CustomButton::Selected
               } else {
                  CustomButton::Text
               }
            } else {
               CustomButton::Text
            });
            if let UserInfo(_) | ChangeInfo(_) | ChangeGroups(_) = content {
               btn = btn.on_press(SelectedUsr(idx));
            }
            let row_btn = Row::new().width(Length::Fill).push(Space::with_width(Length::Units(7))).push(btn).push(Space::with_width(Length::Units(7)));

            scrollable.push::<Element<_>>(match idx {
               0 => Column::new()
                  .spacing(2)
                  .push(
                     Container::new(Container::new(Text::new("Current User").size(11)).width(Length::Fill).padding(3).style(CustomContainer::Header))
                        .width(Length::Fill)
                        .padding(2),
                  )
                  .push(row_btn)
                  .into(),
               1 => Column::new()
                  .spacing(2)
                  .push(
                     Container::new(Container::new(Text::new("Other Users").size(11)).width(Length::Fill).padding(3).style(CustomContainer::Header))
                        .width(Length::Fill)
                        .padding(2),
                  )
                  .push(row_btn)
                  .into(),
               _ => row_btn.into(),
            })
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
      let users_pane = Container::new(Column::new().push(Container::new(Text::new("Users")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_users).push(btn_group))
         .height(Length::Fill)
         .width(Length::FillPortion(3))
         .style(CustomContainer::ForegroundWhite);

      let right_sec = match content {
         AddUser(add_user_page) => add_user_page.view().map(|msg| AddUserMSG(msg)),
         ChangePwd(change_pwd_page) => change_pwd_page.view().map(|msg| ChangePwdMSG(msg)),
         ChangeInfo(change_info_page) => change_info_page.view().map(|msg| ChangeInfoMSG(msg)),
         UserInfo(user_info_page) => user_info_page.view().map(|msg| UserInfoMSG(msg)),
         ChangeGroups(change_groups_page) => change_groups_page.view().map(|msg| ChangeGroupsMSG(msg)),
         Empty => Container::new(Text::new("There is no users available")).width(Length::Fill).height(Length::Fill).center_x().center_y().into(),
      };

      Container::new(Row::new().width(Length::Fill).spacing(10).push(users_pane).push(Container::new(right_sec).width(Length::FillPortion(7))))
         .width(Length::Fill)
         .height(Length::Fill)
         .into()
   }
}
