use crate::gui::styles::{CustomButton, CustomCheckbox, CustomTextInput};
use iced::{button, text_input, Align, Button, Checkbox, Column, Container, Element, Image, Length, Row, Space, Text, TextInput};
use users::{get_user_by_uid, os::unix::UserExt, uid_t, User};

#[derive(Default)]
pub struct UserInfoPage {
    pub user: Option<User>,
    pub is_curr_usr: bool,
    pub btn_change_pwd_state: button::State,
    pub allow_usr_admin: bool,
}

#[derive(Debug, Clone)]
pub enum UserInfoMsg {
    ChangePwdClicked,
    AllowUsrAdminToggled(bool),
}

impl UserInfoPage {
    pub fn new(uid: uid_t, allow_usr_admin: bool) -> Self {
        Self {
            user: get_user_by_uid(uid),
            is_curr_usr: true,
            btn_change_pwd_state: button::State::new(),
            allow_usr_admin,
        }
    }

    pub fn with_uid(&mut self, uid: uid_t, is_curr_usr: bool, allow_usr_admin: bool) {
        self.user = get_user_by_uid(uid);
        self.is_curr_usr = is_curr_usr;
        self.allow_usr_admin = allow_usr_admin;
    }

    pub fn update(&mut self, msg: UserInfoMsg) {
        use UserInfoMsg::*;
        match msg {
            ChangePwdClicked => {}
            AllowUsrAdminToggled(is_checked) => self.allow_usr_admin = is_checked,
        }
    }

    pub fn view(&mut self) -> Element<UserInfoMsg> {
        use UserInfoMsg::*;
        let Self {
            user,
            is_curr_usr,
            btn_change_pwd_state,
            allow_usr_admin,
        } = self;

        if let Some(user) = &user {
            let profile = Image::new(user.home_dir().join(".face.icon")).width(Length::Units(75)).height(Length::Units(75));
            let txt_username = Text::new(user.name().to_str().unwrap_or(""));
            let btn_change_pwd = Button::new(btn_change_pwd_state, Text::new(format!("  {}  ", if *is_curr_usr { "Change Password" } else { "Reset Password" })))
                .on_press(ChangePwdClicked)
                .style(CustomButton::Default);
            let chb_allow_usr_admin = Checkbox::new(*allow_usr_admin, "Allow administrator access", AllowUsrAdminToggled).spacing(10).style(CustomCheckbox::Default);
            Container::new(
                Column::new()
                    .push(Row::new().padding(20).spacing(15).align_items(Align::Center).push(profile).push(txt_username).push(Space::with_width(Length::Fill)).push(btn_change_pwd))
                    .push(chb_allow_usr_admin),
            )
            .width(Length::FillPortion(7))
            .height(Length::Fill)
            .into()
        } else {
            Row::new().into()
        }
    }
}
