mod add_user_page;
mod change_pwd_page;
mod user_info_page;

use crate::gui::styles::{CustomButton, CustomContainer};
use iced::{button, scrollable, Align, Button, Column, Container, Element, Image, Length, Row, Scrollable, Text};
use iced_custom_widget::Icon;
use smart_default::SmartDefault;
use std::path::PathBuf;
use users::{all_users, get_current_uid, os::unix::UserExt, uid_t, User};
use {
    add_user_page::{AddUserMsg, AddUserPage},
    change_pwd_page::{ChangePwdMsg, ChangePwdPage},
    user_info_page::{UserInfoMsg, UserInfoPage},
};

#[derive(SmartDefault)]
pub struct UsersTab {
    #[default = 0]
    curr_usr_uid: uid_t,
    pub ls_users: Vec<(User, button::State)>,
    pub selected_user: Option<usize>,
    pub scroll_users: scrollable::State,
    pub add_state: button::State,
    pub remove_state: button::State,

    // dynamic section
    pub user_info_page: UserInfoPage,
    pub change_pwd_page: Option<ChangePwdPage>,
    pub add_user_page: Option<AddUserPage>,
}

#[derive(Debug, Clone)]
pub enum UsersMsg {
    SelectedUsr(usize),
    AddClicked,
    RemoveClicked,
    UserInfoMSG(UserInfoMsg),
    ChangePwdMSG(ChangePwdMsg),
    AddUserMSG(AddUserMsg),
}

impl UsersTab {
    pub fn new() -> Self {
        let curr_usr_uid = get_current_uid();
        let mut ls_users: Vec<(User, button::State)> = unsafe { all_users() }.map(|usr| (usr, button::State::new())).collect();
        if let Some(idx) = ls_users.iter().position(|(usr, _)| usr.uid() == curr_usr_uid) {
            ls_users.swap(0, idx);
        }
        Self {
            curr_usr_uid,
            ls_users,
            selected_user: Some(0),
            user_info_page: UserInfoPage::new(curr_usr_uid, true),
            ..Self::default()
        }
    }

    pub fn update(&mut self, msg: UsersMsg) {
        use UsersMsg::*;
        let Self {
            ls_users,
            user_info_page,
            add_user_page,
            change_pwd_page,
            ..
        } = self;

        match msg {
            SelectedUsr(idx) => {
                self.selected_user = Some(idx);
                if let Some((user, _)) = ls_users.get(idx) {
                    user_info_page.with_uid(user.uid(), self.curr_usr_uid == user.uid(), true);
                }
            }
            AddClicked => self.add_user_page = Some(AddUserPage::new()),
            RemoveClicked => {
                if let Some(selected) = self.selected_user {
                    ls_users.remove(selected);
                }
                self.selected_user = None;
            }
            UserInfoMSG(UserInfoMsg::ChangePwdClicked) => {
                self.add_user_page = None;
                if let Some(idx) = self.selected_user {
                    if let Some((user, _)) = ls_users.get(idx) {
                        self.change_pwd_page = Some(ChangePwdPage::new(self.curr_usr_uid == user.uid(), user.uid()));
                    }
                }
            }
            UserInfoMSG(usr_info_msg) => user_info_page.update(usr_info_msg),
            ChangePwdMSG(ChangePwdMsg::ChangeClicked) => self.change_pwd_page = None,
            ChangePwdMSG(ChangePwdMsg::CancelClicked) => self.change_pwd_page = None,
            ChangePwdMSG(change_pwd_msg) => {
                if let Some(change_pwd_page) = change_pwd_page {
                    change_pwd_page.update(change_pwd_msg);
                }
            }
            AddUserMSG(AddUserMsg::CreateClicked(user)) => {
                // self.ls_users.push((user, button::State::new()))
                self.add_user_page = None;
            }
            AddUserMSG(AddUserMsg::CancelClicked) => self.add_user_page = None,
            AddUserMSG(add_user_msg) => {
                if let Some(add_user_page) = add_user_page {
                    add_user_page.update(add_user_msg);
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<UsersMsg> {
        use UsersMsg::*;
        let Self {
            ls_users,
            selected_user,
            scroll_users,
            add_state,
            remove_state,
            user_info_page,
            change_pwd_page,
            add_user_page,
            ..
        } = self;

        let scrollable_users = ls_users
            .iter_mut()
            .enumerate()
            .fold(Scrollable::new(scroll_users).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (user, state))| {
                // let mut profile_path = user.home_dir().join(".face");
                // if !profile_path.exists() {
                //    profile_path = ;
                // }
                let content = Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(
                        Container::new(Image::new(PathBuf::from("/usr/share/sddm/faces/root.face.icon")))
                            .width(Length::Units(30))
                            .height(Length::Units(30))
                            .style(CustomContainer::Header),
                    )
                    .push(Text::new(user.name().to_str().unwrap_or("")));
                let btn = Button::new(state, content).width(Length::Fill).on_press(SelectedUsr(idx)).style(if let Some(selected) = *selected_user {
                    if selected == idx {
                        CustomButton::Selected
                    } else {
                        CustomButton::Text
                    }
                } else {
                    CustomButton::Text
                });
                scrollable.push(btn)
            });
        let btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(AddClicked).style(CustomButton::Text);
        let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
        if selected_user.is_some() {
            btn_remove = btn_remove.on_press(RemoveClicked);
        }
        let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
        let group_pane = Container::new(Column::new().push(Container::new(Text::new("Users")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_users).push(btn_group))
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .style(CustomContainer::ForegroundWhite);

        let right_sec = if let Some(add_user_page) = add_user_page {
            add_user_page.view().map(|msg| AddUserMSG(msg))
        } else if let Some(change_pwd_page) = change_pwd_page {
            change_pwd_page.view().map(|msg| ChangePwdMSG(msg))
        } else {
            user_info_page.view().map(|msg| UserInfoMSG(msg))
        };

        Container::new(Row::new().width(Length::Fill).spacing(10).push(group_pane).push(right_sec)).width(Length::Fill).height(Length::Fill).into()
    }
}
