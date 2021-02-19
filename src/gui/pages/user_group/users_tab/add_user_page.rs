use iced::{
   text_input, button, pick_list, Button, TextInput, PickList, Text, Container, Length, Column, Row, Align, Space, Element,
};
use crate::gui::styles::{CustomTextInput, CustomButton, CustomSelect};
use crate::gui::addon_widgets::icon_btn;
use iced_custom_widget::Icon;

#[derive(Default)]
pub struct AddUserPage {
   pub acc_type_state: pick_list::State<String>,
   pub selected_acc_type: Option<String>,
   pub fullname_state: text_input::State,
   pub fullname_val: String,
   pub username_state: text_input::State,
   pub username_val: String,
   pub pwd_state: text_input::State,
   pub pwd_val: String,
   pub verify_pwd_state: text_input::State,
   pub verify_pwd_val: String,
   pub pwd_hint_state: text_input::State,
   pub pwd_hint_val: String,
   pub btn_create_state: button::State,
   pub btn_cancel_state: button::State,
   pub is_show_pwd: bool,
   pub btn_show_pwd: button::State,
}

#[derive(Debug, Clone)]
pub struct User {
   acc_type: String,
   fullname: String,
   username: String,
   pwd: String,
   pwd_hint: String,
}

#[derive(Debug, Clone)]
pub enum AddUserMsg {
   AccTypeChanged(String),
   FullNameChanged(String),
   UserNameChanged(String),
   PwdChanged(String),
   VerifyPwdChanged(String),
   PwdHintChanged(String),
   ShowPwdToggled,
   CreateClicked(User),
   CancelClicked,
}

impl AddUserPage {
   const ACC_TYPES: [&'static str; 2] = ["Standard", "Administrator"];
   pub fn new() -> Self {
      Self::default()
   }

   pub fn update(&mut self, msg: AddUserMsg) {
      use AddUserMsg::*;

      match msg {
         AccTypeChanged(val) => self.selected_acc_type = Some(val),
         FullNameChanged(val) => self.fullname_val = val,
         UserNameChanged(val) => self.username_val = val,
         PwdChanged(val) => self.pwd_val = val,
         VerifyPwdChanged(val) => self.verify_pwd_val = val,
         PwdHintChanged(val) => self.pwd_hint_val = val,
         ShowPwdToggled => self.is_show_pwd = !self.is_show_pwd,
         CreateClicked(_) | CancelClicked => {},
      }
   }

   pub fn view(&mut self) -> Element<AddUserMsg> {
      use AddUserMsg::*;
      let Self {
         acc_type_state, selected_acc_type, fullname_state, fullname_val, username_state, username_val,
         pwd_state, pwd_val, verify_pwd_state, verify_pwd_val, pwd_hint_state, pwd_hint_val, 
         btn_create_state, btn_cancel_state, btn_show_pwd, ..
      } = self;
      
      let lb_fullname = Text::new("Full Name:");
      let lb_username = Text::new("User Name:");
      let lb_acc_type = Text::new("Account Type:");
      let lb_pwd = Text::new("Password:");
      let lb_verify_pwd = Text::new("Verify Password:");
      let lb_pwd_hint = Text::new("Hint:");
      let lb_sec = Column::new().spacing(20).align_items(Align::End)
         .push(lb_fullname)
         .push(lb_username)
         .push(lb_acc_type)
         .push(lb_pwd)
         .push(lb_verify_pwd)
         .push(lb_pwd_hint);
      let ls_acc_types: Vec<String> = Self::ACC_TYPES.iter().map(|acc| acc.to_string()).collect();
      let txt_fullname = TextInput::new(fullname_state, "Full name", &fullname_val, FullNameChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let txt_username = TextInput::new(username_state, "User name", &username_val, UserNameChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let pl_acc_type = PickList::new(acc_type_state, ls_acc_types.clone(), selected_acc_type.clone(), AccTypeChanged).style(CustomSelect::Primary);
      let mut txt_pwd = TextInput::new(pwd_state, "Required", &pwd_val, PwdChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      if !self.is_show_pwd {
         txt_pwd = txt_pwd.password();
      }
      let btn_show_pwd = Button::new(btn_show_pwd, Icon::new(if self.is_show_pwd {'\u{f09c}'} else {'\u{f023}'}).size(18)).on_press(ShowPwdToggled).style(CustomButton::Hovered);
      let txt_verify = TextInput::new(verify_pwd_state, "Verify", &verify_pwd_val, VerifyPwdChanged).password().padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let txt_hint = TextInput::new(pwd_hint_state, "(Recommended)", &pwd_hint_val, PwdHintChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let info_sec = Column::new().spacing(5)
         .push(txt_fullname)
         .push(txt_username)
         .push(pl_acc_type)
         .push(Row::new().spacing(10).push(txt_pwd).push(btn_show_pwd))
         .push(txt_verify)
         .push(txt_hint);

      let mut btn_create = icon_btn(btn_create_state, '\u{f00c}', "Create", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      
      if let Some(acc_type) = selected_acc_type {
         if !(fullname_val.is_empty() && username_val.is_empty() && pwd_val.is_empty() && pwd_val != verify_pwd_val) {
            let user = User {
               acc_type: acc_type.to_owned(),
               fullname: fullname_val.to_owned(),
               username: username_val.to_owned(),
               pwd: pwd_val.to_owned(),
               pwd_hint: pwd_hint_val.to_owned(),
            };
            btn_create = btn_create.on_press(CreateClicked(user));
         }
      }

      Container::new(
         Column::new().width(Length::Fill).spacing(10)
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(lb_sec)
            .push(info_sec)
         )
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(btn_cancel)
            .push(btn_create)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}