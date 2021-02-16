#![allow(dead_code)]
use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle};
use iced::{button, Align, Button, Column, Container, Element, Length, Space, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Toggler;
#[derive(Debug, Default, Copy, Clone)]
pub struct Wire {
    is_enable: bool,
    add_net_con: button::State,
}
#[derive(Debug, Copy, Clone)]
pub enum WireMsg {
    EnableWired(bool),
    NetworkAdded,
}
impl Wire {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }
    pub fn update(&mut self, msg: WireMsg) {
        match msg {
            WireMsg::EnableWired(is_enable) => {
                self.is_enable = is_enable;
            }
            WireMsg::NetworkAdded => {}
        }
    }
    pub fn view(&mut self) -> Element<WireMsg> {
        Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(Toggler::new(self.is_enable, String::from("Wire Network Adapter"), WireMsg::EnableWired))
            .push(
                Container::new(Text::new("Plug in the network cable first"))
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Units(100))
                    .style(ContainerStyle::LightGrayCircle),
            )
            .push(Space::with_height(Length::Fill))
            .push(
                Button::new(&mut self.add_net_con, Icon::new('\u{f067}').size(24))
                    .style(ButtonStyle::BigCircular(86, 101, 115, 1.0))
                    .padding(10)
                    .width(Length::Units(50))
                    .height(Length::Units(50))
                    .on_press(WireMsg::NetworkAdded),
            )
            .into()
    }
}
