mod add_group_page;
mod edit_group_page;

use {
   add_group_page::{AddGroupPage, AddGroupMsg}, edit_group_page::{EditGroupPage, EditGroupMsg},
};
use std::{cell::RefCell, rc::Rc};
use libkoompi::system_settings::users_groups::{UsersGroupsManager, Group, User};
use iced::{
   button, scrollable, Scrollable, Button, Text, Container, Length, Column, Row, Element,
};
use iced_custom_widget::Icon;
use crate::gui::styles::{CustomButton, CustomContainer};

#[derive(Debug, Default)]
pub struct GroupsTab {
   usrgrp_mn: RefCell<UsersGroupsManager>,
   curr_usr_is_admin: bool,
   ls_grps: Vec<(Group, button::State)>,
   ls_users: Vec<User>,
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
      Self::EditGroup(EditGroupPage::default())
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
   pub fn new(usrgrp_mn: Rc<RefCell<UsersGroupsManager>>) -> Self {
      use ContentPage::*;
      let usrgrp_mn_ptr = Rc::into_raw(usrgrp_mn);
      let usrgrp_mn = unsafe { &*usrgrp_mn_ptr };
      let usrgrp_ref = usrgrp_mn.borrow();
      let list_users = usrgrp_ref.list_users();
      let list_groups = usrgrp_ref.list_groups();
      let curr_group = list_groups.first();

      Self {
         usrgrp_mn: usrgrp_mn.clone(),
         curr_usr_is_admin: list_users.first().map(|usr| usr.is_admin()).unwrap_or(false),
         ls_grps: list_groups.iter().map(ToOwned::to_owned).map(|grp| (grp.clone(), button::State::new())).collect(),
         ls_users: list_users.iter().map(ToOwned::to_owned).map(|usr| usr.clone()).collect(),
         selected_grp: curr_group.map(|_| 0),
         content: if let Some(curr_group) = curr_group {
            EditGroup(EditGroupPage::new(curr_group, list_users))
         } else {
            Empty
         },
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: GroupsMsg) {
      use GroupsMsg::*;
      use ContentPage::*;

      let Self {
         usrgrp_mn,
         ls_grps,
         ls_users,
         content,
         ..
      } = self;

      let is_admin = self.curr_usr_is_admin;
      match msg {
         SelecteGroup(idx) => {
            self.selected_grp = Some(idx);
            if let EditGroup(edit_group_page) = content {
               if let Some((group, _)) = ls_grps.get_mut(idx) {
                  edit_group_page.with_grp(group);
               }
            }
         },
         AddClicked => {
            if is_admin {
               self.content = AddGroup(AddGroupPage::new())
            }
         },
         RemoveClicked => {
            if is_admin {
               if let Some(idx) = self.selected_grp {
                  if let Some((group, _)) = ls_grps.get(idx) {
                     match usrgrp_mn.borrow_mut().delete_group(group.name()) {
                        Ok(is_ok) => if is_ok {
                           ls_grps.remove(idx);
                        } else {
                           println!("can not delete group");
                        },
                        Err(err) => eprintln!("{:?}", err)
                     }
                  }
               }
               self.selected_grp = None;
            }
         },
         EditGroupMSG(edit_group_msg) => {
            if let EditGroup(edit_group_page) = content {
               if let Some(idx) = self.selected_grp {
                  if let Some((grp, _)) = ls_grps.get_mut(idx) {
                     use EditGroupMsg::*;
                     match edit_group_msg {
                        // GroupNameSubmitted(grp_name) => {
                        //    println!("Submitted");
                        //    edit_group_page.with_grp(grp);
                        // },
                        OkayClicked(grp_name, ls_members) => {
                           match usrgrp_mn.borrow_mut().change_group_members(grp.name(), ls_members.iter().map(|usr| usr.as_str()).collect()) {
                              Ok(is_ok) => if is_ok {
                                 println!("change group members success")
                              } else {
                                 println!("can not change group members")
                              },
                              Err(err) => eprintln!("{:?}", err)
                           }
                           match usrgrp_mn.borrow_mut().change_group_name(grp.name(), &grp_name) {
                              Ok(is_ok) => if is_ok {
                                 println!("change group name success")
                              } else {
                                 println!("can not change group name")
                              },
                              Err(err) => eprintln!("{:?}", err)
                           }
                           edit_group_page.with_grp(grp);
                        },
                        _ => edit_group_page.update(edit_group_msg)
                     }
                  }
               }
            }
         },
         AddGroupMSG(add_group_msg) => {
            if let AddGroup(add_group_page) = content {
               use AddGroupMsg::*;
               match add_group_msg {
                  CreateClicked(group_name) => match usrgrp_mn.borrow_mut().create_group(group_name.as_str()) {
                     Ok(is_ok) => if is_ok {
                        if let Some(group) = usrgrp_mn.borrow().group_from_name(group_name.as_str()) {
                           ls_grps.push((group.clone(), button::State::new()));
                           self.selected_grp = ls_grps.iter().map(|(grp, _)| grp).position(|grp| grp.gid() == group.gid());
                           self.content = EditGroup(EditGroupPage::new(group, ls_users.iter().map(|usr| usr).collect()));
                        }
                     } else {
                        if let Some(idx) = self.selected_grp {
                           if let Some((grp, _)) = ls_grps.get_mut(idx) {
                              self.content = EditGroup(EditGroupPage::new(grp, ls_users.iter().map(|usr| usr).collect()));
                           }
                        } else {
                           self.content = Empty;
                        }
                     },
                     Err(err) => eprintln!("{:?}", err)
                  },
                  CancelClicked => {
                     if let Some(idx) = self.selected_grp {
                        if let Some((grp, _)) = ls_grps.get_mut(idx) {
                           self.content = EditGroup(EditGroupPage::new(grp, ls_users.iter().map(|usr| usr).collect()));
                        }
                     } else {
                        self.content = Empty;
                     }
                  },
                  _ => add_group_page.update(add_group_msg),
               }
            }
         }
      }
   }

   pub fn view(&mut self) -> Element<GroupsMsg> {
      use GroupsMsg::*;
      use ContentPage::*;

      let Self {
         ls_grps, selected_grp, scroll_grps, add_state, remove_state, content, ..
      } = self;

      let is_admin = self.curr_usr_is_admin;
      let scrollable_group = ls_grps.iter_mut().enumerate().fold(Scrollable::new(scroll_grps).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (group, state))| {
         let mut btn = Button::new(state, Text::new(group.formatted_name())).width(Length::Fill).style(
            if let Some(selected) = *selected_grp {
               if selected == idx {CustomButton::Selected}
               else {CustomButton::Text}
            }
            else {CustomButton::Text}
         );
         if let EditGroup(_) = content {
            btn = btn.on_press(SelecteGroup(idx));
         }
         scrollable.push(btn)
      });
      let mut btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).style(CustomButton::Text);
      let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
      if is_admin {
         btn_add = btn_add.on_press(AddClicked);
      }
      if selected_grp.is_some() && is_admin {
         btn_remove = btn_remove.on_press(RemoveClicked);
      }
      let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
      let group_pane = Container::new(
         Column::new()
         .push(Container::new(Text::new("Groups")).width(Length::Fill).padding(7).style(CustomContainer::Header))
         .push(scrollable_group)
         .push(btn_group)
      ).height(Length::Fill).width(Length::FillPortion(3)).style(CustomContainer::ForegroundWhite);

      let right_sec = match content{
         AddGroup(add_group_page) => add_group_page.view().map(|msg| AddGroupMSG(msg)),
         EditGroup(edit_group_page) => edit_group_page.view().map(|msg| EditGroupMSG(msg)),
         Empty => Container::new(
            Text::new("There is no groups available")
         ).width(Length::FillPortion(7)).height(Length::Fill).center_x().center_y().into()
      };

      Container::new(
         Row::new().width(Length::Fill).spacing(10)
         .push(group_pane)
         .push(right_sec)
      ).width(Length::Fill).height(Length::Fill).into()
   }
}