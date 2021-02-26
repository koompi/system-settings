use iced::{
   text_input, button, pick_list, Button, TextInput, PickList, Text, Container, Length, Column, Row, Align, Space, Element,
};
use libkoompi::{
   system_settings::users_groups::AccountType, helpers::to_account_name
};
use crate::gui::styles::{CustomTextInput, CustomButton, CustomSelect};
use crate::gui::addon_widgets::icon_btn;
use iced_custom_widget::Icon;

#[derive(Debug, Clone, Default)]
pub struct AddUserPage {
   acc_type_state: pick_list::State<AccountType>,
   selected_acc_type: Option<AccountType>,
   fullname_state: text_input::State,
   fullname_val: String,
   username_state: text_input::State,
   username_val: String,
   pwd_state: text_input::State,
   pwd_val: String,
   verify_pwd_state: text_input::State,
   verify_pwd_val: String,
   is_show_pwd: bool,
   is_usrname_directly_changed: bool,
   btn_create_state: button::State,
   btn_cancel_state: button::State,
   btn_show_pwd: button::State,
}

#[derive(Debug, Clone)]
pub struct UserReq {
   pub acc_type: AccountType,
   pub fullname: String,
   pub username: String,
   pub pwd: String,
   pub pwd_verify: String,
}

#[derive(Debug, Clone)]
pub enum AddUserMsg {
   AccTypeChanged(AccountType),
   FullNameChanged(String),
   UserNameChanged(String),
   PwdChanged(String),
   VerifyPwdChanged(String),
   ShowPwdToggled,
   CreateClicked(UserReq),
   CancelClicked,
}

impl AddUserPage {
   pub fn new() -> Self {
      Self {
         selected_acc_type: Some(AccountType::default()),
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: AddUserMsg) {
      use AddUserMsg::*;

      match msg {
         AccTypeChanged(val) => self.selected_acc_type = Some(val),
         FullNameChanged(val) => {
            self.fullname_val = val;
            if !self.is_usrname_directly_changed {
               self.username_val = to_account_name(&self.fullname_val);
            }
         },
         UserNameChanged(val) => {
            self.is_usrname_directly_changed = true;
            self.username_val = to_account_name(&val); 
         },
         PwdChanged(val) => self.pwd_val = val,
         VerifyPwdChanged(val) => self.verify_pwd_val = val,
         ShowPwdToggled => self.is_show_pwd = !self.is_show_pwd,
         CreateClicked(_) | CancelClicked => {},
      }
   }

   pub fn view(&mut self) -> Element<AddUserMsg> {
      use AddUserMsg::*;
      let Self {
         acc_type_state, selected_acc_type, fullname_state, fullname_val, username_state, username_val,
         pwd_state, pwd_val, verify_pwd_state, verify_pwd_val,
         btn_create_state, btn_cancel_state, btn_show_pwd, ..
      } = self;
      
      let lb_fullname = Text::new("Full Name:");
      let lb_username = Text::new("User Name:");
      let lb_acc_type = Text::new("Account Type:");
      let lb_pwd = Text::new("Password:");
      let lb_verify_pwd = Text::new("Verify Password:");
      let lb_sec = Column::new().spacing(20)
         .push(lb_fullname)
         .push(lb_username)
         .push(lb_acc_type)
         .push(lb_pwd)
         .push(lb_verify_pwd);
         
      let txt_fullname = TextInput::new(fullname_state, "Full name", &fullname_val, FullNameChanged).padding(7).width(Length::Units(227)).style(CustomTextInput::Default);
      let txt_username = TextInput::new(username_state, "User name", &username_val, UserNameChanged).padding(7).width(Length::Units(227)).style(CustomTextInput::Default);
      let pl_acc_type = PickList::new(acc_type_state, &AccountType::ALL[..], selected_acc_type.clone(), AccTypeChanged).style(CustomSelect::Primary);
      let mut txt_pwd = TextInput::new(pwd_state, "Required", &pwd_val, PwdChanged).padding(7).width(Length::Units(200)).style(CustomTextInput::Default);
      if !self.is_show_pwd {
         txt_pwd = txt_pwd.password();
      }
      let btn_show_pwd = Button::new(btn_show_pwd, Icon::new(if self.is_show_pwd {'\u{f06e}'} else {'\u{f070}'})).on_press(ShowPwdToggled).style(CustomButton::Text);
      let txt_verify = TextInput::new(verify_pwd_state, "Verify", &verify_pwd_val, VerifyPwdChanged).password().padding(7).width(Length::Units(227)).style(CustomTextInput::Default);
      let info_sec = Column::new().spacing(7)
         .push(txt_fullname)
         .push(txt_username)
         .push(pl_acc_type)
         .push(Row::new().spacing(5).align_items(Align::Center).push(txt_pwd).push(btn_show_pwd))
         .push(txt_verify);

      let mut btn_create = icon_btn(btn_create_state, '\u{f00c}', "Create", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      
      if let Some(acc_type) = selected_acc_type {
         if !(fullname_val.is_empty() || username_val.is_empty() || pwd_val.is_empty() || pwd_val.as_str().ne(verify_pwd_val.as_str())) {
            let user = UserReq {
               acc_type: *acc_type,
               fullname: fullname_val.to_owned(),
               username: username_val.to_owned(),
               pwd: pwd_val.to_owned(),
               pwd_verify: verify_pwd_val.to_owned(),
            };
            btn_create = btn_create.on_press(CreateClicked(user));
         }
      }

      Container::new(
         Column::new().width(Length::Fill).spacing(10).align_items(Align::Center)
         .push(
            Row::new().padding(10).spacing(10).align_items(Align::Center)
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
      ).width(Length::Fill).height(Length::Fill).into()
   }
}