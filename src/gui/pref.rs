use iced::{button, Align, Button, Column, Element, Text, Svg, Length, HorizontalAlignment, Container};
use super::styles::CustomButton;

#[derive(Debug, Clone)]
pub struct Pref {
   pub path: String,
   pub name: String,
   pub category: Category,
   pub button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Category {
   System,
   Hardware
}

#[derive(Debug, Clone, Copy)]
pub enum PrefMessage {
   PrefClicked,
}

impl Pref {
   pub fn new(path: String, name: String, category: Category) -> Self {
      Self {
         path,
         name,
         category,
         button_state: button::State::new(),
      }
   }

   pub fn update(&mut self, message: PrefMessage) {
      match message {
         PrefMessage::PrefClicked => {}
      }
   }

   pub fn view_main(&mut self) -> Element<PrefMessage> {
      let icon = Svg::from_path(&self.path).width(Length::Fill).height(Length::Fill);
      let icon_button = Button::new(&mut self.button_state, icon)
         .width(Length::Units(80))
         .height(Length::Units(80))
         .padding(10)
         .on_press(PrefMessage::PrefClicked)
         .style(CustomButton::Text);
      let name = Text::new(&self.name).horizontal_alignment(HorizontalAlignment::Center);
      let pref = Column::new().spacing(10).align_items(Align::Center).push(icon_button).push(name);
      Container::new(pref).width(Length::Units(100)).into()
   }

   pub fn view_sidebar(&mut self, is_selected: bool) -> Element<PrefMessage> {
      let icon = Svg::from_path(&self.path).width(Length::Fill).height(Length::Fill);
      let icon_container = Container::new(icon)
         .width(Length::Units(65))
         .height(Length::Units(65));
      let name = Text::new(&self.name).horizontal_alignment(HorizontalAlignment::Center);
      let pref = Column::new().spacing(10).align_items(Align::Center).push(icon_container).push(name);
      Button::new(&mut self.button_state, pref)
         .width(Length::Units(85))
         .padding(6)
         .on_press(PrefMessage::PrefClicked)
         .style(
            if is_selected {CustomButton::Selected} 
            else {CustomButton::Sidebar}
         ).into()
   }
}
