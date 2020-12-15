const LABEL_WIDTH: u16 = 250;
const ROW_WIDTH: u16 = 500;
use super::super::styles::{CustomButton, CustomCheckbox, CustomContainer, CustomRadio};
use iced::{
    button, text_input, Align, Button, Checkbox, Column, Container, Element, HorizontalAlignment,
    Length, Radio, Row, Rule, Space, Svg, Text, TextInput,
};

#[derive(Debug, Clone)]
pub enum NotifyMsg {
    Disturb1Changed(bool),
    Disturb2Changed(bool),
    CriticalChanged(bool),
    Priority1Changed(bool),
    Priority2Changed(bool),
    Progress1Changed(bool),
    Progress2Changed(bool),
    Progress3Changed(bool),
    BadgesChanged(bool),
    HideTimeChanged(String),
    SwitchApp(usize),
    ApplicationChagned,
    PopupChanged(bool),
    BackHome,
    CustomPosition,
    OnSearch(String),
}
#[derive(Default, Debug, Clone)]
pub struct NotifyPage {
    disturb1: bool,
    disturb2: bool,
    critical: bool,
    priority1: bool,
    priority2: bool,
    appprogress1: bool,
    appprogress2: bool,
    appprogress3: bool,
    notifybadges: bool,
    hide_time: text_input::State,
    hidetimevalue: String,
    app: button::State,
    is_poistion: bool,
    configure: button::State,
    cusviews: CustomView,
    back_home: button::State,
    custom_pos: button::State,
    current_idx: usize,
    list_item: Vec<(AppListItem, button::State)>,
    filter_list: Vec<(AppListItem, button::State)>,
    search: text_input::State,
    search_val: String,
}
#[derive(Debug, Clone)]
pub enum CustomView {
    Home,
    Configure,
    Position,
}
impl Default for CustomView {
    fn default() -> Self {
        CustomView::Home
    }
}

