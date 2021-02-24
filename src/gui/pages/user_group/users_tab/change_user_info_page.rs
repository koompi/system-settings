use iced::{
   text_input, button, pick_list, TextInput, PickList, Text, Container, Length, Column, Row, Align, Space, Element,
};
use libkoompi::{
   system_settings::users_groups::{User, MAX_UID, MIN_UID}, helpers::to_account_name,
};
use crate::gui::styles::{CustomTextInput, CustomButton, CustomSelect, CustomContainer, FOREGROUND};
use crate::gui::addon_widgets::icon_btn;
use iced_custom_widget::{number_input, NumberInput, Stack};

#[derive(Debug, Default)]
pub struct ChangeInfoPage {
   login_shells: Vec<String>,
   is_curr_usr: bool,
   uid_state: number_input::State,
   uid: u16,
   groupname_state: text_input::State,
   groupname: String,
   username_state: text_input::State,
   username: String,
   fullname_state: text_input::State,
   fullname: String,
   login_shell_state: pick_list::State<String>,
   login_shell: Option<String>,
   home_dir_state: text_input::State,
   home_dir: String,
   is_changed: bool,
   btn_browse_home_dir: button::State,
   btn_ok_state: button::State,
   btn_cancel_state: button::State,
}

#[derive(Debug, Clone)]
pub enum ChangeInfoMsg {
   UIDChanged(u16),
   GroupNameChanged(String),
   UserNameChanged(String),
   FullNameChanged(String),
   LoginShellChanged(String),
   HomeDirChanged(String),
   BrowseClicked,
   CancelClicked,
   OkayClicked(UserReq),
}

#[derive(Debug, Clone)]
pub struct UserReq {
   pub uid: u16,
   pub gname: String,
   pub fullname: String,
   pub username: String,
   pub login_shell: String,
   pub home_dir: String,
}

impl ChangeInfoPage {
   pub fn new(user: &User, is_curr_usr: bool, groupname: Option<&String>, login_shells: Vec<String>) -> Self {
      Self {
         login_shells, is_curr_usr, uid: user.uid(),
         groupname: groupname.unwrap_or(&String::default()).to_string(),
         username: user.username().clone(),
         fullname: user.fullname().clone(),
         login_shell: Some(user.login_shell().to_owned().into_os_string().into_string().unwrap()),
         home_dir: user.home_dir().to_owned().into_os_string().into_string().unwrap(),
         uid_state: Default::default(),
         groupname_state: Default::default(),
         username_state: Default::default(),
         fullname_state: Default::default(),
         login_shell_state: Default::default(),
         home_dir_state: Default::default(),
         is_changed: false,
         btn_browse_home_dir: Default::default(),
         btn_ok_state: Default::default(),
         btn_cancel_state: Default::default(),
      }
   }

   pub fn update(&mut self, msg: ChangeInfoMsg) {
      use nfd2::Response;
      use ChangeInfoMsg::*;
      let mut has_changed = true;

      match msg {
         UIDChanged(val) => self.uid = val,
         GroupNameChanged(val) => self.groupname = to_account_name(&val),
         FullNameChanged(val) => self.fullname = val,
         UserNameChanged(val) => self.username = to_account_name(&val),
         LoginShellChanged(val) => self.login_shell = Some(val),
         HomeDirChanged(val) => self.home_dir = val,
         BrowseClicked => {
            if let Ok(res) = nfd2::open_file_dialog(None, Some(std::path::Path::new(&self.home_dir))) {
               match res {
                  Response::Okay(file_path) => self.home_dir = file_path.into_os_string().into_string().unwrap(),
                  _ => has_changed = false
               }
            } else {
               has_changed = false
            }
         },
         CancelClicked | OkayClicked(_) => has_changed = false,
      }
      self.is_changed = has_changed;
   }

