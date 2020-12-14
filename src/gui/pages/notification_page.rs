const LABEL_WIDTH: u16 = 250;
const ROW_WIDTH: u16 = 500;
use super::super::styles::{CustomButton, CustomCheckbox, CustomRadio};
use iced::{
    button, text_input, Align, Button, Checkbox, Column, Container, Element, HorizontalAlignment,
    Length, Radio, Row, Rule, Space, Text, TextInput,
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
    fn checkbox_cus<'a, F>(state: bool, label: &str, f: F) -> Checkbox<NotifyMsg>
    where
        F: Fn(bool) -> NotifyMsg + 'static,
    {
        Checkbox::new(state, label, f).style(CustomCheckbox::Default)
    }
    pub fn view(&mut self) -> Element<NotifyMsg> {
        let main_settings = Column::new()
            .push(
                Self::row_template("Do Not Disturb mode:").push(
                    Column::new()
                        .align_items(Align::Start)
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
            .push(Rule::horizontal(2))
            .push(
                Self::row_template("Critical notifications:").push(
                    Column::new()
                        .align_items(Align::Start)
                        .push(Self::checkbox_cus(
                            self.critical,
                            "Always keep on top",
                            NotifyMsg::CriticalChanged,
                        )),
                ),
            )
            .push(
                Self::row_template("Low priority notificatoins:").push(
                    Column::new()
                        .spacing(10)
                        .align_items(Align::Start)
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
                    Column::new()
                        .spacing(10)
                        .align_items(Align::Start)
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
            .push(Rule::horizontal(2))
            .push(
                Self::row_template("Application progress:").push(
                    Column::new()
                        .spacing(10)
                        .align_items(Align::Start)
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
            .push(Rule::horizontal(2))
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
}
