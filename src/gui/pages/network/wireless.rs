use super::netsettings::{NetSettings, NetSettingsMsg};
use crate::gui::styles::{buttons::ButtonStyle, rules::RuleStyle};
use iced::{button, scrollable, Align, Button, Column, Container, Element, Length, Row, Rule, Scrollable, Space, Text};
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
    ssid_vector: Vec<(button::State, bool, char, String)>,
    wifi_interface: Wifi,
    scroll_content: scrollable::State,
}

impl Wireless {
    pub fn new() -> Self {
        let ssid_gen = |btn: button::State, secure: bool, icon: char, ssid: String| (btn, secure, icon, ssid);
        let ssid_info = get_accesspoints();
        let mut initial_list: Vec<(button::State, bool, char, String)> = Vec::new();
        match ssid_info {
            Ok(data) => {
                for accesspoint in data {
                    initial_list.push(ssid_gen(button::State::new(), true, '\u{f1eb}', accesspoint.ssid))
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
        Self {
            ssid_vector: initial_list,
            is_shown: false,
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
            WirelessMsg::NothingButton => {}
            WirelessMsg::ShowSettings => {
                self.is_shown = !self.is_shown;
            }
            WirelessMsg::NetSettingsMsg(msg) => {
                self.network_settings.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<WirelessMsg> {
        println!("size of vector ssid: {}", self.ssid_vector.len());
        // .push(Toggler::new(
        //     self.is_active,
        //     String::from("Wire Network Adapter"),
        //     WirelessMsg::EnableWireless,
        // ))
        let wireless_layout = Column::new()
            .push(self.ssid_vector.iter_mut().fold(Column::new().width(Length::Fill).spacing(4), |column, (state, status, _icon, ssid)| {
                column.push(
                    Row::new()
                        .align_items(Align::Center)
                        .padding(10)
                        .spacing(8)
                        .push(if *status { Icon::new('\u{f3ed}').size(16) } else { Icon::new('\u{f09c}').size(16) })
                        .push(Icon::new('\u{f1eb}').size(24))
                        .push(Text::new(ssid.as_str()).size(16))
                        .push(Space::with_width(Length::Fill))
                        .push(Button::new(state, Icon::new('\u{f105}')).on_press(WirelessMsg::ShowSettings).style(ButtonStyle::Transparent)),
                )
            }))
            .width(Length::Fill)
            .height(Length::Fill);
        let scroll_content = Scrollable::new(&mut self.scroll_content).padding(20).scroller_width(4).scrollbar_width(4);
        let wifi_layout = Row::new()
            .push(
                Column::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .push(Toggler::new(self.is_active, String::from("Wireless Network Adapter"), WirelessMsg::EnableWireless))
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
}