   pub fn view(&mut self) -> Element<ChangeInfoMsg> {
      use ChangeInfoMsg::*;
      let Self {
         uid_state, uid, fullname_state, fullname, username_state, username, login_shell_state, login_shell, 
         home_dir_state, home_dir, groupname_state, groupname, btn_browse_home_dir, btn_ok_state, btn_cancel_state, login_shells, ..
      } = self;
      
      let lb_user_id = Text::new("User ID");
      let lb_fullname = Text::new("Full Name:");
      let lb_username = Text::new("User Name:");
      let lb_group_name = Text::new("Group Name:");
      let lb_login_shell = Text::new("Login Shell:");
      let lb_home_dir = Text::new("Home Directory:");
      let lb_sec = Column::new().spacing(20).push(lb_user_id).push(lb_fullname).push(lb_username)
         .push(lb_group_name).push(lb_login_shell).push(lb_home_dir);

      let txt_user_id = NumberInput::new(uid_state, *uid, MAX_UID, UIDChanged).min(MIN_UID);
      let txt_fullname = TextInput::new(fullname_state, "", &fullname, FullNameChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let txt_username = TextInput::new(username_state, "", &username, UserNameChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let txt_group_name = TextInput::new(groupname_state, "", &groupname, GroupNameChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let pl_login_shell = PickList::new(login_shell_state, login_shells.clone(), login_shell.clone(), LoginShellChanged).style(CustomSelect::Primary);
      let txt_home_dir = TextInput::new(home_dir_state, "", &home_dir, HomeDirChanged).padding(7).style(CustomTextInput::Default);
      let mut btn_browse_home = icon_btn(btn_browse_home_dir, '\u{f07b}', "Browse", None).style(CustomButton::Default);
      if !self.is_curr_usr {
         btn_browse_home = btn_browse_home.on_press(BrowseClicked);
      }
      let sec_home_dir = Row::new().spacing(10).align_items(Align::Center).push(txt_home_dir).push(btn_browse_home);
      
      let (sec_uid, sec_usrname, sec_home_dir): (Element<_>, Element<_>, Element<_>) = if !self.is_curr_usr {
         (txt_user_id.into(), txt_username.into(), sec_home_dir.into())
      } else {
         let txt_uid = Stack::new().push(txt_user_id, None)
            .push(Container::new(Row::new()).width(Length::Units(120)).height(Length::Units(25)).style(CustomContainer::Transparent(FOREGROUND)), None);
         let txt_usrname = Stack::new().push(txt_username, None)
            .push(Container::new(Row::new()).width(Length::Fill).height(Length::Units(27)).style(CustomContainer::Transparent(FOREGROUND)), None);
         let sec_home_dir = Stack::new().push(sec_home_dir, None)
            .push(Container::new(Row::new()).width(Length::Fill).height(Length::Units(27)).style(CustomContainer::Transparent(FOREGROUND)), None);
         (txt_uid.into(), txt_usrname.into(), sec_home_dir.into())
      };

      let info_sec = Column::new().spacing(7).push(sec_uid).push(txt_fullname).push(sec_usrname).push(txt_group_name)
         .push(pl_login_shell).push(sec_home_dir);

      let mut btn_okay = icon_btn(btn_ok_state, '\u{f00c}', "Okay", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      if let Some(login_shell) = login_shell {
         if self.is_changed && !(fullname.is_empty() || username.is_empty() || groupname.is_empty() || home_dir.is_empty()) {
            let user = UserReq {
               uid: *uid,
               gname: groupname.to_owned(),
               fullname: fullname.to_owned(),
               username: username.to_owned(),
               login_shell: login_shell.to_owned(),
               home_dir: home_dir.to_owned(),
            };
            btn_okay = btn_okay.on_press(OkayClicked(user));
         }
      }

      Container::new(
         Column::new().width(Length::Fill).padding(20).spacing(10).align_items(Align::Center)
         .push(
            Row::new().spacing(10).width(Length::Units(400)).align_items(Align::Center)
            .push(lb_sec)
            .push(info_sec)
         )
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(btn_cancel)
            .push(btn_okay)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}