use iced::{
   Container, Checkbox, button, Row, Svg, Length, Text, Button, Column, Align, Space, Element
};
use super::super::styles::{CustomButton, CustomContainer};

#[derive(Debug, Clone)]
pub struct BluetoothPage {
   turn_bt: button::State, 
   advanced_btn: button::State, 
   is_on: bool, 
   show_in_menu_bar: bool
}

impl BluetoothPage {
   pub fn new() -> Self {
      Self {
         turn_bt: button::State::new(), 
         advanced_btn: button::State::new(), 
         is_on: false, show_in_menu_bar: false
      }
   }

   pub fn update(&mut self, msg: BluetoothMessage) {
      match msg {
         BluetoothMessage::ToggleBluetooth => {
            self.is_on = !self.is_on;
         }
         BluetoothMessage::ToggleShowBT(show) => {
            self.show_in_menu_bar = show
         },
      }
   }

   pub fn view(&mut self) -> Element<BluetoothMessage> {
      // ផ្ទាំងខាងឆ្វេង
      let bt_logo = Svg::from_path(format!("{}/assets/images/bluetooth.svg",env!("CARGO_MANIFEST_DIR"))).width(Length::Fill).height(Length::Fill);
      let bt = Container::new(bt_logo).width(Length::Units(150)).height(Length::Units(150)).center_x();
      let bt_state = |is_on| if is_on {"On"} else {"Off"};
      let bt_text = Text::new(format!("Bluetooth: {}", bt_state(self.is_on))).size(15);
      let turn_bt_btn = Button::new(&mut self.turn_bt, Text::new(format!("Turn Bluetooth {}", bt_state(!self.is_on)))).on_press(BluetoothMessage::ToggleBluetooth).style(CustomButton::Default);
      let mut left_pane = Column::new()
         .spacing(10)
         .align_items(Align::Center)
         .push(bt)
         .push(bt_text)
         .push(turn_bt_btn);
      if self.is_on {
         left_pane = left_pane.push(Text::new("Now Discoverable as")).push(Text::new("\"Koompi Laptop\""));
      }

      // ផ្ទាំងខាងស្ដាំ
      let mut device_pane_col = Column::new()
         .spacing(7)
         .width(Length::Fill)
         .push(Container::new(Text::new("Devices").size(12)).width(Length::Fill).padding(7).style(CustomContainer::Header));
      if self.is_on {
         device_pane_col = (1..=5).fold(device_pane_col, |col, i| {
            col.push(
               Row::new()
               .spacing(20)
               .align_items(Align::Center)
               .push(Space::with_width(Length::Units(5)))
               .push(Svg::from_path(format!("{}/assets/images/laptop.svg",env!("CARGO_MANIFEST_DIR"))).width(Length::Units(35)).height(Length::Units(35)))
               .push(Text::new(format!("Device {}", i)))
            )
         });
      }

      let device_pane = Container::new(device_pane_col).height(Length::Fill).style(CustomContainer::ForegroundWhite);
      let chk_show = Checkbox::new(self.show_in_menu_bar, "Show Bluetooth in menu bar", BluetoothMessage::ToggleShowBT).spacing(10);
      let advanced_btn = Button::new(&mut self.advanced_btn, Text::new("Advanced")).style(CustomButton::Default);
      let bottom = Row::new().push(chk_show).push(Space::with_width(Length::Fill)).push(advanced_btn);
      let right_pane = Column::new()
         .spacing(20)
         .push(device_pane)
         .push(bottom);

      // មាតិកា   
      let content = Row::new()
         .height(Length::Fill)
         .push(Container::new(left_pane).width(Length::FillPortion(4)).center_x())
         .push(Container::new(right_pane).width(Length::FillPortion(6)));

      Container::new(content)
         .padding(20)
         .width(Length::FillPortion(15))
         .height(Length::Fill)
         .style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
   ToggleBluetooth,
   ToggleShowBT(bool),
}