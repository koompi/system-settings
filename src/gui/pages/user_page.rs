/// This file is used as part of system setting.
/// it has some improvement to do is
/// link change the selection of a user in the list when a new user will be added.
/// initialize the AccountView with the state of the current user.
/// this file has four main structs which repsent as the user interface backend.
/// 1. UserPage
/// 2 AccountItem which stands for each of users in the list.
/// 3 AccountView which stands for each view that corresponds to the each user when selecting.
/// 4 UserForm which is used to present as field to capture the data and put them to the list of user
const FONT_SIZE: u16 = 16;
use super::super::styles::{CustomButton, CustomContainer};
use iced::{
    button, scrollable, text_input, Align, Button, Checkbox, Column, Container, Element,
    HorizontalAlignment, Length, Row, Scrollable, Svg, Text, TextInput,
};
#[derive(Default, Debug, Clone)]
pub struct UserPage {
    unlock: button::State,
    is_unlock: bool,
    is_added: bool,
    current_acc: AccountItem,
    list_acc: Vec<(AccountItem, button::State)>,
    list_view: AccountView,
    user_info: UserForm,
    list_scroll: scrollable::State,
    add_btn: button::State,
    remove_btn: button::State,
    current_index: usize,
}
#[derive(Debug, Clone)]
pub enum UserPageMsg {
    Unlocked(bool),
    AccountItemMsg(AccountItemMsg),
    AccountViewMsg(AccountViewMsg),
    UserInfomsg(UserInfomsg),
    AccountSwitch(usize),
    UserAdded(bool),
    UserRemove(usize),
}
impl UserPage {
    pub fn new() -> Self {
        Self {
            list_acc: vec![
                (
                    Self::string_convert("sna", "online-user.svg", true),
                    button::State::new(),
                ),
                (
                    Self::string_convert("rotha", "online-user.svg", false),
                    button::State::new(),
                ),
                (
                    Self::string_convert("vannak", "kuser.svg", true),
                    button::State::new(),
                ),
            ],
            user_info: UserForm::new(),
            list_view: AccountView::new(),
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: UserPageMsg) {
        use UserPageMsg::*;
        match msg {
            Unlocked(value) => {
                self.is_unlock = value;
            }
            AccountItemMsg(msg) => self.current_acc.update(msg),
            UserAdded(value) => self.is_added = value,
            UserPageMsg::UserRemove(idx) => {
                self.current_index = idx;
                if !self.list_acc.is_empty() {
                    if idx.lt(&self.list_acc.len()) & !idx.eq(&0) {
                        self.list_acc.remove(idx);
                        let data = self.list_acc.get(idx - 1).unwrap();
                        self.list_view.set_props(&data.0);
                    }
                } else {
                    {}
                }
            }
            UserPageMsg::AccountSwitch(value) => {
                let data = self.list_acc.get(value).unwrap();
                self.current_index = value;
                self.list_view.set_props(&data.0);
                self.is_added = false;
            }
            UserPageMsg::AccountViewMsg(msg) => {
                self.list_view.update(msg);
            }
            UserPageMsg::UserInfomsg(msg) => {
                let data = self.user_info.update(msg);
                if data.user_added {
                    self.list_acc.push((
                        Self::string_convert(data.username.as_str(), "kuser.svg", data.type_val),
                        button::State::new(),
                    ));
                    data.user_added = false;
                    self.is_added = false;
                    let lenght = self.list_acc.len();
                    self.list_view
                        .set_props(&self.list_acc.get(lenght - 1).unwrap().0);
                    data.username = String::default();
                    data.fullname = String::default();
                    data.password = String::default();
                    data.confirm = String::default();
                } else if data.is_back {
                    self.is_added = false;
                    data.is_back = false;
                } else {
                    {}
                }
            }
        }
    }
    fn string_convert(name: &str, icon: &str, acc_type: bool) -> AccountItem {
        AccountItem::new().set_props(name.to_string(), icon.to_string(), acc_type)
    }
    pub fn view(&mut self) -> Element<UserPageMsg> {
        let UserPage {
            unlock,
            is_added,
            list_acc,
            list_view,
            user_info,
            list_scroll,
            add_btn,
            remove_btn,
            current_index,
            ..
        } = self;
        let list_account = list_acc.iter_mut().enumerate().fold(
            Column::new().width(Length::Fill).spacing(10),
            |column, (index, (child, state))| {
                let btn = Button::new(
                    state,
                    child
                        .view()
                        .map(move |msg| UserPageMsg::AccountItemMsg(msg)),
                )
                .on_press(UserPageMsg::AccountSwitch(index))
                .style(if *current_index == index {
                    CustomButton::SelectedSidebar
                } else {
                    CustomButton::Sidebar
                });
                column.push(btn)
            },
        );

        let scroll_content = Scrollable::new(list_scroll)
            .push(list_account)
            .scroller_width(2);
        let group_list = Column::new()
            .align_items(Align::Center)
            .spacing(4)
            .push(Container::new(Text::new("Your Account:").size(20)).width(Length::Fill))
            .push(scroll_content);
        let list_container = Container::new(group_list)
            .padding(4)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(CustomContainer::ForegroundWhite);
        let list_control_container = Column::new()
            .width(Length::FillPortion(3))
            .push(list_container)
            .spacing(4)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .push(
                        Button::new(
                            add_btn,
                            Text::new("Add New User")
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .style(CustomButton::Apply)
                        .width(Length::Units(100))
                        .on_press(UserPageMsg::UserAdded(true))
                        .padding(10),
                    )
                    .spacing(10)
                    .push(
                        Button::new(
                            remove_btn,
                            Text::new("Remove User")
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .style(CustomButton::Delete)
                        .width(Length::Units(100))
                        .padding(10)
                        .on_press(UserPageMsg::UserRemove(*current_index)),
                    ),
            );
        let account_veiw = Column::new()
            .align_items(Align::Center)
            .push(
                list_view
                    .view()
                    .map(move |msg| UserPageMsg::AccountViewMsg(msg)),
            )
            .width(Length::FillPortion(7))
            .height(Length::Fill);
        let user_view = if *is_added {
            user_info
                .view()
                .map(move |msg| UserPageMsg::UserInfomsg(msg))
        } else {
            account_veiw.into()
        };
        let banner = Column::new().align_items(Align::Center).push(
            Row::new()
                .width(Length::Fill)
                .align_items(Align::Center)
                .push(
                    Column::new()
                        .align_items(Align::End)
                        .width(Length::Fill)
                        .push(Text::new("Some settings must be unlocked before changing.")),
                )
                .spacing(10)
                .push(
                    Button::new(
                        unlock,
                        Text::new("Unlock...").horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .padding(10)
                    .style(CustomButton::Cancel)
                    .width(Length::Units(100))
                    .on_press(UserPageMsg::Unlocked(true)),
                ),
        );
        let main_layout = Row::new().push(list_control_container).push(user_view);
        let main_column: Element<_> = Column::new().push(banner).push(main_layout).into();
        Container::new(main_column.explain(iced::Color::BLACK))
            .style(CustomContainer::ForegroundGray)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

#[derive(Default, Debug, Clone)]
pub struct AccountItem {
    icon: String,
    name: String,
    username: String,
    acc_type: bool,
}
#[derive(Debug, Clone)]
pub enum AccountItemMsg {}
impl AccountItem {
    fn new() -> Self {
        Self {
            icon: "setting.svg".to_string(),
            name: "Sna".to_string(),
            username: "Sna".to_string(),
            acc_type: false,
        }
    }
    fn set_props(mut self, name: String, icon: String, acc_type: bool) -> Self {
        self.name = name;
        self.icon = icon;
        self.acc_type = acc_type;
        self
    }
    fn update(&mut self, msg: AccountItemMsg) {
        match msg {}
    }
    fn view(&mut self) -> Element<AccountItemMsg> {
        let AccountItem {
            icon,
            name,
            acc_type,
            ..
        } = self;
        Row::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Column::new().push(
                    Svg::from_path(format!(
                        "{}/assets/images/{}",
                        env!("CARGO_MANIFEST_DIR"),
                        icon
                    ))
                    .width(Length::Units(48))
                    .height(Length::Units(48)),
                ),
            )
            .push(
                Column::new()
                    .width(Length::Fill)
                    .push(Text::new(name.as_str()).size(18))
                    .push(Text::new(if *acc_type { "admin" } else { "user" })),
            )
            .into()
    }
}
#[derive(Debug, Clone)]
pub enum AccountViewMsg {
    AdminChanged(bool),
    NameChanged(String),
    AutoLoginChanged(bool),
    PasswordChanged,
    ActivityChagned,
    AvatarChanged,
}
#[derive(Default, Debug, Clone)]
pub struct AccountView {
    avatar: String,
    name: text_input::State,
    name_value: String,
    auto_login: bool,
    is_admin: bool,
    activity: button::State,
    password: button::State,
    avatar_btn: button::State,
}
impl AccountView {
    pub fn new() -> Self {
        Self {
            avatar: "kuser.svg".to_string(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, msg: AccountViewMsg) -> &mut Self {
        match msg {
            AccountViewMsg::AdminChanged(value) => {
                self.is_admin = value;
            }
            AccountViewMsg::NameChanged(value) => {
                self.name_value = value;
            }
            AccountViewMsg::AutoLoginChanged(value) => {
                self.auto_login = value;
            }
            AccountViewMsg::PasswordChanged => {}
            AccountViewMsg::ActivityChagned => {}
            AccountViewMsg::AvatarChanged => {}
        }
        self
    }
    pub fn set_props(&mut self, item: &AccountItem) -> &mut Self {
        let AccountItem {
            name,
            icon,
            acc_type,
            ..
        } = item;
        self.name_value = name.to_string();
        self.avatar = icon.to_string();
        self.is_admin = *acc_type;
        self
    }
    pub fn view(&mut self) -> Element<AccountViewMsg> {
        let AccountView {
            avatar,
            name,
            name_value,
            auto_login,
            is_admin,
            activity,
            password,
            avatar_btn,
        } = self;
        let content = Column::new().align_items(Align::Start).width(Length::Units(500)).spacing(10)
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .push(Button::new(avatar_btn,
                        Svg::from_path(format!(
                            "{}/assets/images/{}",
                            env!("CARGO_MANIFEST_DIR"),
                            avatar
                        )).width(Length::Units(100)).height(Length::Units(100))).on_press(AccountViewMsg::AvatarChanged)
                    )
                    .push(TextInput::new(
                        name,
                        "",
                        &name_value,
                        AccountViewMsg::NameChanged,
                    ).padding(10)),
            ).push(Text::new("Account Settings"))
            .push(
Container::new(
                Column::new().spacing(20).padding(10)
                    .push(Column::new().push(Checkbox::new(
                        *is_admin,
                        "Administrator",
                        AccountViewMsg::AdminChanged,
                    )).spacing(100))
                .push(Text::new("Administrators can add and remove other users, and can change settings for all users")))
                .style(CustomContainer::ForegroundWhite)).push(Text::new("Authentication & Login")).padding(10).push(
                Container::new(
                Column::new().padding(10).width(Length::Fill).spacing(5)
                .push(Button::new(password, Row::new().push(Text::new("Password")).push(Text::new("******")).width(Length::Fill)).width(Length::Fill).on_press(AccountViewMsg::PasswordChanged).padding(10))
                .push(Checkbox::new(*auto_login, "Automatic Login", AccountViewMsg::AutoLoginChanged).width(Length::Fill)).padding(10)
                .push(Button::new(activity, Text::new("Account Activity")).width(Length::Fill).padding(10)
                .on_press(AccountViewMsg::ActivityChagned))).style(CustomContainer::ForegroundWhite).width(Length::Fill)
        );
        Container::new(content)
            .center_x()
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
#[derive(Debug, Clone)]
pub enum UserInfomsg {
    FullNameChanged(String),
    UsernameChanged(String),
    AccountTypeChanged(bool),
    PasswordChanged(String),
    ConfirmChanged(String),
    TypeChanged(usize),
    AddUser,
    Back,
}

#[derive(Default, Debug, Clone)]
pub struct UserForm {
    acount_type: Vec<(String, button::State)>,
    type_val: bool,
    user_added: bool,
    is_match: bool,
    is_back: bool,
    fullname: String,
    fullname_ui: text_input::State,
    username: String,
    username_ui: text_input::State,
    password: String,
    password_ui: text_input::State,
    confirm: String,
    confirm_ui: text_input::State,
    add_btn: button::State,
    cancel_btn: button::State,
    current_idx: usize,
}

impl UserForm {
    pub fn new() -> Self {
        Self {
            acount_type: vec![
                ("Standard".to_string(), button::State::new()),
                ("Administrator".to_string(), button::State::new()),
            ],
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: UserInfomsg) -> &mut Self {
        use UserInfomsg::*;
        match msg {
            FullNameChanged(val) => {
                self.fullname = val;
            }
            UsernameChanged(val) => {
                self.username = val;
            }
            AccountTypeChanged(val) => {
                self.type_val = val;
            }
            PasswordChanged(val) => {
                self.password = val;
            }
            ConfirmChanged(val) => {
                self.confirm = val;
            }
            TypeChanged(idx) => {
                if idx == 1 {
                    self.type_val = true;
                } else {
                    self.type_val = false;
                };
                self.current_idx = idx;
            }
            Back => {
                self.is_back = true;
            }
            AddUser => {
                self.user_added = true;
            }
        }
        self
    }
    fn apply_row<'a, F>(
        name: &str,
        input: &'a mut text_input::State,
        value: &'a str,
        f: F,
    ) -> Element<'a, UserInfomsg>
    where
        F: Fn(String) -> UserInfomsg + 'static,
    {
        if name.eq_ignore_ascii_case("password") || name.eq_ignore_ascii_case("confirm") {
            Row::new()
                .align_items(Align::Center)
                .push(Text::new(name).size(FONT_SIZE).width(Length::Units(100)))
                .push(
                    TextInput::new(input, "", &value, f)
                        .password()
                        .size(FONT_SIZE)
                        .padding(10),
                )
                .into()
        } else {
            Row::new()
                .align_items(Align::Center)
                .push(Text::new(name).size(FONT_SIZE).width(Length::Units(100)))
                .push(
                    TextInput::new(input, "", &value, f)
                        .size(FONT_SIZE)
                        .padding(10),
                )
                .into()
        }
    }
    pub fn view(&mut self) -> Element<UserInfomsg> {
        let UserForm { current_idx, .. } = self;
        let type_view = Row::new()
            .align_items(Align::Center)
            .push(
                Text::new("Account Type")
                    .size(FONT_SIZE)
                    .width(Length::Units(100)),
            )
            .push(self.acount_type.iter_mut().enumerate().fold(
                Row::new(),
                |row, (index, (name, state))| {
                    row.push(
                        Button::new(state, Text::new(name.as_str()))
                            .width(Length::Fill)
                            .padding(10)
                            .on_press(UserInfomsg::TypeChanged(index))
                            .style(if *current_idx == index {
                                CustomButton::SelectType
                            } else {
                                CustomButton::Type
                            }),
                    )
                },
            ))
            .into();
        let fullname = Self::apply_row(
            "FullName",
            &mut self.fullname_ui,
            &self.fullname,
            UserInfomsg::FullNameChanged,
        );
        let username = Self::apply_row(
            "Username",
            &mut self.username_ui,
            &self.username,
            UserInfomsg::UsernameChanged,
        );
        let password = Self::apply_row(
            "Password",
            &mut self.password_ui,
            &self.password,
            UserInfomsg::PasswordChanged,
        );
        let confirm = Self::apply_row(
            "Confirm",
            &mut self.confirm_ui,
            &self.confirm,
            UserInfomsg::ConfirmChanged,
        );
        let control = Row::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(
                Column::new()
                    .align_items(Align::Start)
                    .width(Length::Fill)
                    .push(
                        Button::new(
                            &mut self.add_btn,
                            Text::new("Cancel")
                                .size(FONT_SIZE)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .style(CustomButton::Cancel)
                        .width(Length::Units(100))
                        .padding(10)
                        .on_press(UserInfomsg::Back),
                    ),
            )
            .push(Column::new().push(if self.password.eq(&self.confirm) {
                Text::new("")
            } else {
                Text::new("The passwords do not match").size(16)
            }))
            .push(
                Column::new()
                    .align_items(Align::End)
                    .width(Length::Fill)
                    .push(
                        if self.username.is_empty()
                            || self.fullname.is_empty()
                            || self.password.is_empty()
                            || self.confirm.is_empty()
                            || !self.password.eq(&self.confirm)
                        {
                            Button::new(
                                &mut self.cancel_btn,
                                Text::new("Add")
                                    .size(FONT_SIZE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .style(CustomButton::Apply)
                            .width(Length::Units(100))
                            .padding(10)
                        } else {
                            Button::new(
                                &mut self.cancel_btn,
                                Text::new("Add")
                                    .size(FONT_SIZE)
                                    .horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .style(CustomButton::Apply)
                            .width(Length::Units(100))
                            .padding(10)
                            .on_press(UserInfomsg::AddUser)
                        },
                    ),
            )
            .into();
        Container::new(
            Column::with_children(vec![
                type_view, fullname, username, password, confirm, control,
            ])
            .width(Length::Units(500))
            .spacing(10),
        )
        .center_x()
        .width(Length::FillPortion(7))
        .height(Length::Fill)
        .into()
    }
}
