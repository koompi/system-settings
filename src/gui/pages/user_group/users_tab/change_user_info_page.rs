use iced::{
   text_input, button, pick_list, Button, TextInput, PickList, Text, Container, Length, Column, Row, Align, Space, Element,
};
use crate::gui::styles::{CustomTextInput, CustomButton, CustomSelect};
use crate::gui::addon_widgets::icon_btn;
use iced_custom_widget::{Icon, number_input, NumberInput};

#[derive(Default)]
pub struct ChangeInfoPage {
   pub uid_state: number_input::State,
   pub uid: u16,
   pub groupname_state: text_input::State,
   pub groupname: String,
   pub username_state: text_input::State,
   pub username: String,
   pub fullname_state: text_input::State,
   pub fullname: String,
   pub login_shell_state: pick_list::State<String>,
   pub login_shell: Option<String>,
   pub home_dir_state: text_input::State,
   pub home_dir: String,
   pub is_changed: bool,
   pub btn_browse_home_dir: button::State,
   pub btn_ok_state: button::State,
   pub btn_cancel_state: button::State,
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
   OkayClicked,
}

impl ChangeInfoPage {
   pub fn new() -> Self {
      Self::default()
   }

   pub fn update(&mut self, msg: ChangeInfoMsg) {
      use nfd2::Response;
      use ChangeInfoMsg::*;
      let mut has_changed = true;
      match msg {
         UIDChanged(val) => self.uid = val,
         GroupNameChanged(val) => self.groupname = val,
         FullNameChanged(val) => self.fullname = val,
         UserNameChanged(val) => self.username = val,
         LoginShellChanged(val) => self.login_shell = Some(val),
         HomeDirChanged(val) => self.home_dir = val,
         BrowseClicked => {
            if let Ok(res) = nfd2::open_file_dialog(None, Some(std::path::Path::new(&self.home_dir))) {
               match res {
                  Response::Okay(file_path) => self.home_dir = file_path.into_os_string().into_string().unwrap(),
                  // Response::Cancel => has_changed = false,
                  _ => has_changed = false
               }
            } else {
               has_changed = false
            }
         },
         CancelClicked | OkayClicked => {has_changed = false},
      }
      self.is_changed = has_changed;
   }

   pub fn view(&mut self) -> Element<ChangeInfoMsg> {
      use ChangeInfoMsg::*;
      let Self {
         uid_state, uid, fullname_state, fullname, username_state, username,
         login_shell_state, login_shell, home_dir_state, home_dir, groupname_state, groupname, 
         btn_browse_home_dir, btn_ok_state, btn_cancel_state, ..
      } = self;
      
      let lb_user_id = Text::new("User ID");
      let lb_fullname = Text::new("Full Name:");
      let lb_username = Text::new("User Name:");
      let lb_group_name = Text::new("Group Name:");
      let lb_login_shell = Text::new("Login Shell:");
      let lb_home_dir = Text::new("Home Directory:");
      let lb_sec = Column::new().spacing(20).align_items(Align::End)
         .push(lb_user_id)
         .push(lb_fullname)
         .push(lb_username)
         .push(lb_group_name)
         .push(lb_login_shell)
         .push(lb_home_dir);
      // let ls_acc_types: Vec<String> = Self::ACC_TYPES.iter().map(|acc| acc.to_string()).collect();
      let txt_user_id = NumberInput::new(uid_state, *uid, 2000, UIDChanged).min(1000);
      let txt_fullname = TextInput::new(fullname_state, "", &fullname, FullNameChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let txt_username = TextInput::new(username_state, "", &username, UserNameChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let txt_group_name = TextInput::new(groupname_state, "", &groupname, GroupNameChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let pl_login_shell = PickList::new(login_shell_state, vec![String::from("/bin/bash"), String::from("/bin/zsh"), String::from("/bin/fish")].clone(), login_shell.clone(), LoginShellChanged).style(CustomSelect::Primary);
      let txt_home_dir = TextInput::new(home_dir_state, "", &home_dir, HomeDirChanged).padding(7).width(Length::Units(127)).style(CustomTextInput::Default);
      let btn_browse_home = icon_btn(btn_browse_home_dir, '\u{f05e}', "Browse Directory", None).on_press(BrowseClicked).style(CustomButton::Default);
      let info_sec = Column::new().spacing(5)
         .push(txt_user_id)
         .push(txt_fullname)
         .push(txt_username)
         .push(txt_group_name)
         .push(pl_login_shell)
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(txt_home_dir)
            .push(btn_browse_home)
         );

      let mut btn_okay = icon_btn(btn_ok_state, '\u{f00c}', "Okay", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      if self.is_changed {
         btn_okay = btn_okay.on_press(OkayClicked);
      }

      Container::new(
         Column::new().width(Length::Fill).spacing(10).align_items(Align::Center)
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
            .push(btn_okay)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}