use super::netsettings::{NetSettings, NetSettingsMsg};
use super::wire::{Wire, WireMsg};
use super::wireless::{Wireless, WirelessMsg};
use crate::gui::styles::containers::ContainerStyle;
use iced::{scrollable, Align, Column, Container, Element, Length, Row, Rule, Scrollable, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Tab;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}
#[derive(Default, Debug, Clone)]
pub struct NetworkPage {
    choice: Choice,
    wireless: Wireless,
    wire: Wire,
    network: NetSettings,
    is_active: bool,
    scroll_content: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum NetMessage {
    TabSelect(Choice),
    WirelessMsg(WirelessMsg),
    WireMsg(WireMsg),
    ToggleChange(bool),
    NetSettingsMsg(NetSettingsMsg),
}
impl NetworkPage {
    pub fn new() -> Self {
        Self {
            network: NetSettings::new(),
            wireless: Wireless::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: NetMessage) {
        match message {
            NetMessage::TabSelect(select) => {
                self.choice = select;
            }
            NetMessage::WirelessMsg(msg) => {
                self.wireless.update(msg);
            }
            NetMessage::ToggleChange(is_active) => {
                self.is_active = is_active;
            }
            NetMessage::WireMsg(msg) => {
                self.wire.update(msg);
            }
            NetMessage::NetSettingsMsg(msg) => {
                self.network.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<NetMessage> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(Tab::new(Choice::A, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f796}', "Ethernet")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::B, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f1eb}', "Wireless")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::C, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f6ff}', "DSL")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::D, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f3ed}', "VPN")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::E, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f7ba}', "System Proxy")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::F, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f7b9}', "Application Proxy")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::G, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f0c1}', "Personal Hotspot")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::H, Some(self.choice), NetMessage::TabSelect, tab_content('\u{f05a}', "Network Details")).width(Length::Fill).height(Length::Units(50)));
        let contnet = Column::new().height(Length::Fill).align_items(Align::Center).padding(20).push(match self.choice {
            Choice::A => self.wire.view().map(move |msg| NetMessage::WireMsg(msg)),
            Choice::B => self.wireless.view().map(move |msg| NetMessage::WirelessMsg(msg)),
            Choice::C => self.network.view().map(move |msg| NetMessage::NetSettingsMsg(msg)),
            Choice::D => Text::new("Content D").into(),
            Choice::F => Text::new("Content F").into(),
            Choice::G => Text::new("Content G").into(),
            Choice::E => Text::new("Content E").into(),
            Choice::H => Text::new("Content H").into(),
        });
        let netsidebar_scroll = Scrollable::new(&mut self.scroll_content).push(row).padding(10).scrollbar_width(4).scroller_width(4);
        let whole_content: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Container::new(netsidebar_scroll.height(Length::Fill)).style(ContainerStyle::White).width(Length::FillPortion(4)).height(Length::Fill))
            .push(Rule::vertical(10))
            .push(
                Container::new(contnet.height(Length::Fill)).width(Length::FillPortion(9)).height(Length::Fill).style(ContainerStyle::White), // .padding(10),
            )
            .into();
        let container = Container::new(whole_content).width(Length::Fill).center_x().center_y();
        Container::new(container).style(ContainerStyle::LightGray).width(Length::Fill).height(Length::Fill).padding(10).center_x().center_y().into()
    }
}
fn tab_content<'a>(unicode: char, name: &str) -> Row<'a, NetMessage> {
    Row::new().push(Icon::new(unicode).size(24)).push(Text::new(name).size(16)).align_items(Align::Center).spacing(8)
}
