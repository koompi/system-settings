use std::path::PathBuf;
use libkoompi::system_settings::users_groups::User;
use iced::{
   button, Checkbox, Text, Container, Length, Column, Row, Space, Element, Button, Image, Align,
};
use crate::gui::styles::{CustomButton, CustomCheckbox, CustomContainer, FOREGROUND};
use iced_custom_widget::Stack;

#[derive(Debug, Default)]
pub struct UserInfoPage {
   fullname: String,
   profile_path: PathBuf,
   is_admin: bool,
   is_curr_usr: bool,
   allow_usr_admin: bool,
   btn_change_pwd_state: button::State,
   btn_change_info_state: button::State,
   btn_change_groups_state: button::State,
}

#[derive(Debug, Clone)]
pub enum UserInfoMsg {
   ChangePwdClicked,
   ChangeInfoClicked,
   ChangeGroupsClicked,
   AllowUsrAdminToggled(bool),
}

impl UserInfoPage {
   pub fn new(user: &User, is_curr_usr: bool, allow_usr_admin: bool) -> Self {
      Self {
         fullname: user.fullname().to_owned(),
         profile_path: user.profile_path(),
         is_admin: user.is_admin(),
         is_curr_usr,
         allow_usr_admin, 
         ..Self::default()
      }
   }

   pub fn with_user(&mut self, user: &User, is_curr_usr: bool) {
      self.fullname = user.fullname().to_owned();
      self.profile_path = user.profile_path();
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
         fullname, profile_path, btn_change_pwd_state, btn_change_info_state, btn_change_groups_state, ..
      } = self;

      // println!("{:?}", profile_path.display());
      let profile: Element<_> = if profile_path.exists() {
         Image::new(profile_path.to_path_buf()).width(Length::Units(75)).height(Length::Units(75)).into()
      } else {
         Container::new(Row::new()).width(Length::Units(75)).height(Length::Units(75)).style(CustomContainer::Header).into()
      };
      let txt_username = Text::new(fullname.as_str());
      let mut btn_change_info = Button::new(btn_change_info_state, Text::new("  Change Information  ")).style(CustomButton::Default);
      let chb_allow_usr_admin = Checkbox::new(self.is_admin, "Allow administrator access", AllowUsrAdminToggled).spacing(10).style(CustomCheckbox::Default);
      let sec_allow_usr_admin: Element<_> = if self.allow_usr_admin && !self.is_curr_usr {
         chb_allow_usr_admin.into()
      } else {
         Stack::new()
         .push(chb_allow_usr_admin, None)
         .push(Container::new(Row::new()).width(Length::Units(200)).height(Length::Units(25)).style(CustomContainer::Transparent(FOREGROUND)), None).into()
      };

      let mut btn_change_pwd = Button::new(btn_change_pwd_state, Text::new(format!("  {}  ", if self.is_curr_usr {"Change Password"} else {"Reset Password"})))
         .style(CustomButton::Default);
      let mut btn_change_grps = Button::new(btn_change_groups_state, Text::new("  Advanced  ")).style(CustomButton::Default);
      if self.allow_usr_admin || self.is_curr_usr {
         btn_change_info = btn_change_info.on_press(ChangeInfoClicked);
         btn_change_pwd = btn_change_pwd.on_press(ChangePwdClicked);
      }
      if self.allow_usr_admin {
         btn_change_grps = btn_change_grps.on_press(ChangeGroupsClicked);
      }

      Container::new(
         Column::new().spacing(10).padding(10)
         .push(
            Row::new().spacing(15).align_items(Align::Center)
            .push(profile)
            .push(txt_username)
            .push(Space::with_width(Length::Fill))
            .push(btn_change_info)
         )
         .push(sec_allow_usr_admin)
         .push(
            Row::new().align_items(Align::Center)
            .push(btn_change_pwd)
            .push(Space::with_width(Length::Fill))
            .push(btn_change_grps)
         )
      ).width(Length::Fill).height(Length::Fill).into()
   }
}