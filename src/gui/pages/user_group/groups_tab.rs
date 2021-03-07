mod add_group_page;
mod edit_group_page;

use crate::gui::styles::{CustomButton, CustomContainer};
use iced::{button, scrollable, Button, Column, Container, Element, Length, Row, Scrollable, Text};
use iced_custom_widget::{Icon, Icons};
use libkoompi::system_settings::users_groups::{Group, User, UsersGroupsManager};
use std::cell::RefCell;
use {
   add_group_page::{AddGroupMsg, AddGroupPage},
   edit_group_page::{EditGroupMsg, EditGroupPage},
};

#[derive(Debug, Default)]
pub struct GroupsTab {
   usrgrp_mn: RefCell<UsersGroupsManager>,
   curr_usr_is_admin: bool,
   ls_grps: Vec<(RefCell<Group>, button::State)>,
   ls_users: RefCell<Vec<User>>,
   selected_grp: Option<usize>,
   scroll_grps: scrollable::State,
   add_state: button::State,
   remove_state: button::State,

   // dynamic section
   content: ContentPage,
}

#[derive(Debug)]
pub enum ContentPage {
   Empty,
   EditGroup(EditGroupPage),
   AddGroup(AddGroupPage),
}

impl Default for ContentPage {
   fn default() -> Self {
      Self::Empty
   }
}

#[derive(Debug, Clone)]
pub enum GroupsMsg {
   SelecteGroup(usize),
   AddClicked,
   RemoveClicked,
   EditGroupMSG(EditGroupMsg),
   AddGroupMSG(AddGroupMsg),
}

