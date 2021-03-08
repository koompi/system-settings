use iced::{
   text_input, button, Button, TextInput, Text, Container, Length, Column, Row, Align, Space, Element,
};
use crate::gui::styles::{CustomTextInput, CustomButton};
use iced_custom_widget::Icon;
use crate::gui::addon_widgets::icon_btn;

#[derive(Debug, Default)]
pub struct ChangePwdPage {
   is_curr_usr: bool,
   old_pwd_state: text_input::State,
   old_pwd_val: String,
   new_pwd_state: text_input::State,
   new_pwd_val: String,
   verify_pwd_state: text_input::State,
   verify_pwd_val: String,
   btn_change_state: button::State,
   btn_cancel_state: button::State,
   is_show_pwd: bool,
   btn_show_pwd: button::State,
}

#[derive(Debug, Clone)]
pub enum ChangePwdMsg {
   OldPwdChanged(String),
   NewPwdChanged(String),
   VerifyPwdChanged(String),
   ShowPwdToggled,
   ChangeClicked(String, String, String),
   CancelClicked,
}

impl ChangePwdPage {
   pub fn new(is_curr_usr: bool) -> Self {
      Self {
         is_curr_usr,
         old_pwd_state: Default::default(),
         old_pwd_val: Default::default(),
         new_pwd_state: Default::default(),
         new_pwd_val: Default::default(),
         verify_pwd_state: Default::default(),
         verify_pwd_val: Default::default(),
         btn_change_state: Default::default(),
         btn_cancel_state: Default::default(),
         is_show_pwd: false,
         btn_show_pwd: Default::default(),
      }
   }

   pub fn update(&mut self, msg: ChangePwdMsg) {
      use ChangePwdMsg::*;
      match msg {
         OldPwdChanged(val) => self.old_pwd_val = val.trim().to_string(),
         NewPwdChanged(val) => self.new_pwd_val = val.trim().to_string(),
         VerifyPwdChanged(val) => self.verify_pwd_val = val.trim().to_string(),
         ShowPwdToggled => self.is_show_pwd = !self.is_show_pwd,
         CancelClicked | ChangeClicked(..) => {},
      }
   }

   pub fn view(&mut self) -> Element<ChangePwdMsg> {
      use ChangePwdMsg::*;
      let Self {
         old_pwd_state, old_pwd_val, new_pwd_state, new_pwd_val, verify_pwd_state, verify_pwd_val,
         btn_change_state, btn_cancel_state, btn_show_pwd, ..
      } = self;

      let lb_old_pwd = Text::new("Old Password:");
      let lb_new_pwd = Text::new("New Password:");
      let lb_verify_pwd = Text::new("Verify Password:");
      let mut lb_sec = Column::new().spacing(20);

      let txt_old_pwd = TextInput::new(old_pwd_state, "", &old_pwd_val, OldPwdChanged).password().padding(7).width(Length::Units(227)).style(CustomTextInput::Default);
      let mut txt_new_pwd = TextInput::new(new_pwd_state, "", &new_pwd_val, NewPwdChanged).padding(7).width(Length::Units(200)).style(CustomTextInput::Default);
      if !self.is_show_pwd {
         txt_new_pwd = txt_new_pwd.password();
      }
      let btn_show_pwd = Button::new(btn_show_pwd, Icon::new(if self.is_show_pwd {'\u{f06e}'} else {'\u{f070}'})).on_press(ShowPwdToggled).style(CustomButton::Text);
      let txt_verify_pwd = TextInput::new(verify_pwd_state, "", &verify_pwd_val, VerifyPwdChanged).password().padding(7).width(Length::Units(227)).style(CustomTextInput::Default);
      let mut input_sec = Column::new().spacing(7);
      if self.is_curr_usr {
         lb_sec = lb_sec.push(lb_old_pwd); 
         input_sec = input_sec.push(txt_old_pwd);
      }
      lb_sec = lb_sec.push(lb_new_pwd).push(lb_verify_pwd);
      input_sec = input_sec
         .push(Row::new().spacing(5).align_items(Align::Center).push(txt_new_pwd).push(btn_show_pwd))
         .push(txt_verify_pwd);

      let mut btn_change = icon_btn(btn_change_state, '\u{f00c}', "Okay", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
   
      if (!self.is_curr_usr || !old_pwd_val.is_empty()) && (!new_pwd_val.is_empty() && new_pwd_val.as_str().eq(verify_pwd_val.as_str()))  {
         btn_change = btn_change.on_press(ChangeClicked(old_pwd_val.clone(), new_pwd_val.clone(), verify_pwd_val.clone()));
      }

      Container::new(
         Column::new().width(Length::Fill).spacing(10).align_items(Align::Center)
         .push(
            Row::new().padding(10).spacing(10).align_items(Align::Center)
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
      ).width(Length::Fill).height(Length::Fill).into()
   }
}