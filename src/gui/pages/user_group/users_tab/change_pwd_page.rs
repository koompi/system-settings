use iced::{
   text_input, button, Button, TextInput, Text, Container, Length, Column, Row, Align, Space, Element,
};
use users::{User, uid_t, get_user_by_uid};
use crate::gui::styles::{CustomTextInput, CustomButton};
use iced_custom_widget::Icon;
use crate::gui::addon_widgets::icon_btn;

#[derive(Debug, Default)]
pub struct ChangePwdPage {
   pub user: Option<User>,
   pub is_curr_usr: bool,
   pub old_pwd_state: text_input::State,
   pub old_pwd_val: String,
   pub new_pwd_state: text_input::State,
   pub new_pwd_val: String,
   pub verify_pwd_state: text_input::State,
   pub verify_pwd_val: String,
   pub pwd_hint_state: text_input::State,
   pub pwd_hint_val: String,
   pub btn_change_state: button::State,
   pub btn_cancel_state: button::State,
   pub is_show_pwd: bool,
   pub btn_show_pwd: button::State,
}

#[derive(Debug, Clone)]
pub enum ChangePwdMsg {
   OldPwdChanged(String),
   NewPwdChanged(String),
   VerifyPwdChanged(String),
   PwdHintChanged(String),
   ShowPwdToggled,
   ChangeClicked,
   CancelClicked,
}

impl ChangePwdPage {
   pub fn new(is_curr_usr: bool, uid: uid_t) -> Self {
      Self {
         user: get_user_by_uid(uid),
         is_curr_usr,
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: ChangePwdMsg) {
      use ChangePwdMsg::*;
      match msg {
         OldPwdChanged(val) => self.old_pwd_val = val,
         NewPwdChanged(val) => self.new_pwd_val = val,
         VerifyPwdChanged(val) => self.verify_pwd_val = val,
         PwdHintChanged(val) => self.pwd_hint_val = val,
         ShowPwdToggled => self.is_show_pwd = !self.is_show_pwd,
         ChangeClicked | CancelClicked => {},
      }
   }

   pub fn view(&mut self) -> Element<ChangePwdMsg> {
      use ChangePwdMsg::*;
      let Self {
         user, is_curr_usr, old_pwd_state, old_pwd_val, new_pwd_state, new_pwd_val, verify_pwd_state, verify_pwd_val,
         pwd_hint_state, pwd_hint_val, btn_change_state, btn_cancel_state, btn_show_pwd, ..
      } = self;

      let lb_old_pwd = Text::new("Old Password:");
      let lb_new_pwd = Text::new("New Password:");
      let lb_verify_pwd = Text::new("Verify Password:");
      let lb_pwd_hint = Text::new("Hint:");
      let lb_sec = Column::new().spacing(20).align_items(Align::End)
         .push(lb_old_pwd)
         .push(lb_new_pwd)
         .push(lb_verify_pwd)
         .push(lb_pwd_hint);
      let txt_old_pwd = TextInput::new(old_pwd_state, "", &old_pwd_val, OldPwdChanged).password().padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let mut txt_new_pwd = TextInput::new(new_pwd_state, "", &new_pwd_val, NewPwdChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      if !self.is_show_pwd {
         txt_new_pwd = txt_new_pwd.password();
      }
      let btn_show_pwd = Button::new(btn_show_pwd, Icon::new(if self.is_show_pwd {'\u{f09c}'} else {'\u{f023}'}).size(18)).on_press(ShowPwdToggled).style(CustomButton::Hovered);
      let txt_verify_pwd = TextInput::new(verify_pwd_state, "", &verify_pwd_val, VerifyPwdChanged).password().padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let txt_pwd_hint = TextInput::new(pwd_hint_state, "(Recommended)", &pwd_hint_val, PwdHintChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let input_sec = Column::new().spacing(5)
         .push(txt_old_pwd)
         .push(Row::new().spacing(10).push(txt_new_pwd).push(btn_show_pwd))
         .push(txt_verify_pwd)
         .push(txt_pwd_hint);

      let mut btn_change = icon_btn(btn_change_state, '\u{f00c}', "Change Password", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
   
      if !(old_pwd_val.is_empty() && new_pwd_val.is_empty() && new_pwd_val != verify_pwd_val) {
         btn_change = btn_change.on_press(ChangeClicked);
      }

      Container::new(
         Column::new().width(Length::Fill).spacing(10)
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(lb_sec)
            .push(input_sec)
         )
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(btn_cancel)
            .push(btn_change)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}