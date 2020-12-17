use super::super::styles::{CustomButton, CustomContainer, CustomProgressBar};
use chrono::prelude::*;
use iced::{
    button, time, Align, Button, Checkbox, Column, Command, Container, Element, Length, ProgressBar, Row, Space, Subscription, Svg, Text,
};
use std::time::{Duration, Instant};
#[derive(Debug, Clone)]
pub enum SoftUpdateMsg {
    OnUpdate,
    OnAdvance,
    UpToDate(bool),
    Tick(Instant)
}
#[derive(Debug, Clone)]
enum State {
    Normal,
    Updating { last_tick: Instant },
}
impl Default for State {
    fn default() -> Self {
        State::Normal
    }
}
#[derive(Debug, Clone)]
enum Status {
    Update,
    LostConnection,
    LackDiskspace,
    UpdateProcess,
    Finished,
}
impl Default for Status {
    fn default() -> Self {
        Status::Update
    }
}
#[derive(Default, Debug, Clone)]
pub struct SoftwareUpdate {
    update: button::State,
    advance: button::State,
    duration: Duration,
    up_to_date: bool,
    status: Status,
    state: State,
    version: String,
    progress_val: f32,
    timing: u64,
    is_online: bool,
    is_enought_space: bool,
    is_finished: bool,
    is_downloading: bool,
    is_installing: bool,
}

