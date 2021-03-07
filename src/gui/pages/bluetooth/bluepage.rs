use super::blue_content::{BlueConentMsg, BlueContent};
use super::bluesettings::{BluetoothSettings, BluetoothSettingsMsg};
use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle, rules::RuleStyle, textinput::InputStyle};
use iced::{button, scrollable, text_input, Align, Button, Column, Container, Element, Length, Row, Rule, Scrollable, Space, Text, TextInput};
use iced_custom_widget as icw;
use icw::components::{Icon, Icons, Toggler};
#[derive(Default, Debug, Clone)]
pub struct BluetoothPage {
    is_enable: bool,
    is_allowed: bool,
    is_input: bool,
    is_shown_settings: bool,
    edit_dev: button::State,
    show_settings: button::State,
    refresh: button::State,
    device_name: String,
    dev_name: text_input::State,
    dev_name_val: String,
    bluetooth_settings: BluetoothSettings,
    scroll_area: scrollable::State,
    content: BlueContent,
}

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
    BlueContentMsg(BlueConentMsg),
    DevEdited,
    DevEditedVal(String),
    DevEditedSubmmit,
    DevEnabled(bool),
    DevAllowed(bool),
    DevSettingsShown,
    CloseApp,
    // Escape,
    BluetoothSettingsMsg(BluetoothSettingsMsg),
    // WindowResize((u32, u32)),
    // FileDrop(std::path::PathBuf),
}
impl BluetoothPage {
    pub fn new() -> Self {
        Self {
            content: BlueContent::new(),
            bluetooth_settings: BluetoothSettings::new(),
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: BluetoothMessage) {
        match msg {
            BluetoothMessage::BlueContentMsg(msg) => self.content.update(msg),
            BluetoothMessage::BluetoothSettingsMsg(msg) => self.bluetooth_settings.update(msg),
            BluetoothMessage::DevEnabled(is_enable) => {
                self.is_enable = is_enable;
            }
            BluetoothMessage::DevSettingsShown => self.is_shown_settings = !self.is_shown_settings,
            BluetoothMessage::DevAllowed(is_allow) => self.is_allowed = is_allow,
            _ => {}
        }
    }
    pub fn view(&mut self) -> Element<BluetoothMessage> {
        let inner_layout = Container::new(
            Column::new()
                .spacing(10)
                .push(
                    Row::new()
                        .push(Row::new().width(Length::FillPortion(1)).align_items(Align::Center).spacing(4).push(Text::new(&self.device_name)).push(if self.is_input {
                            Row::new().push(
                                TextInput::new(&mut self.dev_name, "", &self.dev_name_val, BluetoothMessage::DevEditedVal)
                                    .on_submit(BluetoothMessage::DevEditedSubmmit)
                                    .padding(6)
                                    .style(InputStyle::InkBorder),
                            )
                        } else {
                            Row::new().push(Button::new(&mut self.edit_dev, Icon::new(Icons::Edit)).on_press(BluetoothMessage::DevEdited).style(ButtonStyle::Transparent))
                        }))
                        .push(Toggler::new(self.is_enable, String::from(""), BluetoothMessage::DevEnabled).width(Length::FillPortion(1))),
                )
                .push(Rule::horizontal(10).style(RuleStyle {}))
                .push(if self.is_enable {
                    Row::new()
                        .push(Text::new("Allow other Bluetooth devices to find this device"))
                        .push(Toggler::new(self.is_allowed, String::from(""), BluetoothMessage::DevAllowed))
                } else {
                    Row::new().push(Text::new("Enable Bluetooth for devices (Mouse, Keyboard, Headphone)"))
                }),
        )
        .width(Length::Fill)
        .padding(10)
        .style(ContainerStyle::LightGrayCircle);
        let know_devices = Column::new().spacing(10).push(Text::new("My Devices").size(24)).push(
            Column::new().padding(10).width(Length::Fill).height(Length::Shrink).push(
                Row::new().spacing(6).push(Icon::new(Icons::DiceFive)).push(Text::new("Linux")).push(Space::with_width(Length::Fill)).push(
                    Row::new().align_items(Align::Center).spacing(4).push(Text::new("Not Connected")).push(
                        Button::new(&mut self.show_settings, Icon::new(Icons::ArrowRight))
                            .on_press(BluetoothMessage::DevSettingsShown)
                            .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                    ),
                ),
            ),
        );
        let content_list = self.content.view().map(move |msg| BluetoothMessage::BlueContentMsg(msg));
        let scroll_conent = Scrollable::new(&mut self.scroll_area)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .push(Column::new().spacing(20).push(inner_layout).push(if self.is_enable { Column::new().push(know_devices).push(content_list) } else { Column::new() }));
        let embbeded_layout = Row::new().width(Length::Fill).height(Length::Fill).push(scroll_conent.padding(10).scroller_width(4).scrollbar_width(4)).push(if self.is_shown_settings {
            self.bluetooth_settings.view().map(move |msg| BluetoothMessage::BluetoothSettingsMsg(msg))
        } else {
            Space::with_width(Length::Shrink).into()
        });
        let inner_container = Container::new(embbeded_layout).style(ContainerStyle::White).padding(10);
        Container::new(inner_container).padding(10).width(Length::Fill).height(Length::Fill).style(ContainerStyle::LightGray).into()
    }
}
