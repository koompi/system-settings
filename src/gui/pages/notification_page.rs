use iced::{
    button, text_input, Align, Button, Checkbox, Color, Column, Container, Element, Length, Radio,
    Row, Rule, Text, TextInput,
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
    ApplicationChagned,
    PopupChanged(bool),
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
}

impl NotifyPage {
    pub fn new() -> Self {
        Self {
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
                println!("app pressed");
            }
        }
        self
    }
    pub fn view(&mut self) -> Element<NotifyMsg> {
        let main_settings: Element<_> = Column::new()
            .push(
                Row::new()
                    .push(Text::new("Do Not disturb mode: ").width(Length::Units(100)))
                    .push(
                        Column::new()
                            .push(Checkbox::new(
                                self.disturb1,
                                "Enable when screens are mirrored",
                                NotifyMsg::Disturb1Changed,
                            ))
                            .push(Checkbox::new(
                                self.disturb2,
                                "Show scritical notifications",
                                NotifyMsg::Disturb2Changed,
                            )),
                    ),
            )
            .push(Rule::horizontal(2))
            .push(
                Row::new()
                    .push(Text::new("Critical notifications").width(Length::Units(100)))
                    .push(Checkbox::new(
                        self.critical,
                        "Always keep on top",
                        NotifyMsg::CriticalChanged,
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Low priority notificatoins: ").width(Length::Units(100)))
                    .push(
                        Column::new()
                            .push(Checkbox::new(
                                self.priority1,
                                "Show popup",
                                NotifyMsg::Priority1Changed,
                            ))
                            .push(Checkbox::new(
                                self.priority2,
                                "Show in history",
                                NotifyMsg::Priority2Changed,
                            )),
                    ),
            )
            .push(Rule::horizontal(2))
            .push(
                Row::new()
                    .push(Text::new("Popup").width(Length::Units(100)))
                    .push(
                        Column::new()
                            .push(Radio::new(
                                true,
                                "Show near notification icon",
                                Some(self.is_poistion),
                                NotifyMsg::PopupChanged,
                            ))
                            .push(Radio::new(
                                false,
                                "Show popup with positions",
                                Some(self.is_poistion),
                                NotifyMsg::PopupChanged,
                            ))
                            .push(
                                Row::new().push(Text::new("Hide after: ")).push(
                                    TextInput::new(
                                        &mut self.hide_time,
                                        "8 seconds",
                                        &self.hidetimevalue,
                                        NotifyMsg::HideTimeChanged,
                                    )
                                    .padding(10),
                                ),
                            ),
                    ),
            )
            .push(Rule::horizontal(2))
            .push(
                Row::new()
                    .push(Text::new("Application progress: ").width(Length::Units(100)))
                    .push(
                        Column::new()
                            .push(Checkbox::new(
                                self.appprogress1,
                                "Show in task manager",
                                NotifyMsg::Progress1Changed,
                            ))
                            .push(Checkbox::new(
                                self.appprogress2,
                                "Show in notificaitons",
                                NotifyMsg::Progress2Changed,
                            ))
                            .push(Checkbox::new(
                                self.appprogress3,
                                "Keep popup open during progress",
                                NotifyMsg::Progress3Changed,
                            )),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new("Notification Badges: ").width(Length::Units(100)))
                    .push(Checkbox::new(
                        self.notifybadges,
                        "Show in task manager",
                        NotifyMsg::BadgesChanged,
                    )),
            )
            .push(Rule::horizontal(2))
            .push(
                Row::new()
                    .push(Text::new("Applications").width(Length::Units(100)))
                    .push(
                        Button::new(&mut self.configure, Text::new("Configure"))
                            .on_press(NotifyMsg::ApplicationChagned),
                    ),
            )
            .align_items(Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .into();
        Container::new(main_settings.explain(Color::BLACK))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
