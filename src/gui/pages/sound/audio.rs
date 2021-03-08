use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle, picklist::PickListStyle, sliders::SliderStyle};
use iced::{button, pick_list, slider, Align, Button, Column, Container, Element, Font, HorizontalAlignment, Length, PickList, Row, Slider, Space, Text};
use libkoompi::system_settings::sounds::controllers::{AppControl, DeviceControl, SinkController, SourceController};
use libkoompi::system_settings::SoundCard;
use std::fmt;

#[derive(Default)]
pub struct AudioTab {
    speaker_mute: button::State,
    headset_mute: button::State,
    notification_mute: button::State,
    is_speak_mute: bool,
    is_headset_mute: bool,
    is_notification_mute: bool,
    output_slider: slider::State,
    input_slider: slider::State,
    notification_slider: slider::State,
    output_pick: pick_list::State<OutputPort>,
    select_output_pick: OutputPort,
    input_pick: pick_list::State<InputPort>,
    select_input_pick: InputPort,
    output_val: f64,
    input_val: f64,
    notify_val: f64,
    sink_input: SinkController,
    source_output: SourceController,
    list_ports: Vec<OutputPort>,
    list_source_ports: Vec<InputPort>,
    list_sinks: Vec<(String, String)>,
    list_sources: Vec<(String, String)>,
}
// pub fn list_devices(dev: &Vec::<>
impl AudioTab {
    pub fn new() -> Self {
        let mut sink_obj = SinkController::create();
        let mut source_obj = SourceController::create();
        let list_sink_dev = sink_obj.list_devices();
        let list_source_dev = source_obj.list_devices();
        let mut sinks = Vec::new();
        let mut sources = Vec::new();
        let mut list_src_ports = Vec::new();
        let mut list_ports_device = Vec::new();
        let current_sinks = sink_obj.get_volume();
        let current_source = source_obj.get_volume();
        println!("Current Source Volume: {:?}", current_source);
        println!("Current Sink Volume: {:?}", current_sinks);

        // get all sinks and sources device description and name
        match list_sink_dev {
            Ok(devices) => {
                for dev in devices {
                    sinks.push((
                        match dev.name {
                            Some(dev_name) => dev_name,
                            None => String::from(""),
                        },
                        match dev.description {
                            Some(descr) => descr,
                            None => String::from(""),
                        },
                    ))
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
        match list_source_dev {
            Ok(devices) => {
                for dev in devices {
                    sources.push((
                        match dev.name {
                            Some(dev_name) => dev_name,
                            None => String::from(""),
                        },
                        match dev.description {
                            Some(descr) => descr,
                            None => String::from(""),
                        },
                    ));
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
        match sink_obj.get_card_info_list() {
            Ok(list_cards) => {
                for data in list_cards {
                    for ports in data.ports {
                        list_ports_device.push(match ports.description {
                            Some(ref port_name) => OutputPort { port: port_name.to_string() },
                            None => OutputPort { port: "".to_string() },
                        });
                        list_src_ports.push(match ports.description {
                            Some(port_name) => InputPort { port: port_name },
                            None => InputPort { port: "".to_string() },
                        });
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
        let first_port: String = match list_ports_device.get(2) {
            Some(d) => d.port.clone(),
            None => String::from("Port Unavailable"),
        };
        let second_port: String = match list_src_ports.get(0) {
            Some(d) => d.port.clone(),
            None => String::from("Port Unavailable"),
        };
        Self {
            list_sinks: sinks,
            list_sources: sources,
            list_ports: list_ports_device,
            list_source_ports: list_src_ports,
            output_val: match current_sinks {
                Ok(mut vec_vol) => match vec_vol.pop() {
                    Some(val) => match val.parse() {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            50.0
                        }
                    },
                    None => 50.0,
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    50.0
                }
            },
            input_val: match current_source {
                Ok(mut vec_vol) => match vec_vol.pop() {
                    Some(val) => match val.parse() {
                        Ok(d) => d,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            50.0
                        }
                    },
                    None => 50.0,
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    50.0
                }
            },
            sink_input: sink_obj,
            source_output: source_obj,
            select_input_pick: InputPort { port: second_port },
            select_output_pick: OutputPort { port: first_port },
            ..AudioTab::default()
        }
    }
    pub fn update(&mut self, msg: AudioTabMsg) {
        match msg {
            AudioTabMsg::InputChanged(val) => {
                for dev in &self.list_sources {
                    match self.source_output.set_device_volume_by_name(&dev.0, val / 100.0) {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                }
                self.input_val = val;
            }
            AudioTabMsg::OutputChanged(val) => {
                for dev in &self.list_sinks {
                    match self.sink_input.set_device_volume_by_name(&dev.0, val / 100.0) {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                        }
                    }
                }
                self.output_val = val;
            }
            AudioTabMsg::NotifyChanged(val) => self.notify_val = val,
            AudioTabMsg::OutputPortChanged(port) => self.select_output_pick = port,
            AudioTabMsg::InputPortChanged(port) => self.select_input_pick = port,
            AudioTabMsg::SpeakerMute => {
                self.is_speak_mute = !self.is_speak_mute;
                for dev in &self.list_sinks {
                    match self.sink_input.set_app_mute_by_name(&dev.0, if self.is_speak_mute { true } else { false }) {
                        Ok(_) => {}
                        Err(e) => println!("Error: {:?}", e),
                    }
                }
            }
            AudioTabMsg::MicrophoneMute => {
                self.is_headset_mute = !self.is_headset_mute;
                for dev in &self.list_sources {
                    match self.source_output.set_app_mute_by_name(&dev.0, if self.is_headset_mute { true } else { false }) {
                        Ok(_) => {}
                        Err(e) => println!("Error: {:?}", e),
                    }
                }
            }
            AudioTabMsg::NotificationMute => {
                self.is_notification_mute = !self.is_notification_mute;
            }
        }
    }
    pub fn view(&mut self) -> Element<AudioTabMsg> {
        let output_view = Container::new(
            Column::new()
                .spacing(10)
                .align_items(Align::Center)
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .push(Text::new("Headphones (Built-in Audio Analog stereo").width(Length::FillPortion(6)))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Row::new().align_items(Align::Center).spacing(4).width(Length::FillPortion(3)).push(Text::new("Port")).push(
                                PickList::new(&mut self.output_pick, &self.list_ports, Some(self.select_output_pick.clone()), AudioTabMsg::OutputPortChanged)
                                    .style(PickListStyle {})
                                    .width(Length::Fill),
                            ),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .align_items(Align::Center)
                        .push(
                            Button::new(&mut self.speaker_mute, if self.is_speak_mute { speaker_icon_mute() } else { speaker_icon() })
                                .on_press(AudioTabMsg::SpeakerMute)
                                .style(ButtonStyle::Transparent),
                        )
                        .push(
                            Slider::new(&mut self.output_slider, 0.0..=150.0, self.output_val, AudioTabMsg::OutputChanged)
                                .step(1.0)
                                .style(SliderStyle::Circle(10.0))
                                .width(Length::Fill),
                        )
                        .push(
                            Row::new().align_items(Align::Center).push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(Text::new(&self.output_val.to_string()).horizontal_alignment(HorizontalAlignment::Center).width(Length::Units(20)))
                                    .push(Text::new("%")),
                            ),
                        ),
                ),
        )
        .style(ContainerStyle::LightGrayCircle)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(10);
        let input_view = Container::new(
            Column::new()
                .spacing(10)
                .align_items(Align::Center)
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .push(Text::new("Headset Microphone  (Built-in Audio Analog stereo").width(Length::FillPortion(6)))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Row::new().align_items(Align::Center).spacing(4).width(Length::FillPortion(3)).push(Text::new("Port")).push(
                                PickList::new(&mut self.input_pick, &self.list_source_ports, Some(self.select_input_pick.clone()), AudioTabMsg::InputPortChanged)
                                    .style(PickListStyle {})
                                    .width(Length::Fill),
                            ),
                        ),
                )
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(
                            Button::new(&mut self.headset_mute, if self.is_headset_mute { microphone_icon_mute() } else { microphone_icon() })
                                .on_press(AudioTabMsg::MicrophoneMute)
                                .style(ButtonStyle::Transparent),
                        )
                        .push(Slider::new(&mut self.input_slider, 0.0..=150.0, self.input_val, AudioTabMsg::InputChanged).step(1.0).style(SliderStyle::Circle(10.0)).width(Length::Fill))
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .push(Text::new(&self.input_val.to_string()).horizontal_alignment(HorizontalAlignment::Center).width(Length::Units(20)))
                                .push(Text::new("%")),
                        ),
                ),
        )
        .style(ContainerStyle::LightGrayCircle)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(10);
        let notify_view = Container::new(
            Column::new()
                .spacing(10)
                .align_items(Align::Center)
                .push(Row::new().align_items(Align::Center).push(Text::new("Notification Sounds")).push(Space::with_width(Length::Fill)))
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(
                            Button::new(&mut self.notification_mute, if self.is_notification_mute { bell_icon_mute() } else { bell_icon() })
                                .on_press(AudioTabMsg::NotificationMute)
                                .style(ButtonStyle::Transparent),
                        )
                        .push(
                            Slider::new(&mut self.notification_slider, 0.0..=150.0, self.notify_val, AudioTabMsg::NotifyChanged)
                                .step(1.0)
                                .style(SliderStyle::Circle(10.0))
                                .width(Length::Fill),
                        )
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .push(Text::new(&self.notify_val.to_string()).horizontal_alignment(HorizontalAlignment::Center).width(Length::Units(20)))
                                .push(Text::new("%")),
                        ),
                ),
        )
        .style(ContainerStyle::LightGrayCircle)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(10);
        Container::new(
            Column::new()
                .spacing(10)
                .push(header("Speaker"))
                .push(output_view)
                .push(header("Microphone"))
                .push(input_view)
                .push(header("Notification Audio"))
                .push(notify_view),
        )
        .width(Length::Fill)
        .into()
    }
}
fn header(title: &str) -> Text {
    Text::new(title).size(18)
}
#[derive(Debug, Clone)]
pub enum AudioTabMsg {
    InputChanged(f64),
    OutputChanged(f64),
    NotifyChanged(f64),
    OutputPortChanged(OutputPort),
    InputPortChanged(InputPort),
    SpeakerMute,
    MicrophoneMute,
    NotificationMute,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OutputPort {
    pub port: String,
}
impl fmt::Display for OutputPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.port)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InputPort {
    pub port: String,
}
impl fmt::Display for InputPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.port)
    }
}

const ICONS: Font = Font::External {
    name: "Line Awesome",
    bytes: include_bytes!("../../../../assets/fonts/la-solid-900.woff"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string()).font(ICONS).width(Length::Units(20)).horizontal_alignment(HorizontalAlignment::Center).size(20)
}

fn speaker_icon() -> Text {
    icon('\u{f028}')
}
fn speaker_icon_mute() -> Text {
    icon('\u{f6a9}')
}
fn microphone_icon() -> Text {
    icon('\u{f130}')
}
fn microphone_icon_mute() -> Text {
    icon('\u{f131}')
}

fn bell_icon() -> Text {
    icon('\u{f0f3}')
}

fn bell_icon_mute() -> Text {
    icon('\u{f1f6}')
}
