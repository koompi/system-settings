use super::super::styles::{CustomButton, CustomContainer, CustomSelect};
use iced::{
    pick_list, text_input, Align, Button, Checkbox, Color, Column, Container, Element,
    HorizontalAlignment, Length, PickList, Row, Rule, Space, Svg, Text, TextInput,
};
#[derive(Debug, Default, Clone)]
pub struct NetworkPage {
    search: text_input::State,
    con_name: text_input::State,
    value: String,
}
#[derive(Debug, Clone)]
pub enum NetMessage {
    OnSearchWif(String),
    OnConnectName(String),
}
#[derive(Debug, Clone)]
pub enum Control {
    GeneralCon,
    Wifi,
    Wifi_Secure,
    IPv4,
    IPv6,
}

impl NetworkPage {
    pub fn new() -> Self {
        Self {
            search: text_input::State::new(),
            con_name: text_input::State::focused(),
            value: String::default(),
        }
    }
    pub fn update(&mut self, msg: NetMessage) {
        match msg {
            NetMessage::OnConnectName(name) => {}
            NetMessage::OnSearchWif(text) => {
                self.value = text;
            }
        }
    }

    pub fn view(&mut self) -> Element<NetMessage> {
        let list_side = Column::new()
            .width(Length::FillPortion(3))
            .align_items(Align::Center)
            .height(Length::Fill)
            .spacing(10)
            .push(Text::new("Connection").size(25))
            .push(
                TextInput::new(
                    &mut self.search,
                    "searching..",
                    &self.value,
                    NetMessage::OnSearchWif,
                )
                .size(18)
                .padding(5),
            )
            .push(Text::new("Wi-Fi").size(18))
            .push(Text::new("wifi list here"));
        let content_side = Column::new()
            .width(Length::FillPortion(6))
            .height(Length::Fill)
            .push(Text::new("Nothing better than C"));
        let main_layout: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(list_side)
            .push(Rule::vertical(10))
            .push(content_side)
            .into();
        Container::new(main_layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(CustomContainer::Background)
            .into()
    }
}
