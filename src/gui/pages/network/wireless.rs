use super::netsettings::{NetSettings, NetSettingsMsg};
use crate::gui::styles::{buttons::ButtonStyle, rules::RuleStyle};
use async_std::task;
use iced::{button, scrollable, text_input, Align, Button, Column, Container, Element, Length, Row, Rule, Scrollable, Space, Text, TextInput};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
use libkoompi::system_settings::network::{get_accesspoints, wifi::Connectivity, wifi::WifiInterface, Wifi};
use std::sync::mpsc;
#[derive(Default, Debug, Clone)]
pub struct Wireless {
    is_active: bool,
    is_shown: bool,
    status: String,
    security: Option<String>,
    ssid: String,
    network_settings: NetSettings,
    ssid_vector: Vec<WifiProperty>,
    search_vector: Vec<WifiProperty>,
    push_section: Vec<WifiProperty>,
    wifi_interface: Wifi,
    scroll_content: scrollable::State,
    search_wifi: button::State,
    refresh_wifi: button::State,
    is_shown_search: bool,
    input_search: text_input::State,
    input_search_val: String,
    connect_wifi: button::State,
    connect_status: String,
    is_connect: bool,
    is_shown_passwd: bool,
    is_found: bool,
    passwd: String,
}
#[derive(Default, Debug, Clone)]
struct WifiProperty {
    pub detail: button::State,
    pub status: bool,
    pub settings_icon: char,
    pub settings: button::State,
    pub ssid: String,
    pub connect: button::State,
    pub input_passwd: text_input::State,
    pub show_passwd_btn: button::State,
    pub is_shown: bool,
    pub password: String,
    pub is_pressed: bool,
    pub is_disable: bool,
    pub push_to_section: bool,
    pub number_clicked: usize,
    pub is_connecting: bool,
    pub button_string: String,
    pub con_state: ConnectionState,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Started,
    Activated,
    Deactivated,
    Activating,
    Deactivating,
    Finished,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkState {
    Known,
    Unknown,
}
impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::Started
    }
}
impl WifiProperty {
    pub fn new() -> Self {
        Self {
            button_string: "Connect".to_string(),
            ..Default::default()
        }
    }
}
fn get_list_ssid() -> Vec<WifiProperty> {
    let ssid_info = get_accesspoints();
    let mut initial_list: Vec<WifiProperty> = Vec::new();
    match ssid_info {
        Ok(data) => {
            for accesspoint in data {
                let mut wifi_props: WifiProperty = WifiProperty::new();
                wifi_props.ssid = accesspoint.ssid;
                wifi_props.detail = button::State::new();
                wifi_props.settings_icon = '\u{f1eb}';
                wifi_props.settings = button::State::new();
                wifi_props.connect = button::State::new();
                wifi_props.input_passwd = text_input::State::new();
                wifi_props.status = true;
                wifi_props.show_passwd_btn = button::State::new();
                initial_list.push(wifi_props);
                // initial_button::State::new(), true, '\u{f1eb}', accesspoint.ssid, button::State::new()list.push(button::State::new(), true, '\u{f1eb}', accesspoint.ssid, button::State::new());
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
    initial_list
}
impl Wireless {
    pub fn new() -> Self {
        // let result = task::spawn(async {
        //     let handle = task::spawn(async move {
        //         return get_list_ssid();
        //     });
        //     handle.await;
        // });
        // println!("Result: {:?}", result);
        Self {
            ssid_vector: get_list_ssid(),
            is_shown: false,
            is_active: match Wifi::is_wifi_enabled() {
                Ok(status) => status,
                Err(e) => {
                    println!("Error : {:?}", e);
                    false
                }
            },
            search_vector: get_list_ssid(),
            network_settings: NetSettings::new(),
            is_found: true,
            ..Self::default()
        }
    }
    pub fn update(&mut self, msg: WirelessMsg) {
        match msg {
            WirelessMsg::EnableWireless(value) => {
                self.is_active = value;
                if value {
                    match Wifi::turn_on() {
                        Ok(()) => {}
                        Err(e) => println!("Error : {:?}", e),
                    }
                } else {
                    match Wifi::turn_off() {
                        Ok(()) => {}
                        Err(e) => println!("Error : {:?}", e),
                    }
                }
            }
            WirelessMsg::ConnectButton(ssid) => {
                self.is_shown_passwd = !self.is_shown_passwd;
                let mut new_data = self.ssid_vector.to_owned();
                let mut is_active: ConnectionState = ConnectionState::default();
                let mut is_deactive: ConnectionState = ConnectionState::default();
                let new_owner = self.ssid_vector.iter_mut().filter(|v| v.ssid.to_lowercase().contains(&ssid.to_lowercase()));
                self.push_section.iter_mut().filter(|v| v.ssid == ssid).for_each(|wifi_object| {
                    wifi_object.button_string = "Connect".to_string();
                    wifi_object.push_to_section = false;
                    wifi_object.con_state = ConnectionState::Deactivated;
                    is_deactive = wifi_object.con_state.clone();
                    let (tx, rv) = mpsc::channel();
                    let ssid_clone = wifi_object.ssid.clone();
                    task::spawn(async move {
                        let handle = task::spawn(async move {
                            // test(s, p);
                            let result = match Wifi::disconnect(ssid_clone) {
                                Ok(status) => status,
                                Err(e) => {
                                    println!("Error : {:?}", e);
                                    false
                                }
                            };
                            result
                        });
                        let result: bool = handle.await;
                        match tx.send(result) {
                            Ok(()) => {}
                            Err(e) => println!("Error : {:?}", e),
                        }
                    });
                    match rv.recv() {
                        Ok(status) => println!("Disconnect status: {}", status),
                        Err(e) => println!("Error : {:?}", e),
                    };
                });
                for v in new_owner {
                    match v.con_state {
                        ConnectionState::Activated => {
                            let ssid_clone = ssid.clone();
                            let join_handler = std::thread::spawn(move || match Wifi::disconnect(ssid_clone) {
                                Ok(_is_disable) => {}
                                Err(e) => println!("Error: {:?}", e),
                            });
                            match join_handler.join() {
                                Ok(()) => {}
                                Err(e) => println!("{:?}", e),
                            }
                            v.button_string = String::from("Connect");
                            v.con_state = ConnectionState::Started;
                        }
                        ConnectionState::Started => {
                            v.is_shown = !v.is_shown;
                            v.is_pressed = !v.is_pressed;
                            v.number_clicked += 1;
                            if v.number_clicked <= 1 {
                                v.is_disable = !v.is_disable;
                                v.input_passwd.focus();
                            // we should do the actual count on the connection that open.
                            } else {
                                let s = ssid.clone();
                                let p = v.password.clone();
                                let (tx, rx): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();
                                let handler = task::spawn(async move {
                                    let handle = task::spawn(async move {
                                        // test(s, p);
                                        let result = match Wifi::connect(s, p) {
                                            Ok(status) => status,
                                            Err(e) => {
                                                println!("Error : {:?}", e);
                                                false
                                            }
                                        };
                                        result
                                    });
                                    let result: bool = handle.await;
                                    match tx.send(result) {
                                        Ok(()) => {}
                                        Err(e) => println!("Error : {:?}", e),
                                    }
                                });
                                println!("Task state: {:?}", handler.task());
                                match rx.recv() {
                                    Ok(data) => {
                                        if data {
                                            v.con_state = ConnectionState::Activated;
                                            v.push_to_section = true;
                                            self.is_connect = true;
                                            is_deactive = ConnectionState::Started;
                                            println!("Connection established......");
                                        } else {
                                            {}
                                        }
                                    }
                                    Err(e) => {
                                        println!("Error: {:?}", e)
                                    }
                                }
                                println!("Run after receving message");
                                is_active = v.con_state.clone();
                                new_data.iter_mut().for_each(|v| v.button_string = "Connect".to_string());
                                if v.con_state == ConnectionState::Activated {
                                    v.button_string = String::from("Disconnect");
                                } else {
                                    {}
                                }
                                v.number_clicked = 0;
                            }
                            v.password = "".to_string();
                        }
                        _ => {}
                    }
                }
                if is_active == ConnectionState::Activated {
                    let mut is_active_con = String::new();
                    self.ssid_vector.iter_mut().for_each(|v| {
                        if v.push_to_section {
                            is_active_con = v.ssid.clone();
                        } else {
                            {}
                        }
                        if v.is_shown == true {
                            v.is_shown = false;
                            v.is_disable = false;
                        } else {
                            {}
                        }
                    });
                    self.ssid = is_active_con;
                    self.push_section = self.ssid_vector.to_vec();
                    self.ssid_vector.retain(|v| v.push_to_section == false);
                } else {
                    {}
                }
                // Operaton when we have a known section

                if is_deactive == ConnectionState::Deactivated {
                    let (transmitter, reciever) = mpsc::channel();
                    let handler = std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(4000));
                        let list_network = get_list_ssid();
                        match transmitter.send(list_network) {
                            Ok(()) => {}
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                    });
                    self.ssid_vector = reciever.recv().unwrap();
                    match handler.join() {
                        Ok(()) => {}
                        Err(e) => println!("ERror: {:?}", e),
                    }
                    self.is_connect = false;
                    self.push_section = Vec::new();
                    self.ssid = String::from("");
                } else {
                    {}
                }
            }
            WirelessMsg::NothingButton => {}
            WirelessMsg::ShowSettings => {
                self.is_shown = !self.is_shown;
            }
            WirelessMsg::Password(pwd) => {
                self.ssid_vector.iter_mut().filter(|v| v.is_pressed == true && v.input_passwd.is_focused()).for_each(|v| {
                    if pwd.len() >= 8 {
                        v.is_disable = false;
                    } else {
                        v.is_disable = true;
                    }
                    v.password = pwd.clone();
                });
            }
            WirelessMsg::SearchWifi => {
                self.input_search.focus();
                self.is_shown_search = !self.is_shown_search;
            }
            WirelessMsg::SearchAction(val) => {
                let data: Vec<WifiProperty> = self.search_vector.iter().filter(|&v| v.ssid.to_lowercase().contains(&val.to_lowercase())).cloned().collect();
                let is_found = self.search_vector.iter().any(|v| v.ssid.to_lowercase() == val.to_lowercase());
                self.is_found = is_found;
                self.ssid_vector = data;
                self.input_search_val = val;
            }
            WirelessMsg::RefreshWifi => {
                self.ssid_vector = get_list_ssid().iter().filter(|v| v.ssid != self.ssid).cloned().collect();
                self.search_vector = (*self.ssid_vector).to_vec();
            }
            WirelessMsg::NetSettingsMsg(msg) => {
                self.network_settings.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<WirelessMsg> {
        let mut counter: usize = 0;
        let wireless_layout = Column::new()
            .push(if self.is_connect {
                Column::new()
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(self.push_section.iter_mut().fold(Column::new().height(Length::Shrink).width(Length::Fill).spacing(4), |column, wifi_prop| {
                        column.push(if wifi_prop.push_to_section {
                            Row::new()
                                .width(Length::Fill)
                                .align_items(Align::Center)
                                .spacing(8)
                                .padding(10)
                                .push(if wifi_prop.status { Icon::new('\u{f3ed}').size(16) } else { Icon::new('\u{f09c}').size(16) })
                                .push(Icon::new('\u{f1eb}').size(24))
                                .push(Text::new(wifi_prop.ssid.as_str()).size(16))
                                .push(Space::with_width(Length::Fill))
                                .push(if wifi_prop.is_disable {
                                    Button::new(&mut wifi_prop.connect, Row::new().align_items(Align::Center).spacing(10).push(Icon::new('\u{f1e6}')).push(Text::new(&wifi_prop.button_string))).style(ButtonStyle::Transparent)
                                } else {
                                    Button::new(&mut wifi_prop.connect, Row::new().align_items(Align::Center).spacing(10).push(Icon::new('\u{f1e6}')).push(Text::new(&wifi_prop.button_string)))
                                        .style(ButtonStyle::Transparent)
                                        .on_press(WirelessMsg::ConnectButton(wifi_prop.ssid.clone()))
                                })
                                .push(Button::new(&mut wifi_prop.settings, Icon::new('\u{f105}')).on_press(WirelessMsg::ShowSettings).style(ButtonStyle::Transparent))
                        } else {
                            counter += 1;
                            Row::new().width(Length::Units(0)).height(Length::Units(0))
                        })
                    }))
                    .push(Rule::horizontal(10))
            } else {
                Column::new()
            })
            .push(self.ssid_vector.iter_mut().fold(Column::new().width(Length::Fill).spacing(4), |column, wifi_prop| {
                column
                    .push(
                        Row::new()
                            .align_items(Align::Center)
                            .padding(10)
                            .spacing(8)
                            .push(if wifi_prop.status { Icon::new('\u{f3ed}').size(16) } else { Icon::new('\u{f09c}').size(16) })
                            .push(Icon::new('\u{f1eb}').size(24))
                            .push(Text::new(wifi_prop.ssid.as_str()).size(16))
                            .push(Space::with_width(Length::Fill))
                            .push(if wifi_prop.is_disable {
                                Button::new(&mut wifi_prop.connect, Row::new().align_items(Align::Center).spacing(10).push(Icon::new('\u{f1e6}')).push(Text::new(&wifi_prop.button_string))).style(ButtonStyle::Transparent)
                            } else {
                                Button::new(&mut wifi_prop.connect, Row::new().align_items(Align::Center).spacing(10).push(Icon::new('\u{f1e6}')).push(Text::new(&wifi_prop.button_string)))
                                    .style(ButtonStyle::Transparent)
                                    .on_press(WirelessMsg::ConnectButton(wifi_prop.ssid.clone()))
                            })
                            .push(Button::new(&mut wifi_prop.settings, Icon::new('\u{f105}')).on_press(WirelessMsg::ShowSettings).style(ButtonStyle::Transparent)),
                    )
                    .push(if wifi_prop.is_shown {
                        Container::new(
                            TextInput::new(&mut wifi_prop.input_passwd, "Password....", &wifi_prop.password, WirelessMsg::Password)
                                .on_submit(WirelessMsg::ConnectButton(wifi_prop.ssid.clone()))
                                .padding(6),
                        )
                        .width(Length::Fill)
                    } else {
                        Container::new(Space::with_height(Length::Units(0)))
                    })
            }))
            .width(Length::Fill);
        let scroll_content = Scrollable::new(&mut self.scroll_content).scroller_width(4).scrollbar_width(4);
        let wifi_layout: Element<_> = Row::new()
            .push(
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(
                        Column::new()
                            .spacing(10)
                            .width(Length::Fill)
                            .align_items(Align::Center)
                            .push(
                                Row::new()
                                    .align_items(Align::Center)
                                    .push(Text::new("Wireless Network Adapter").size(24))
                                    .push(Space::with_width(Length::Fill))
                                    .spacing(10)
                                    .push(Button::new(&mut self.search_wifi, Icon::new('\u{f002}')).style(ButtonStyle::Transparent).on_press(WirelessMsg::SearchWifi))
                                    .push(Button::new(&mut self.refresh_wifi, Icon::new('\u{f2f9}')).style(ButtonStyle::Transparent).on_press(WirelessMsg::RefreshWifi))
                                    .push(Toggler::new(self.is_active, String::from(""), WirelessMsg::EnableWireless).width(Length::Shrink)),
                            )
                            .push(if self.is_shown_search {
                                Container::new(TextInput::new(&mut self.input_search, "Search...", &self.input_search_val, WirelessMsg::SearchAction).width(Length::Fill).padding(6)).width(Length::Fill)
                            } else {
                                Container::new(Space::with_height(Length::Units(0)))
                            }),
                    )
                    .spacing(10)
                    .push(scroll_content.push(wireless_layout))
                    .push(if self.is_found {
                        Container::new(Space::with_height(Length::Units(0)))
                    } else {
                        Container::new(Text::new("No Connection Found"))
                    }),
            )
            .push(Rule::vertical(10).style(RuleStyle {}))
            .push(if self.is_shown {
                self.network_settings.view().map(move |msg| WirelessMsg::NetSettingsMsg(msg))
            } else {
                Space::with_width(Length::Shrink).into()
            })
            .into();
        Container::new(wifi_layout).center_x().center_y().width(Length::Fill).height(Length::Fill).into()
    }
}
#[derive(Debug, Clone)]
pub enum WirelessMsg {
    EnableWireless(bool),
    NothingButton,
    ShowSettings,
    NetSettingsMsg(NetSettingsMsg),
    RefreshWifi,
    SearchWifi,
    SearchAction(String),
    ConnectButton(String),
    Password(String),
}