impl SoftwareUpdate {
    pub fn new() -> Self {
        Self {
            timing: 10,
            version: String::from("0.0.1"),
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: SoftUpdateMsg) -> Command<SoftUpdateMsg> {
        match msg {
            SoftUpdateMsg::OnAdvance => {}
            SoftUpdateMsg::OnUpdate => match self.state {
                State::Normal => {
                    if self.is_online {
                        self.status = Status::LostConnection;
                    } else if self.is_enought_space {
                        self.status = Status::LackDiskspace;
                    } else {
                        self.status = Status::UpdateProcess;
                    }
                    self.state = State::Updating {
                        last_tick: Instant::now(),
                    };
                }
                State::Updating { .. } => self.state = State::Normal,
            },
            SoftUpdateMsg::Tick(now) => match &mut self.state {
                State::Updating { last_tick } => {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                    if self.progress_val.eq(&100.0) {
                        self.state = State::Normal;
                        self.status = Status::Finished;
                    } else {
                        self.progress_val += 10.0;
                        self.timing -= 1;
                    }
                }
                _ => {}
            },
            SoftUpdateMsg::UpToDate(val) => {
                self.up_to_date = val;
            }
        }
        Command::none()
    }
    pub fn subscription(&self) -> Subscription<SoftUpdateMsg> {
        match self.state {
            State::Normal => Subscription::none(),
            State::Updating { .. } => {
                time::every(Duration::from_millis(1000)).map(SoftUpdateMsg::Tick)
            }
        }
    }
    pub fn view(&mut self) -> Element<SoftUpdateMsg> {
        let seconds = self.duration.as_secs();
        println!("{}", seconds);
        println!("{}", self.progress_val);
        let row : Element<_> = Row::new().width(Length::Units(700)).padding(10)
            .push(
                Container::new(
                Column::new()
                    .align_items(Align::Center).width(Length::FillPortion(2)).padding(20)
                    .push(
                        Svg::from_path(format!(
                            "{}/assets/images/settings.svg",
                            env!("CARGO_MANIFEST_DIR")
                        ))
                        .width(Length::Units(128))
                        .height(Length::Units(128)),
                    ).push(Space::with_height(Length::Units(10)))
                    .push(Text::new("Software Update").size(18)))
            )
            .push(match self.status {
                Status::Update => {
                    Container::new(
                    Column::new().padding(15).spacing(20).width(Length::FillPortion(5)).align_items(Align::Center).push(
                                            Row::new().align_items(Align::Center).width(Length::Fill)
                                            .push(Svg::from_path(format!(
                                                "{}/assets/images/update.svg",
                                                env!("CARGO_MANIFEST_DIR")
                                            )).width(Length::Units(64)).height(Length::Units(64)))
                                            .push(
                                                Column::new().align_items(Align::Center).spacing(4)
                                                    .push(Text::new("Koompi OS "))
                                                    .push(Text::new(format!(
                                                        "{} - {}",self.version, "8.0 GB"
                                                    )))
                                                    .push(Text::new("More Info...")),
                                            ).push(Column::new()
                                            .width(Length::Fill)
                                            .align_items(Align::End)
                                            .push(
                                                Button::new(
                                                    &mut self.update,
                                                    Text::new("Update Now"),
                                                )
                                                .style(CustomButton::Default)
                                                .on_press(SoftUpdateMsg::OnUpdate),
                                            )))
                            .push(Column::new().spacing(10).width(Length::Fill)
                            .push(Text::new("Other updates are available.Your Computer will try to update laster tonight and will automatically restart."))
                            .push(Text::new("More Info..")))
                            .push(Column::new().spacing(10).push(Text::new("Use of this software is subject to the original license agreement that accompanied the software being updated.")))
                            .push(Row::new().width(Length::Fill).push(Checkbox::new(self.up_to_date, "Automatically keep my computer up to date", SoftUpdateMsg::UpToDate).width(Length::Fill))
                            .push(Button::new(&mut self.advance, Text::new("Advanced..."))
                            .on_press(SoftUpdateMsg::OnAdvance).style(CustomButton::Default)))).style(CustomContainer::ForegroundGray)
                }
                Status::UpdateProcess => {
                    Container::new(
                    Column::new().padding(15).width(Length::FillPortion(5))
                    .push(Text::new(if self.progress_val.ge(&70.0) {"Installing..."} else {"Downloading..."})).push(
                        ProgressBar::new(0.0..=100.00, self.progress_val).height(Length::Units(10)).style(CustomProgressBar::ForegroundGrey)
                    )
                    .push(Text::new(if !self.timing.eq(&0) {format!("About {} {} remaining", self.timing, if self.timing > 1 {"seconds"} else if self.timing == 1  {"seconds"} else {""} ) } else {format!("")}))
                    .push(Space::with_height(Length::Units(90)))
                    .push(Row::new().width(Length::Fill).push(Checkbox::new(self.up_to_date, "Automatically keep my computer up to date", SoftUpdateMsg::UpToDate).width(Length::Fill))
                    .push(Button::new(&mut self.advance, Text::new("Advanced..."))
                    .on_press(SoftUpdateMsg::OnAdvance).style(CustomButton::Default)))).style(CustomContainer::ForegroundGray)
                }
                Status::LackDiskspace => {
                    Container::new(
                    Column::new().push(Text::new("Lower Disk Space")))
                    .width(Length::FillPortion(5)).style(CustomContainer::ForegroundGray)
                }
                Status::Finished => {
                    Container::new(Column::new().spacing(10).push(Row::new().push(Row::new().align_items(Align::Center).push(
                        Text::new("You computer is up to date (KOOMPI OS 0.2.5)").size(20)
                    ).push(Svg::from_path(format!("{}/assets/images/updated.svg",env!("CARGO_MANIFEST_DIR")))))).push(
                        Text::new(format!("Last Check: {}", Local::now().date().to_string()))
                    )
                    .push(Space::with_height(Length::Units(10)))
                    .push(Row::new().width(Length::Fill).push(Checkbox::new(self.up_to_date, "Automatically keep my computer up to date", SoftUpdateMsg::UpToDate).width(Length::Fill))
                    .push(Button::new(&mut self.advance, Text::new("Advanced...")).on_press(SoftUpdateMsg::OnAdvance).style(CustomButton::Default))))
                    .padding(15).style(CustomContainer::ForegroundGray)
                }, 
                Status::LostConnection => { Container::new(
                    Column::new().push(Text::new("No Internet Connection")))}
            }).into();
        Container::new(row)
            .style(CustomContainer::Background)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
