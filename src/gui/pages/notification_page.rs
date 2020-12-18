const LABEL_WIDTH: u16 = 250;
const ROW_WIDTH: u16 = 500;
use super::super::styles::{
    CustomButton, CustomCheckbox, CustomContainer, CustomRadio, CustomSelect,
};
use crate::helpers::ROOT_PATH;
use iced::{
    button, pick_list, text_input, Align, Button, Checkbox, Column, Container, Element,
    HorizontalAlignment, Length, PickList, Radio, Row, Rule, Space, Svg, Text, TextInput,
};
use vedas_core::macros::select;

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
    AppNotifSettingsMsg(AppNotifSettingsMsg),
    ApplyChanged,
    ApplyPosition,
    PositionChanged(Position),
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
    applistsetting: Vec<AppNotifSettings>,
    apply: button::State,
    apply_position: button::State,
    select_positon: Position,
    select_state: pick_list::State<Position>,
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
            applistsetting: vec![AppNotifSettings::new(); 5],
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
            PositionChanged(pos) => {
                self.select_positon = pos;
            }
            AppNotifSettingsMsg(msg) => match self.current_idx {
                0 => {
                    self.applistsetting[0].update(msg);
                }
                1 => {
                    self.applistsetting[1].update(msg);
                }
                2 => {
                    self.applistsetting[2].update(msg);
                }
                3 => {
                    self.applistsetting[3].update(msg);
                }
                4 => {
                    self.applistsetting[4].update(msg);
                }
                _ => {}
            },
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
            ApplyPosition => {

            }
            ApplyChanged => {}
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
                            .padding(10)
                            .align_items(Align::End)
                            .width(Length::FillPortion(6))
                            .push(match *current_idx {
                                0 => self.applistsetting[0]
                                    .view(&self.list_item.get(*current_idx).unwrap().0)
                                    .map(move |msg| NotifyMsg::AppNotifSettingsMsg(msg)),
                                1 => self.applistsetting[1]
                                    .view(&self.list_item.get(*current_idx).unwrap().0)
                                    .map(move |msg| NotifyMsg::AppNotifSettingsMsg(msg)),
                                2 => self.applistsetting[2]
                                    .view(&self.list_item.get(*current_idx).unwrap().0)
                                    .map(move |msg| NotifyMsg::AppNotifSettingsMsg(msg)),
                                3 => self.applistsetting[3]
                                    .view(&self.list_item.get(*current_idx).unwrap().0)
                                    .map(move |msg| NotifyMsg::AppNotifSettingsMsg(msg)),
                                4 => self.applistsetting[4]
                                    .view(&self.list_item.get(*current_idx).unwrap().0)
                                    .map(move |msg| NotifyMsg::AppNotifSettingsMsg(msg)),
                                _ => Container::new(Text::new("Please Select item in the list"))
                                    .into(),
                            })
                            .push(
                                Row::new()
                                    .push(
                                        Button::new(
                                            &mut self.back_home,
                                            Text::new("Back")
                                                .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .style(CustomButton::Cancel)
                                        .on_press(NotifyMsg::BackHome)
                                        .padding(10)
                                        .width(Length::Units(100)),
                                    )
                                    .spacing(10)
                                    .push(
                                        Button::new(
                                            &mut self.apply,
                                            Text::new("Apply")
                                                .horizontal_alignment(HorizontalAlignment::Center),
                                        )
                                        .padding(10)
                                        .on_press(NotifyMsg::ApplyChanged)
                                        .width(Length::Units(100))
                                        .style(CustomButton::Apply),
                                    ),
                            ),
                    )
                    .into();
                Container::new(content)
                    .style(CustomContainer::ForegroundGray)
                    .center_x()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            CustomView::Position => {
                let NotifyPage {
                    select_state,
                    select_positon,
                    ..
                } = self;
                let data = Column::new()
                    .align_items(Align::Center)
                    .align_items(Align::End)
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_items(Align::Center)
                            .push(
                                Svg::from_path(format!(
                                    "{}/assets/images/desktop.svg",
                                    ROOT_PATH()
                                ))
                                .width(Length::Units(256))
                                .height(Length::Units(256)),
                            )
                            .push(Text::new(
                                "Select option belows for showing on screen position",
                            ))
                            .push(
                                PickList::new(
                                    select_state,
                                    &Position::ALL[..],
                                    Some(*select_positon),
                                    NotifyMsg::PositionChanged,
                                )
                                .padding(10)
                                .style(CustomSelect::Default),
                            ),
                    )
                    .push(
                        Row::new()
                            .push(
                                Button::new(
                                    &mut self.back_home,
                                    Text::new("Back")
                                        .horizontal_alignment(HorizontalAlignment::Center),
                                )
                                .style(CustomButton::Cancel)
                                .on_press(NotifyMsg::BackHome)
                                .padding(10)
                                .width(Length::Units(100)),
                            )
                            .spacing(10)
                            .push(
                                Button::new(
                                    &mut self.apply_position,
                                    Text::new("Apply")
                                        .horizontal_alignment(HorizontalAlignment::Center),
                                )
                                .padding(10)
                                .on_press(NotifyMsg::ApplyPosition)
                                .width(Length::Units(100))
                                .style(CustomButton::Apply),

                            ),
                    );
                Container::new(data)
                    .padding(10)
                    .center_x()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
        }
    }
}
impl Default for Position {
    fn default() -> Self {
        Position::BottomLeft
    }
}
impl Position {
    const ALL: [Position; 6] = [
        Position::TopLeft,
        Position::TopRight,
        Position::BottomLeft,
        Position::BottomRight,
        Position::MiddleBottom,
        Position::MiddleTop,
    ];
}
select_display!(Position, 
Position::TopLeft => "Top Left", 
Position::TopRight => "Top Right", 
Position::BottomLeft => "Bottom Left",
Position::BottomRight => "Bottom Right", 
Position::MiddleBottom => "Middle Bottom",
Position::MiddleTop => "Middle Top");
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    MiddleTop,
    MiddleBottom,
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
                    ROOT_PATH(),
                    self.icon
                ))
                .width(Length::Units(32))
                .height(Length::Units(32)),
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
#[derive(Debug, Clone)]
pub enum AppNotifSettingsMsg {
    OnShowPopup(bool),
    OnNoDisturb(bool),
    OnShowHistory(bool),
    OnNotifyBadgets(bool),
}
#[derive(Default, Debug, Clone)]
pub struct AppNotifSettings {
    popup: bool,
    nodisturb: bool,
    history: bool,
    notifybadges: bool,
}

