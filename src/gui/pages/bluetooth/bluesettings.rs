use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle, textinput::InputStyle};
use iced::{button, text_input, Align, Button, Column, Container, Element, HorizontalAlignment, Length, Text, TextInput, VerticalAlignment};
use iced_custom_widget as icw;
use icw::components::Icon;
#[derive(Default, Debug, Clone)]
pub struct BluetoothSettings {
    connected_host: text_input::State,
    connected_host_val: String,
    disconn_btn: button::State,
    ignore_dev: button::State,
    send_file: button::State,
    hide_btn: button::State,
}
#[derive(Debug, Clone)]
pub enum BluetoothSettingsMsg {
    BluetothNameChanged(String),
    Disconnected,
    Ignoranced,
    SendFile,
    HideSettings,
    SubmitChanged,
}

impl BluetoothSettings {
    pub fn new() -> Self {
        Self {
            connected_host_val: String::from("sna-koompi"),
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: BluetoothSettingsMsg) {
        match msg {
            BluetoothSettingsMsg::BluetothNameChanged(val) => {
                self.connected_host_val = val;
            }
            BluetoothSettingsMsg::Disconnected => {}
            BluetoothSettingsMsg::Ignoranced => {}
            BluetoothSettingsMsg::SendFile => {}
            BluetoothSettingsMsg::HideSettings => {}
            BluetoothSettingsMsg::SubmitChanged => {
                println!("data submit");
            }
        }
    }
    pub fn view(&mut self) -> Element<BluetoothSettingsMsg> {
        let blue_settings_layout = Column::new()
            .spacing(10)
            .padding(10)
            .height(Length::Fill)
            .push(Button::new(&mut self.hide_btn, Icon::new('\u{f104}')).on_press(BluetoothSettingsMsg::HideSettings).style(ButtonStyle::Circular(86, 101, 115, 1.0)))
            .push(Column::new().align_items(Align::Center).width(Length::Fill).push(Text::new("Connected Host Bluetooth").size(16)))
            .push(
                TextInput::new(&mut self.connected_host, &self.connected_host_val, "", BluetoothSettingsMsg::BluetothNameChanged)
                    .on_submit(BluetoothSettingsMsg::SubmitChanged)
                    .padding(6)
                    .style(InputStyle::InkBorder),
            )
            .push(
                Button::new(
                    &mut self.disconn_btn,
                    Text::new("Disconnect").width(Length::Fill).horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::Disconnected),
            )
            .push(
                Button::new(
                    &mut self.ignore_dev,
                    Text::new("Ignore this device").width(Length::Fill).horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::Ignoranced),
            )
            .push(
                Button::new(
                    &mut self.send_file,
                    Text::new("Send Files").width(Length::Fill).horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::SendFile),
            );
        Container::new(blue_settings_layout).center_x().center_y().width(Length::FillPortion(1)).style(ContainerStyle::LightGrayCircle).into()
    }
}
