#![allow(dead_code)]
use crate::gui::styles::buttons::ButtonStyle;
use iced::{button, Align, Button, Checkbox, Column, Element, Length, Row, Space, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
#[derive(Debug, Default, Clone)]
pub struct BlueContent {
    is_shown: bool,
    btn_refresh: button::State,
    vector_bluetooths: Vec<(BluetoothDevType, String, BluetoothStatus)>,
    bluedata: blue_backend::BluetoothData,
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
        let mut data = blue_backend::BluetoothData::new();
        let mut blue_address = Vec::<(String, String)>::new();
        match data.get_address() {
            Ok(incoming_result) => blue_address = incoming_result.to_vec(),
            Err(e) => println!("Error : {:?}", e),
        }
        let mut init_vec_state: Vec<(BluetoothDevType, String, BluetoothStatus)> = Vec::new();
        for new_data in blue_address {
            init_vec_state.push((BluetoothDevType::Computer, format!("{} {}", new_data.0, new_data.1), BluetoothStatus::NoConnected));
        }
        Self {
            vector_bluetooths: init_vec_state,
            ..Default::default()
        }
    }
    fn get_blueaddress(&mut self) -> Vec<(String, String)> {
        let mut blue_address = Vec::<(String, String)>::new();
        match self.bluedata.get_address() {
            Ok(addresses) => blue_address = addresses.to_vec(),
            Err(e) => println!("Error: {:?}", e),
        };
        blue_address
    }
    pub fn update(&mut self, msg: BlueConentMsg) {
        match msg {
            BlueConentMsg::DevRefreshed => {
                self.bluedata.get_data().clear();
                let length = self.vector_bluetooths.len();
                if length != 0 {
                    for _ in 0..=length - 1 {
                        self.vector_bluetooths.pop();
                    }
                    for new_data in self.get_blueaddress() {
                        self.vector_bluetooths.push((BluetoothDevType::Computer, format!("{} {}", new_data.0, new_data.1), BluetoothStatus::NoConnected));
                    }
                } else {
                    {}
                }
            }
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
                    .push(Button::new(&mut self.btn_refresh, Icon::new('\u{f021}')).on_press(BlueConentMsg::DevRefreshed).style(ButtonStyle::Circular(86, 101, 115, 1.0))),
            )
            .push(self.vector_bluetooths.iter_mut().fold(Column::new().padding(10).spacing(16), |column, (b_type, b_ssid, b_status)| {
                column.push(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(4)
                        .push(
                            Icon::new(match b_type {
                                BluetoothDevType::Computer => '\u{f108}',
                                BluetoothDevType::Headphone => '\u{f3cd}',
                                BluetoothDevType::SmartPhone => '\u{f58f}',
                                BluetoothDevType::Unknown => '\u{f17c}',
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

pub mod blue_backend {
    use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
    use blurz::bluetooth_device::BluetoothDevice as Device;
    use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
    use blurz::bluetooth_session::BluetoothSession as Session;
    use std::thread;
    use std::time::Duration;
    #[derive(Default, Debug, Clone)]
    pub struct BluetoothData {
        data: Vec<(String, String)>,
        is_trusted: bool,
        is_paired: bool,
        is_connected: bool,
    }
    impl BluetoothData {
        pub fn new() -> Self {
            Self { ..Default::default() }
        }
        pub fn get_data(&mut self) -> &mut Vec<(String, String)> {
            &mut self.data
        }
        pub fn turn_on_or_off(&mut self, status: bool) -> Result<bool, Box<dyn std::error::Error>> {
            let bt_session = &Session::create_session(None)?;
            let adapter: Adapter = Adapter::init(bt_session)?;
            if status {
                adapter.set_powered(true)?;
            } else {
                adapter.set_powered(false)?;
            }
            Ok(true)
        }
        pub fn get_address(&mut self) -> Result<&Vec<(String, String)>, Box<dyn std::error::Error>> {
            let bt_session = &Session::create_session(None)?;
            let adapter: Adapter = Adapter::init(bt_session)?;
            adapter.set_powered(true)?;
            let session = DiscoverySession::create_session(&bt_session, adapter.get_id()).unwrap();
            thread::sleep(Duration::from_millis(200));
            match session.start_discovery() {
                Ok(()) => {
                    println!("discover successfully")
                }
                Err(e) => println!("Error: {:?}", e),
            }
            thread::sleep(Duration::from_millis(200));
            let devices = adapter.get_device_list().unwrap();
            println!("{} device(s) found", devices.len());
            for d in devices {
                let device = Device::new(bt_session, d);
                println!("{:?} name: {:?} is trust: {:?}", device.get_address(), device.get_name(), device.is_trusted(),);
                self.data.push((String::from(""), device.get_address()?));
                match device.get_name() {
                    Ok(name) => {
                        self.data.push((name, String::from("")));
                    }
                    Err(e) => println!("Error: {}", e),
                }
                // adapter.remove_device(device.get_id()).unwrap();
            }
            match session.stop_discovery() {
                Ok(()) => println!("Not error: "),
                Err(e) => println!("Error: {:?}", e),
            }
            Ok(&self.data)
        }
    }
}