impl AppNotifSettings {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: AppNotifSettingsMsg) -> &mut Self {
        use AppNotifSettingsMsg::*;
        match msg {
            OnShowPopup(val) => self.popup = val,
            OnNoDisturb(val) => self.nodisturb = val,
            OnShowHistory(val) => self.history = val,
            OnNotifyBadgets(val) => self.notifybadges = val,
        }
        self
    }
    pub fn view(&mut self, item: &AppListItem) -> Element<AppNotifSettingsMsg> {
        Container::new(
            Column::new()
                .spacing(10)
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(
                            Svg::from_path(format!(
                                "{}/assets/images/{}.svg",
                                ROOT_PATH(),
                                item.icon
                            ))
                            .width(Length::Units(48))
                            .height(Length::Units(48)),
                        )
                        .push(Text::new(item.name.as_str()).size(18)),
                )
                .push(Checkbox::new(
                    self.popup,
                    "Show popups",
                    AppNotifSettingsMsg::OnShowPopup,
                ))
                .push(Checkbox::new(
                    self.nodisturb,
                    "Show in Do not disturb mode",
                    AppNotifSettingsMsg::OnNoDisturb,
                ))
                .push(Checkbox::new(
                    self.history,
                    "Show in history",
                    AppNotifSettingsMsg::OnShowHistory,
                ))
                .push(Checkbox::new(
                    self.notifybadges,
                    "Show notificatons badges",
                    AppNotifSettingsMsg::OnNotifyBadgets,
                )),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
    }
}
