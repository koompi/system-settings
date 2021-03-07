#![allow(dead_code)]
use crate::gui::styles::buttons::ButtonStyle;
use async_std::task;
use async_std::task::sleep;
use iced::{button, Align, Button, Checkbox, Column, Element, Length, Row, Space, Text};
use iced_custom_widget as icw;
use icw::components::{Icon, Icons};
use libkoompi::system_settings::bluez_api_export::BluetoothSession;
use std::time::Duration;
const SCAN_DURATION: Duration = Duration::from_secs(4);
#[derive(Debug, Default, Clone)]
pub struct BlueContent {
    is_shown: bool,
    btn_refresh: button::State,
    vector_bluetooths: Vec<(BluetoothDevType, String, BluetoothStatus)>,
}

#[derive(Debug, Clone)]
pub enum BlueConentMsg {
    DevShowNameless(bool),
    DevRefreshed,
}

#[derive(Debug, Clone)]
pub enum BluetoothStatus {
    Connected,
    Connecting,
    NoConnected,
    DisConnected,
}

#[derive(Debug, Clone)]
pub enum BluetoothDevType {
    SmartPhone,
    Computer,
    Headphone,
    Unknown,
}
impl Default for BluetoothDevType {
    fn default() -> Self {
        BluetoothDevType::SmartPhone
    }
}

impl BlueContent {
    pub fn new() -> Self {
        let _simpler_code = |b_type: BluetoothDevType, b_ssid: &str, b_status: BluetoothStatus| (b_type, b_ssid.to_string(), b_status);
        let mut blue_address = Vec::<(String, String)>::new();
        let mut init_vec_state: Vec<(BluetoothDevType, String, BluetoothStatus)> = Vec::new();
        // task::block_on(async {
        //     if let Ok((_, session)) = BluetoothSession::new().await {
        //         match session.start_discovery().await {
        //             Ok(()) => {}
        //             Err(e) => println!("Error: {:?}", e),
        //         }
        //         sleep(SCAN_DURATION).await;
        //         match session.stop_discovery().await {
        //             Ok(()) => {}
        //             Err(e) => println!("Error: {:?}", e),
        //         }
        //         // Get the list of all devices which BlueZ knows about.
        //         match session.get_devices().await {
        //             Ok(list_devices) => {
        //                 println!("{:#?}", list_devices);
        //             }
        //             Err(e) => eprintln!("Error: {:?}", e),
        //         }
        //     } else {
        //         {}
        //     }
        // });
        Self {
            vector_bluetooths: init_vec_state,
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: BlueConentMsg) {
        match msg {
            BlueConentMsg::DevRefreshed => {}
            BlueConentMsg::DevShowNameless(is_show) => {
                self.is_shown = is_show;
            }
        }
    }
    pub fn view(&mut self) -> Element<BlueConentMsg> {
        let other_devices = Column::new()
            .spacing(10)
            .push(Text::new("Other Devices").size(24))
            .push(
                Row::new()
                    .padding(10)
                    .push(Checkbox::new(self.is_shown, "Show Bluetooth devices without names", BlueConentMsg::DevShowNameless))
                    .push(Space::with_width(Length::Fill))
                    .push(Button::new(&mut self.btn_refresh, Icon::new(Icons::Circle)).on_press(BlueConentMsg::DevRefreshed).style(ButtonStyle::Circular(86, 101, 115, 1.0))),
            )
            .push(self.vector_bluetooths.iter_mut().fold(Column::new().padding(10).spacing(16), |column, (b_type, b_ssid, b_status)| {
                column.push(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(4)
                        .push(
                            Icon::new(match b_type {
                                BluetoothDevType::Computer => Icons::Laptop,
                                BluetoothDevType::Headphone => Icons::Phone,
                                BluetoothDevType::SmartPhone => Icons::Phone,
                                BluetoothDevType::Unknown => Icons::Unlink,
                            })
                            .size(24),
                        )
                        .push(Text::new(b_ssid.as_str()))
                        .push(Space::with_width(Length::Fill))
                        .push(Text::new(match b_status {
                            BluetoothStatus::Connected => "Connected",
                            BluetoothStatus::Connecting => "Connecting",
                            BluetoothStatus::DisConnected => "Disconnected",
                            BluetoothStatus::NoConnected => "Not connected",
                        })),
                )
            }));
        other_devices.into()
    }
}