impl NotifyPage {
    pub fn new() -> Self {
        let data = |name: &str, icon: &str| -> (AppListItem, button::State) {
            (
                AppListItem::new().set_props(name, icon),
                button::State::new(),
            )
        };
        Self {
            list_item: vec![
                data("Teams", "teams"),
                data("Telegram", "telegram"),
                data("Firefox", "firefox"),
                data("Discord", "discord"),
                data("Chrome", "chrome"),
            ],
            filter_list: vec![
                data("Teams", "teams"),
                data("Telegram", "telegram"),
                data("Firefox", "firefox"),
                data("Discord", "discord"),
                data("Chrome", "chrome"),
            ],
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: NotifyMsg) -> &mut Self {
        use NotifyMsg::*;
        match msg {
            Disturb1Changed(val) => {
                self.disturb1 = val;
            }
            Disturb2Changed(val) => {
                self.disturb2 = val;
            }
            CriticalChanged(val) => self.critical = val,
            Priority1Changed(val) => self.priority1 = val,
            Priority2Changed(val) => self.priority2 = val,
            PopupChanged(val) => self.is_poistion = val,
            Progress1Changed(val) => self.appprogress1 = val,
            Progress2Changed(val) => self.appprogress2 = val,
            Progress3Changed(val) => self.appprogress3 = val,
            BadgesChanged(val) => self.notifybadges = val,
            HideTimeChanged(val) => self.hidetimevalue = val,
            ApplicationChagned => {
                self.cusviews = CustomView::Configure;
            }
            CustomPosition => {
                self.cusviews = CustomView::Position;
            }
            BackHome => self.cusviews = CustomView::Home,
            SwitchApp(idx) => {
                self.current_idx = idx;
            }
            OnSearch(val) => {
                self.search_val = val;
                self.filter_list = self
                    .list_item
                    .iter()
                    .filter(|(item, _)| {
                        item.name
                            .to_lowercase()
                            .contains(&self.search_val.to_lowercase())
                    })
                    .cloned()
                    .collect();
            }
        }
        self
    }
    fn row_template<'a>(label: &str) -> Row<'a, NotifyMsg> {
        Row::new()
            .width(Length::Units(ROW_WIDTH))
            .push(
                Text::new(label)
                    .size(18)
                    .horizontal_alignment(HorizontalAlignment::Right)
                    .width(Length::Units(LABEL_WIDTH)),
            )
            .spacing(4)
    }
    fn column_cus<'a>() -> Column<'a, NotifyMsg> {
        Column::new().align_items(Align::Start)
    }
    fn checkbox_cus<'a, F>(state: bool, label: &str, f: F) -> Checkbox<NotifyMsg>
    where
        F: Fn(bool) -> NotifyMsg + 'static,
    {
        Checkbox::new(state, label, f).style(CustomCheckbox::Default)
    }
    fn custom_space<'a>() -> Row<'a, NotifyMsg> {
        Row::new()
            .push(Rule::horizontal(2))
            .width(Length::Units(ROW_WIDTH))
    }
    pub fn view(&mut self) -> Element<NotifyMsg> {
        match self.cusviews {
            CustomView::Home => {
                let main_settings = Column::new()
                    .push(
                        Self::row_template("Do Not Disturb mode:").push(
                            Self::column_cus()
                                .spacing(10)
                                .push(Self::checkbox_cus(
                                    self.disturb1,
                                    "Enable when screens are mirrored",
                                    NotifyMsg::Disturb1Changed,
                                ))
                                .push(Self::checkbox_cus(
                                    self.disturb2,
                                    "Show critical Notificatons",
                                    NotifyMsg::Disturb2Changed,
                                )),
                        ),
                    )
                    .push(Space::with_height(Length::Units(50)))
                    .push(Self::custom_space())
                    .push(Self::row_template("Critical notifications:").push(
                        Self::column_cus().push(Self::checkbox_cus(
                            self.critical,
                            "Always keep on top",
                            NotifyMsg::CriticalChanged,
                        )),
                    ))
                    .push(
                        Self::row_template("Low priority notificatoins:").push(
                            Self::column_cus()
                                .spacing(10)
                                .push(Self::checkbox_cus(
                                    self.priority1,
                                    "Show popup",
                                    NotifyMsg::Priority1Changed,
                                ))
                                .push(Self::checkbox_cus(
                                    self.priority2,
                                    "Show in history",
                                    NotifyMsg::Priority2Changed,
                                )),
                        ),
                    )
                    .push(
                        Self::row_template("Popup:").push(
                            Self::column_cus()
                                .spacing(10)
                                .push(
                                    Radio::new(
                                        true,
                                        "Show near notification icon",
                                        Some(self.is_poistion),
                                        NotifyMsg::PopupChanged,
                                    )
                                    .size(18)
                                    .style(CustomRadio::Purple),
                                )
                                .push(
                                    Radio::new(
                                        false,
                                        "Show popup with positions",
                                        Some(self.is_poistion),
                                        NotifyMsg::PopupChanged,
                                    )
                                    .size(18)
                                    .style(CustomRadio::Purple),
                                )
                                .push(
                                    Button::new(
                                        &mut self.custom_pos,
                                        Text::new("Custom Position ..."),
                                    )
                                    .style(CustomButton::Cancel)
                                    .on_press(NotifyMsg::CustomPosition),
                                )
                                .push(
                                    Row::new()
                                        .align_items(Align::Center)
                                        .push(Text::new("Hide after: "))
                                        .push(
                                            TextInput::new(
                                                &mut self.hide_time,
                                                "8 seconds",
                                                &self.hidetimevalue,
                                                NotifyMsg::HideTimeChanged,
                                            )
                                            .width(Length::Units(100))
                                            .padding(10),
                                        ),
                                ),
                        ),
                    )
                    .push(Self::custom_space())
                    .push(
                        Self::row_template("Application progress:").push(
                            Self::column_cus()
                                .spacing(10)
                                .push(Self::checkbox_cus(
                                    self.appprogress1,
                                    "Show in task manager",
                                    NotifyMsg::Progress1Changed,
                                ))
                                .push(Self::checkbox_cus(
                                    self.appprogress2,
                                    "Show in notificaitons",
                                    NotifyMsg::Progress2Changed,
                                ))
                                .push(Self::checkbox_cus(
                                    self.appprogress3,
                                    "Keep popup open during progress",
                                    NotifyMsg::Progress3Changed,
                                )),
                        ),
                    )
                    .push(
                        Self::row_template("Notification Badges:").push(Self::checkbox_cus(
                            self.notifybadges,
                            "Show in task manager",
                            NotifyMsg::BadgesChanged,
                        )),
                    )
                    .push(Self::custom_space())
                    .push(
                        Self::row_template("Applications:").push(
                            Button::new(&mut self.configure, Text::new("Configure..."))
                                .style(CustomButton::Cancel)
                                .on_press(NotifyMsg::ApplicationChagned),
                        ),
                    )
                    .align_items(Align::Center)
                    .spacing(10);
                Container::new(main_settings)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .into()
            }
            CustomView::Configure => {
                let NotifyPage {
                    current_idx,
                    filter_list,
                    ..
                } = self;
                let list_view: Element<_> = if filter_list.len() > 0 {
                    filter_list
                        .iter_mut()
                        .enumerate()
                        .fold(
                            Column::new()
                                .push(
                                    Container::new(Text::new("Applications").size(18))
                                        .padding(10)
                                        .width(Length::Fill)
                                        .style(CustomContainer::Background),
                                )
                                .align_items(Align::Center)
                                .spacing(4)
                                .width(Length::Fill)
                                .height(Length::Fill),
                            |column, (idx, (item, state))| {
                                column.push(Rule::horizontal(2)).push(
                                    Button::new(state, item.view())
                                        .width(Length::Fill)
                                        .on_press(NotifyMsg::SwitchApp(idx))
                                        .style(if *current_idx == idx {
                                            CustomButton::Selected
                                        } else {
                                            CustomButton::Sidebar
                                        }),
                                )
                            },
                        )
                        .into()
                } else {
                    Container::new(Text::new("No applciations match your search").size(18))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_y()
                        .center_x()
                        .into()
                };
                let column = Column::new()
                    .padding(10)
                    .spacing(10)
                    .push(
                        TextInput::new(
                            &mut self.search,
                            "Search....",
                            &self.search_val,
                            NotifyMsg::OnSearch,
                        )
                        .width(Length::Fill)
                        .padding(10),
                    )
                    .push(
                        Container::new(list_view)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(CustomContainer::ForegroundWhite),
                    )
                    .width(Length::FillPortion(3))
                    .height(Length::Fill);
                let content: Element<_> = Row::new()
                    .push(column)
                    .push(
                        Column::new()
                            .height(Length::Fill)
                            .width(Length::FillPortion(6)),
                    )
                    .into();
                Container::new(content)
                    .style(CustomContainer::ForegroundGray)
                    .center_x()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            CustomView::Position => Button::new(&mut self.back_home, Text::new("Home"))
                .on_press(NotifyMsg::BackHome)
                .style(CustomButton::Apply)
                .into(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AppListItem {
    name: String,
    icon: String,
}
impl AppListItem {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn view(&mut self) -> Element<NotifyMsg> {
        Row::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Svg::from_path(format!(
                    "{}/assets/images/{}.svg",
                    env!("CARGO_MANIFEST_DIR"),
                    self.icon
                ))
                .width(Length::Units(48))
                .height(Length::Units(48)),
            )
            .spacing(4)
            .push(Text::new(self.name.as_str()).size(18))
            .into()
    }
    pub fn set_props(mut self, name: &str, icon: &str) -> Self {
        self.name = name.to_string();
        self.icon = icon.to_string();
        self
    }
}

#[derive(Default, Debug, Clone)]
pub struct AppNotifSettings {
    settings
}
