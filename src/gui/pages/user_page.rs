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
                    Self::string_convert("sna", "online-user.svg", "admin"),
                    button::State::new(),
                ),
                (
                    Self::string_convert("rotha", "online-user.svg", "user"),
                    button::State::new(),
                ),
                (
                    Self::string_convert("vannak", "kuser.svg", "user"),
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
            UserAdded(value) => {
                self.list_acc.push((
                    AccountItem::new().set_props(
                        "Sna".to_string(),
                        "kuser.svg".to_string(),
                        "Admin".to_string(),
                    ),
                    button::State::new(),
                ));
                // self.is_added = value
            }
            UserPageMsg::UserRemove(idx) => {
                self.current_index = idx;
                println!("index: {}", idx);
                if !self.list_acc.is_empty() {
                    if idx.lt(&self.list_acc.len()) & !idx.eq(&0) {
                        self.list_acc.remove(idx);
                    }
                    let data = self.list_acc.get(idx - 1).unwrap();
                    self.list_view.set_props(&data.0);
                } else {
                    {}
                }
            }
            UserPageMsg::AccountSwitch(value) => {
                let data = self.list_acc.get(value).unwrap();
                self.current_index = value;
                self.list_view.set_props(&data.0);
            }
            UserPageMsg::AccountViewMsg(msg) => {
                self.list_view.update(msg);
            }
            UserPageMsg::UserInfomsg(msg) => {
                self.user_info.update(msg);
            }
        }
    }
    fn string_convert(name: &str, icon: &str, acc_type: &str) -> AccountItem {
        AccountItem::new().set_props(name.to_string(), icon.to_string(), acc_type.to_string())
    }
    pub fn view(&mut self) -> Element<UserPageMsg> {
        let UserPage {
            unlock,
            is_unlock,
            is_added,
            current_acc,
            list_acc,
            list_view,
            user_info,
            list_scroll,
            add_btn,
            remove_btn,
            current_index,
        } = self;
        let _banner = Row::new().push(Text::new("Security")).push(Text::new(
            "Unlock to Change Settings\nSome setting must be nlocked before they can be changed.",
        )).push(Button::new(unlock, Text::new("Unlock...")).on_press(UserPageMsg::Unlocked));
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
                        Button::new(add_btn, Text::new("Add New User"))
                            .width(Length::Fill)
                            .on_press(UserPageMsg::UserAdded(true))
                            .padding(10),
                    )
                    .push(
                        Button::new(remove_btn, Text::new("Remove User"))
                            .width(Length::Fill)
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
        let main_layout = Row::new().push(list_control_container).push(user_view);
        Container::new(main_layout)
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
    acc_type: String,
}
#[derive(Debug, Clone)]
pub enum AccountItemMsg {}
impl AccountItem {
    fn new() -> Self {
        Self {
            icon: "setting.svg".to_string(),
            name: "Sna".to_string(),
            username: "Sna".to_string(),
            acc_type: "Login User".to_string(),
        }
    }
    fn set_props(mut self, name: String, icon: String, acc_type: String) -> Self {
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
                    .push(Text::new(acc_type.as_str())),
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

    pub fn update(&mut self, msg: AccountViewMsg) {
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
        self.is_admin = true;
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
    AddUser,
    Back,
}

#[derive(Default, Debug, Clone)]
pub struct UserForm {
    acount_type: [button::State; 2],
    type_val: bool,
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
    list_item: Vec<(AccountItem, button::State)>,
}

impl UserForm {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: UserInfomsg) {
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
            Back => {}
            AddUser => {}
        }
    }
    pub fn set_list(mut self, list: &[(AccountItem, button::State)]) -> Self {
        self.list_item = list.to_vec();
        self
    }
    pub fn view(&mut self) -> Element<UserInfomsg> {
        let type_view = Row::new()
            .align_items(Align::Center)
            .push(
                Text::new("Account Type")
                    .size(FONT_SIZE)
                    .width(Length::Units(100)),
            )
            .push(self.acount_type.iter_mut().fold(Row::new(), |row, state| {
                row.push(
                    Button::new(state, Text::new("Standard"))
                        .padding(10)
                        .width(Length::Fill),
                )
            }))
            .into();
        let fullname = Row::new()
            .align_items(Align::Center)
            .push(
                Text::new("FullName")
                    .size(FONT_SIZE)
                    .width(Length::Units(100)),
            )
            .push(
                TextInput::new(
                    &mut self.fullname_ui,
                    "",
                    &self.fullname,
                    UserInfomsg::FullNameChanged,
                )
                .size(FONT_SIZE)
                .padding(10),
            )
            .into();
        let username = Row::new()
            .align_items(Align::Center)
            .push(
                Text::new("Uername")
                    .size(FONT_SIZE)
                    .width(Length::Units(100)),
            )
            .push(
                TextInput::new(
                    &mut self.username_ui,
                    "",
                    &self.username,
                    UserInfomsg::UsernameChanged,
                )
                .size(FONT_SIZE)
                .padding(10),
            )
            .into();
        let password = Row::new()
            .align_items(Align::Center)
            .push(
                Text::new("Password")
                    .size(FONT_SIZE)
                    .width(Length::Units(100)),
            )
            .push(
                TextInput::new(
                    &mut self.password_ui,
                    "",
                    &self.password,
                    UserInfomsg::PasswordChanged,
                )
                .size(FONT_SIZE)
                .password()
                .padding(10),
            )
            .into();
        let confirm = Row::new()
            .align_items(Align::Center)
            .push(
                Text::new("confirm")
                    .size(FONT_SIZE)
                    .width(Length::Units(100)),
            )
            .push(
                TextInput::new(
                    &mut self.confirm_ui,
                    "",
                    &self.confirm,
                    UserInfomsg::ConfirmChanged,
                )
                .size(FONT_SIZE)
                .padding(10)
                .password(),
            )
            .into();
        let control = Row::new()
            .spacing(10)
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
                        .width(Length::Units(100))
                        .padding(10)
                        .on_press(UserInfomsg::Back),
                    ),
            )
            .push(
                Column::new()
                    .align_items(Align::End)
                    .width(Length::Fill)
                    .push(
                        Button::new(
                            &mut self.cancel_btn,
                            Text::new("Add")
                                .size(FONT_SIZE)
                                .horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .width(Length::Units(100))
                        .padding(10)
                        .on_press(UserInfomsg::AddUser),
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
