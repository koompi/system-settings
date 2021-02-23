use crate::gui::styles::{containers::ContainerStyle, picklist::PickListStyle, sliders::SliderStyle};
use iced::{button, pick_list, slider, Align, Button, Column, Container, Element, HorizontalAlignment, Length, PickList, Row, Slider, Space, Text, VerticalAlignment};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
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
}
impl AudioTab {
    pub fn new() -> Self {
        AudioTab::default()
    }
    pub fn update(&mut self, msg: AudioTabMsg) {
        match msg {
            AudioTabMsg::InputChanged(val) => self.input_val = val,
            AudioTabMsg::OutputChanged(val) => self.output_val = val,
            AudioTabMsg::NotifyChanged(val) => self.notify_val = val,
            AudioTabMsg::OutputPortChanged(port) => self.select_output_pick = port,
            AudioTabMsg::InputPortChanged(port) => self.select_input_pick = port,
            AudioTabMsg::SpeakerMute(is_mute) => self.is_speak_mute = is_mute,
            AudioTabMsg::MicrophoneMute(is_mute) => self.is_headset_mute = is_mute,
            AudioTabMsg::NotificationMute(is_mute) => self.is_notification_mute = is_mute,
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
                            Row::new().align_items(Align::Center).spacing(4).width(Length::FillPortion(2)).push(Text::new("Port")).push(
                                PickList::new(&mut self.output_pick, &OutputPort::ALL[..], Some(self.select_output_pick), AudioTabMsg::OutputPortChanged)
                                    .style(PickListStyle {})
                                    .width(Length::Fill),
                            ),
                        ),
                )
                .push(
                    Row::new()
                        .spacing(10)
                        .align_items(Align::Center)
                        .push(Slider::new(&mut self.output_slider, 0.0..=150.0, self.output_val, AudioTabMsg::OutputChanged).style(SliderStyle::Default).width(Length::Fill))
                        .push(Text::new(&self.output_val.to_string())),
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
                            Row::new().align_items(Align::Center).spacing(4).width(Length::FillPortion(2)).push(Text::new("Port")).push(
                                PickList::new(&mut self.input_pick, &InputPort::ALL[..], Some(self.select_input_pick), AudioTabMsg::InputPortChanged)
                                    .style(PickListStyle {})
                                    .width(Length::Fill),
                            ),
                        ),
                )
                .push(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Slider::new(&mut self.input_slider, 0.0..=150.0, self.input_val, AudioTabMsg::InputChanged).style(SliderStyle::Default).width(Length::Fill))
                        .push(Text::new(format!("{}%", &self.input_val.to_string())).width(Length::Units(20)).horizontal_alignment(HorizontalAlignment::Center)),
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
                        .push(Slider::new(&mut self.notification_slider, 0.0..=150.0, self.notify_val, AudioTabMsg::NotifyChanged).style(SliderStyle::Default).width(Length::Fill))
                        .push(Text::new(&self.notify_val.to_string())),
                ),
        )
        .style(ContainerStyle::LightGrayCircle)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(10);
        Container::new(
            Column::new()
                .spacing(10)
                .push(Text::new("Playback Devices"))
                .push(output_view)
                .push(Text::new("Recording Devices"))
                .push(input_view)
                .push(Text::new("Playback Streams"))
                .push(notify_view),
        )
        .width(Length::Fill)
        .into()
    }
}
#[derive(Debug, Clone)]
pub enum AudioTabMsg {
    InputChanged(f64),
    OutputChanged(f64),
    NotifyChanged(f64),
    OutputPortChanged(OutputPort),
    InputPortChanged(InputPort),
    SpeakerMute(bool),
    MicrophoneMute(bool),
    NotificationMute(bool),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputPort {
    Headphone,
    Speakers,
}

impl OutputPort {
    const ALL: [OutputPort; 2] = [OutputPort::Headphone, OutputPort::Speakers];
}
impl fmt::Display for OutputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                OutputPort::Headphone => "Headphone",
                OutputPort::Speakers => "Speakers",
            }
        )
    }
}
impl Default for OutputPort {
    fn default() -> Self {
        OutputPort::Speakers
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputPort {
    HeadsetMicrophone,
    Microphone,
}
impl InputPort {
    const ALL: [InputPort; 2] = [InputPort::HeadsetMicrophone, InputPort::Microphone];
}

impl fmt::Display for InputPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                InputPort::HeadsetMicrophone => "HeadsetMicrophone",
                InputPort::Microphone => "Microphone",
            }
        )
    }
}

impl Default for InputPort {
    fn default() -> Self {
        InputPort::Microphone
    }
}
