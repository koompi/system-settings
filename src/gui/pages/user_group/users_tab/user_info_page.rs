use std::path::PathBuf;
use libkoompi::system_settings::users_groups::User;
use iced::{
   button, Checkbox, Text, Container, Length, Column, Row, Space, Element, Button, Image, Align,
};
use crate::gui::styles::{CustomButton, CustomCheckbox, CustomContainer, FOREGROUND};
use iced_custom_widget::{Stack};

#[derive(Debug, Default)]
pub struct UserInfoPage {
   fullname: String,
   profile_path: PathBuf,
   is_admin: bool,
   is_curr_usr: bool,
   allow_usr_admin: bool,
   btn_change_pwd_state: button::State,
   btn_change_info_state: button::State,
}

#[derive(Debug, Clone)]
pub enum UserInfoMsg {
   ChangePwdClicked,
   ChangeInfoClicked,
   AllowUsrAdminToggled(bool),
}

impl UserInfoPage {
   pub fn new(user: &User, is_curr_usr: bool, allow_usr_admin: bool) -> Self {
      Self {
         fullname: user.fullname().to_owned(),
         profile_path: user.home_dir().join(".face"),
         is_admin: user.is_admin(),
         is_curr_usr,
         allow_usr_admin, 
         btn_change_info_state: Default::default(),
         btn_change_pwd_state: Default::default(),
      }
   }

   pub fn with_user(&mut self, user: &User, is_curr_usr: bool) {
      self.fullname = user.fullname().to_owned();
      self.profile_path = user.home_dir().join(".face");
      self.is_admin = user.is_admin();
      self.is_curr_usr = is_curr_usr;
   }

   // pub fn update(&mut self, msg: UserInfoMsg) {
   //    use UserInfoMsg::*;
   //    match msg {
   //       ChangePwdClicked | ChangeInfoClicked | AllowUsrAdminToggled(_) => {},
   //    }
   // }

   pub fn view(&mut self) -> Element<UserInfoMsg> {
      use UserInfoMsg::*;
      let Self {
         fullname, profile_path, btn_change_pwd_state, btn_change_info_state, ..
      } = self;

      let profile = Image::new(profile_path.to_path_buf()).width(Length::Units(75)).height(Length::Units(75));
      let txt_username = Text::new(fullname.as_str());
      let btn_change_pwd = Button::new(btn_change_pwd_state, Text::new(format!("  {}  ", if self.is_curr_usr {"Change Password"} else {"Reset Password"})))
         .on_press(ChangePwdClicked).style(CustomButton::Default);

      let chb_allow_usr_admin: Element<_> = if self.allow_usr_admin && !self.is_curr_usr {
         Checkbox::new(self.is_admin, "Allow administrator access", AllowUsrAdminToggled).spacing(10).style(CustomCheckbox::Default).into()
      } else {
         Stack::new()
         .push(Checkbox::new(self.is_admin, "Allow administrator access", AllowUsrAdminToggled).spacing(10).style(CustomCheckbox::Default), None)
         .push( 
            Container::new(Row::new()).width(Length::Units(200)).height(Length::Units(25)).style(CustomContainer::Transparent(FOREGROUND)),
            None
         ).into()
        
      };
      let btn_change_info = Button::new(btn_change_info_state, Text::new("  Change Information  ")).on_press(ChangeInfoClicked).style(CustomButton::Default);

      Container::new(
         Column::new().padding(20)
         .push(
            Row::new().spacing(15).align_items(Align::Center)
            .push(profile)
            .push(txt_username)
            .push(Space::with_width(Length::Fill))
            .push(btn_change_pwd)
         )
         .push(
            Row::new().align_items(Align::Center)
            .push(chb_allow_usr_admin)
            .push(Space::with_width(Length::Fill))
            .push(btn_change_info)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}