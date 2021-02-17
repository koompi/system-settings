use super::netsettings::{NetSettings, NetSettingsMsg};
use crate::gui::styles::{buttons::ButtonStyle, rules::RuleStyle};
use iced::{button, scrollable, text_input, Align, Button, Column, Container, Element, Length, Row, Rule, Scrollable, Space, Text, TextInput};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
use libkoompi::system_settings::network::{get_accesspoints, wifi::WifiInterface, AccessPoint, Wifi};
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
}
impl WifiProperty {
    pub fn new() -> Self {
        Default::default()
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
        Self {
            ssid_vector: get_list_ssid(),
            is_shown: false,
            search_vector: get_list_ssid(),
            network_settings: NetSettings::new(),
            ..Self::default()
        }
    }
    pub fn update(&mut self, msg: WirelessMsg) {
        match msg {
            WirelessMsg::EnableWireless(value) => {
                self.is_active = value;
                if value {
                    Wifi::turn_on();
                } else {
                    Wifi::turn_off();
                }
            }
            WirelessMsg::ConnectButton(ssid) => {
                self.is_shown_passwd = !self.is_shown_passwd;
                self.ssid_vector.iter_mut().filter(|v| v.ssid.to_lowercase().contains(&ssid.to_lowercase())).for_each(|v| v.is_shown = !v.is_shown);
                println!("You cannect with {:?}", ssid);
            }
            WirelessMsg::NothingButton => {}
            WirelessMsg::ShowSettings => {
                self.is_shown = !self.is_shown;
            }
            WirelessMsg::Password(pwd) => {
                self.passwd = pwd;
            }
            WirelessMsg::SearchWifi => self.is_shown_search = !self.is_shown_search,
            WirelessMsg::SearchAction(val) => {
                self.ssid_vector = self.search_vector.iter().filter(|&v| v.ssid.to_lowercase().contains(&val.to_lowercase())).cloned().collect();
                self.input_search_val = val;
            }
            WirelessMsg::RefreshWifi => {
                self.ssid_vector = get_list_ssid();
            }
            WirelessMsg::NetSettingsMsg(msg) => {
                self.network_settings.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<WirelessMsg> {
        let is_shown_passwd = self.is_shown_passwd;
        let passwd = &self.passwd;
        let wireless_layout = Column::new()
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
                            .push(
                                Button::new(
                                    &mut wifi_prop.connect,
                                    Row::new().align_items(Align::Center).spacing(10).push(Icon::new('\u{f1e6}')).push(Text::new(if wifi_prop.status { "Connect" } else { "Disconnect" })),
                                )
                                .style(ButtonStyle::Transparent)
                                .on_press(WirelessMsg::ConnectButton(wifi_prop.ssid.clone())),
                            )
                            .push(Button::new(&mut wifi_prop.settings, Icon::new('\u{f105}')).on_press(WirelessMsg::ShowSettings).style(ButtonStyle::Transparent)),
                    )
                    .push(if wifi_prop.is_shown {
                        Container::new(TextInput::new(&mut wifi_prop.input_passwd, "Password....", &passwd, WirelessMsg::Password).padding(10)).width(Length::Fill)
                    } else {
                        Container::new(Space::with_height(Length::Units(0)))
                    })
            }))
            .width(Length::Fill);
        let scroll_content = Scrollable::new(&mut self.scroll_content).padding(20).scroller_width(4).scrollbar_width(4);
        let wifi_layout = Row::new()
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
                    .push(scroll_content.push(wireless_layout)),
            )
            .push(Rule::vertical(10).style(RuleStyle {}))
            .push(if self.is_shown {
                self.network_settings.view().map(move |msg| WirelessMsg::NetSettingsMsg(msg))
            } else {
                Space::with_width(Length::Shrink).into()
            });
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