impl GroupsTab {
   pub fn new(usrgrp_mn: &mut UsersGroupsManager, ls_grps: &mut Vec<Group>, ls_usrs: &mut Vec<User>, curr_usr_is_admin: bool) -> Self {
      use ContentPage::*;
      let curr_group = ls_grps.first();

      Self {
         usrgrp_mn: RefCell::new(usrgrp_mn.clone()),
         curr_usr_is_admin,
         ls_grps: ls_grps.iter().map(|grp| (RefCell::new(grp.to_owned()), button::State::new())).collect(),
         ls_users: RefCell::new(ls_usrs.to_vec()),
         selected_grp: curr_group.map(|_| 0),
         content: if let Some(curr_group) = curr_group {
            EditGroup(EditGroupPage::new(curr_group, ls_usrs.as_slice(), curr_usr_is_admin))
         } else {
            Empty
         },
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: GroupsMsg) {
      use ContentPage::*;
      use GroupsMsg::*;

      let Self { usrgrp_mn, ls_grps, ls_users, content, .. } = self;

      let is_admin = self.curr_usr_is_admin;
      let ls_users = ls_users.borrow();

      match msg {
         SelecteGroup(idx) => {
            self.selected_grp = Some(idx);
            if let EditGroup(edit_group_page) = content {
               if let Some((grp, _)) = ls_grps.get(idx) {
                  edit_group_page.with_grp(&grp.borrow(), ls_users.as_slice());
               }
            }
         }
         AddClicked => {
            if is_admin {
               self.content = AddGroup(AddGroupPage::new())
            }
         }
         RemoveClicked => {
            if is_admin {
               if let Some(idx) = self.selected_grp {
                  if let Some((group, _)) = ls_grps.get(idx) {
                     let grp = group.borrow();
                     match usrgrp_mn.borrow_mut().delete_group(grp.name()) {
                        Ok(is_ok) => {
                           if is_ok {
                              std::mem::drop(grp);
                              ls_grps.remove(idx);
                              if let Some((grp, _)) = ls_grps.first() {
                                 self.selected_grp = Some(0);
                                 self.content = EditGroup(EditGroupPage::new(&grp.borrow(), ls_users.as_slice(), self.curr_usr_is_admin));
                              } else {
                                 self.selected_grp = None;
                                 self.content = Empty;
                              }
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
         EditGroupMSG(edit_group_msg) => {
            if let EditGroup(edit_group_page) = content {
               if let Some(idx) = self.selected_grp {
                  if let Some((group, _)) = ls_grps.get_mut(idx) {
                     use EditGroupMsg::*;
                     let mut grp = group.borrow_mut();
                     match edit_group_msg {
                        GroupNameSubmitted(grp_name) => {
                           println!("Group name submitted: {}", grp_name);
                           // edit_group_page.with_grp(&grp, ls_users.as_slice());
                        }
                        OkayClicked(grp_name, ls_members) => {
                           {
                              match usrgrp_mn.borrow_mut().change_group_members(grp.name(), ls_members.iter().map(|usr| usr.as_str()).collect()) {
                                 Ok(newgrp) => {
                                    if let Some(newgrp) = newgrp {
                                       println!("change group members success");
                                       *grp = newgrp;
                                    } else {
                                       println!("did not change group members")
                                    }
                                 }
                                 Err(err) => eprintln!("{:?}", err),
                              }
                           }
                           {
                              match usrgrp_mn.borrow_mut().change_group_name(grp.name(), &grp_name) {
                                 Ok(newgrp) => {
                                    if let Some(newgrp) = newgrp {
                                       println!("change group name success");
                                       *grp = newgrp;
                                    } else {
                                       println!("did not change group name")
                                    }
                                 }
                                 Err(err) => eprintln!("{:?}", err),
                              }
                           }
                           edit_group_page.with_grp(&grp, ls_users.as_slice());
                        }
                        _ => edit_group_page.update(edit_group_msg),
                     }
                  }
               }
            }
         }
         AddGroupMSG(add_group_msg) => {
            if let AddGroup(add_group_page) = content {
               use AddGroupMsg::*;
               match add_group_msg {
                  CreateClicked(group_name) => match usrgrp_mn.borrow_mut().create_group(group_name.as_str()) {
                     Ok(grp) => {
                        if let Some(grp) = grp {
                           ls_grps.push((RefCell::new(grp.to_owned()), button::State::new()));
                           self.selected_grp = Some(ls_grps.len() - 1);
                           if let Some((grp, _)) = ls_grps.last() {
                              self.content = EditGroup(EditGroupPage::new(&grp.borrow(), ls_users.as_slice(), self.curr_usr_is_admin));
                           } else {
                              self.content = Empty;
                           }
                        } else {
                           if let Some(idx) = self.selected_grp {
                              if let Some((grp, _)) = ls_grps.get(idx) {
                                 self.content = EditGroup(EditGroupPage::new(&grp.borrow(), ls_users.as_slice(), self.curr_usr_is_admin));
                              }
                           } else {
                              self.content = Empty;
                           }
                        }
                     }
                     Err(err) => eprintln!("{:?}", err),
                  },
                  CancelClicked => {
                     if let Some(idx) = self.selected_grp {
                        if let Some((grp, _)) = ls_grps.get(idx) {
                           self.content = EditGroup(EditGroupPage::new(&grp.borrow(), ls_users.as_slice(), self.curr_usr_is_admin));
                        }
                     } else {
                        self.content = Empty;
                     }
                  }
                  _ => add_group_page.update(add_group_msg),
               }
            }
         }
      }
   }

   pub fn view(&mut self) -> Element<GroupsMsg> {
      use ContentPage::*;
      use GroupsMsg::*;

      let Self {
         ls_grps,
         selected_grp,
         scroll_grps,
         add_state,
         remove_state,
         content,
         ..
      } = self;

      let is_admin = self.curr_usr_is_admin;
      let scrollable_group = ls_grps
         .iter_mut()
         .enumerate()
         .fold(Scrollable::new(scroll_grps).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (grp, state))| {
            let group = grp.borrow();
            let mut btn = Button::new(state, Text::new(group.formatted_name())).width(Length::Fill).style(if let Some(selected) = *selected_grp {
               if selected == idx {
                  CustomButton::Selected
               } else {
                  CustomButton::Text
               }
            } else {
               CustomButton::Text
            });
            if let EditGroup(_) = content {
               if is_admin {
                  btn = btn.on_press(SelecteGroup(idx));
               }
            }
            scrollable.push(btn)
         });
      let mut btn_add = Button::new(add_state, Icon::new(Icons::Ad).size(23)).padding(2).style(CustomButton::Text);
      let mut btn_remove = Button::new(remove_state, Icon::new(Icons::Minus).size(23)).padding(2).style(CustomButton::Text);
      if is_admin {
         btn_add = btn_add.on_press(AddClicked);
      }
      if selected_grp.is_some() && is_admin {
         btn_remove = btn_remove.on_press(RemoveClicked);
      }
      let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
      let group_pane = Container::new(Column::new().push(Container::new(Text::new("Groups")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_group).push(btn_group))
         .height(Length::Fill)
         .width(Length::FillPortion(3))
         .style(CustomContainer::ForegroundWhite);

      let right_sec = match content {
         AddGroup(add_group_page) => add_group_page.view().map(|msg| AddGroupMSG(msg)),
         EditGroup(edit_group_page) => edit_group_page.view().map(|msg| EditGroupMSG(msg)),
         Empty => Container::new(Text::new("There is no groups available")).width(Length::Fill).height(Length::Fill).center_x().center_y().into(),
      };

      Container::new(Row::new().width(Length::Fill).spacing(10).push(group_pane).push(Container::new(right_sec).width(Length::FillPortion(7))))
         .width(Length::Fill)
         .height(Length::Fill)
         .into()
   }
}
