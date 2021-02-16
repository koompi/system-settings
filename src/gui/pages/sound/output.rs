use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle, picklist::PickListStyle, sliders::SliderStyle};
use iced::{button, pick_list, slider, Align, Button, Column, Container, Element, Length, PickList, Row, Slider, Space, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
use std::fmt;
const FONT_SIZE: u16 = 12;
#[derive(Default, Debug, Clone)]
pub struct SoundOutput {
    selected_out_dev: OutputDevice,
    pick_out_dev: pick_list::State<OutputDevice>,
    out_value: f32,
    mute_out_sound: button::State,
    is_muted: bool,
    slider_output: slider::State,
    is_boost_sound: bool,
    balance_state: slider::State,
    balance_val: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputDevice {
    Internal,
    External,
}
#[derive(Debug, Clone)]
pub enum SoundOutputMsg {
    SeletedOut(OutputDevice),
    MutedSound,
    SoundOutChanged(f32),
    EnableBoostSound(bool),
    BalanceChanged(f32),
}
impl Default for OutputDevice {
    fn default() -> Self {
        OutputDevice::Internal
    }
}
impl OutputDevice {
    const ALL: [OutputDevice; 2] = [OutputDevice::Internal, OutputDevice::External];
}

impl fmt::Display for OutputDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OutputDevice::Internal => "Internal (HDA Intel PCH)",
                OutputDevice::External => "External",
            }
        )
    }
}
impl SoundOutput {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn update(&mut self, msg: SoundOutputMsg) {
        match msg {
            SoundOutputMsg::BalanceChanged(val) => self.balance_val = val,
            SoundOutputMsg::EnableBoostSound(is_enable) => self.is_boost_sound = is_enable,
            SoundOutputMsg::MutedSound => {}
            SoundOutputMsg::SeletedOut(out) => self.selected_out_dev = out,
            SoundOutputMsg::SoundOutChanged(val) => self.out_value = val,
        }
    }
    pub fn view(&mut self) -> Element<SoundOutputMsg> {
        let output_content = Column::new()
            .spacing(10)
            .push(Text::new("Output").size(FONT_SIZE + 12))
            .push(
                Container::new(
                    Row::new().align_items(Align::Center).spacing(10).push(Text::new("Output Device").size(FONT_SIZE + 10)).push(
                        PickList::new(&mut self.pick_out_dev, &OutputDevice::ALL[..], Some(self.selected_out_dev), SoundOutputMsg::SeletedOut)
                            .text_size(14)
                            .style(PickListStyle {})
                            .width(Length::Fill),
                    ),
                )
                .width(Length::Fill)
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(
                            Row::new()
                                .push(Text::new("Output Volume").size(FONT_SIZE + 10))
                                .push(Space::with_width(Length::Fill))
                                .push(Text::new(&format!("{}%", self.out_value.to_string())).size(FONT_SIZE + 10)),
                        )
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .spacing(4)
                                .push(
                                    Button::new(&mut self.mute_out_sound, Icon::new(if self.is_muted { '\u{f026}' } else { '\u{f028}' }))
                                        .on_press(SoundOutputMsg::MutedSound)
                                        .style(ButtonStyle::Transparent),
                                )
                                .push(Slider::new(&mut self.slider_output, 0.0..=100.0, self.out_value, SoundOutputMsg::SoundOutChanged).style(SliderStyle::Circle).step(1.0).width(Length::Fill))
                                .push(Icon::new('\u{f027}')),
                        ),
                )
                .width(Length::Fill)
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Volume Boost").size(FONT_SIZE + 10))
                        .push(Space::with_width(Length::Fill))
                        .push(Toggler::new(self.is_boost_sound, String::from(""), SoundOutputMsg::EnableBoostSound)),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(if self.is_boost_sound {
                Container::new(Text::new("If the volume is lounder than 100%, it may distort audio and be harmdul to your speaker").size(FONT_SIZE + 8)).padding(10)
            } else {
                Container::new(Space::with_height(Length::Units(0)))
            })
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(Text::new("Left/Right Balance").size(FONT_SIZE + 10))
                        .push(Slider::new(&mut self.balance_state, 0.0..=100.0, self.balance_val, SoundOutputMsg::BalanceChanged).style(SliderStyle::Default).step(1.0))
                        .push(Row::new().push(Text::new("Left").size(FONT_SIZE + 8)).push(Space::with_width(Length::Fill)).push(Text::new("Right").size(FONT_SIZE + 8))), // .push(),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            );
        output_content.into()
    }
}
