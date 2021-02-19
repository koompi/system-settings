use crate::gui::styles::{containers::ContainerStyle, picklist::PickListStyle, sliders::SliderStyle};
use iced::{button, pick_list, slider, Align, Button, Column, Container, Element, Length, PickList, Row, Slider, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
use std::fmt;
#[derive(Default, Debug, Clone)]
pub struct SoundInput {
    pick_in_dev: pick_list::State<InputDevice>,
    selected_in_dev: InputDevice,
    mute_in_sound: button::State,
    is_in_muted: bool,
    slider_input: slider::State,
    input_val: f32,
    slider_input_level: slider::State,
    input_level: f32,
    is_auto_noise_suppression: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDevice {
    Internal,
    External,
}
impl Default for InputDevice {
    fn default() -> Self {
        InputDevice::Internal
    }
}
impl InputDevice {
    const ALL: [InputDevice; 2] = [InputDevice::Internal, InputDevice::External];
}

impl fmt::Display for InputDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputDevice::Internal => "Internal (HDA Intel PCH)",
                InputDevice::External => "External",
            }
        )
    }
}
#[derive(Clone, Debug)]
pub enum SoundInputMsg {
    SeletedIn(InputDevice),
    SoundInChanged(f32),
    InputLevelChanged(f32),
    AutomatedSoundSuppression(bool),
}
impl SoundInput {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn update(&mut self, msg: SoundInputMsg) {
        match msg {
            SoundInputMsg::SeletedIn(in_sound) => self.selected_in_dev = in_sound,
            SoundInputMsg::SoundInChanged(val) => self.input_val = val,
            SoundInputMsg::InputLevelChanged(val) => self.input_level = val,
            SoundInputMsg::AutomatedSoundSuppression(is_auto) => self.is_auto_noise_suppression = is_auto,
        }
    }
    pub fn view(&mut self) -> Element<SoundInputMsg> {
        let input_content = Column::new()
            .push(Container::new(Text::new("Input").size(24)))
            .spacing(10)
            .push(
                Container::new(
                    Row::new()
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Input Devices"))
                        .push(PickList::new(&mut self.pick_in_dev, &InputDevice::ALL[..], Some(self.selected_in_dev), SoundInputMsg::SeletedIn).style(PickListStyle {})),
                )
                .width(Length::Fill)
                .style(ContainerStyle::LightGrayCircle)
                .padding(10),
            )
            .push(
                Container::new(
                    Column::new().spacing(10).push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(4)
                            .push(Button::new(&mut self.mute_in_sound, Icon::new(if self.is_in_muted { '\u{f026}' } else { '\u{f028}' })))
                            .push(Slider::new(&mut self.slider_input, 0.0..=100.0, self.input_val, SoundInputMsg::SoundInChanged).style(SliderStyle::Default).step(1.0))
                            .push(Icon::new('\u{f028}')),
                    ),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(
                    Column::new().spacing(10).push(Text::new("Input Level")).push(
                        Row::new()
                            .align_items(Align::Center)
                            .push(Icon::new('\u{f192}'))
                            .spacing(10)
                            .push(Slider::new(&mut self.slider_input_level, 0.0..=100.0, self.input_level, SoundInputMsg::InputLevelChanged).style(SliderStyle::Default).step(1.0))
                            .push(Icon::new('\u{f141}')),
                    ),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(Toggler::new(self.is_auto_noise_suppression, String::from("Automatic Noise Suppression"), SoundInputMsg::AutomatedSoundSuppression))
                    .style(ContainerStyle::LightGrayCircle)
                    .padding(10),
            );
        input_content.into()
    }
}
