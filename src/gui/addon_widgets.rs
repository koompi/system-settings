use crate::gui::styles::{CustomButton, CustomContainer};
use iced::{button, Align, Button, Container, Length, Row, Text};
use iced_custom_widget::Icon;

pub fn icon_btn<'a, M: 'a + Clone>(state: &'a mut button::State, icon: char, text: &str, size: Option<u16>) -> Button<'a, M> {
   let mut ico = Icon::new(icon);
   if let Some(size) = size {
      ico = ico.size(size);
   }
   Button::new(state, Row::new().spacing(3).align_items(Align::Center).push(ico).push(Text::new(&format!("  {}  ", text))))
}

pub fn tabbar<'a, M: 'a + Clone, F: Fn(usize) -> M>(tabs: &'a mut Vec<(&'static str, button::State)>, curr_idx: usize, on_press: F) -> Container<'a, M> {
   let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
   for (idx, (name, btn_state)) in tabs.iter_mut().enumerate() {
      let mut btn = Button::new(btn_state, Text::new(&format!("  {}  ", *name))).padding(5).on_press(on_press(idx));
      if curr_idx == idx {
         btn = btn.style(CustomButton::SelectedTab);
      } else {
         btn = btn.style(CustomButton::Tab);
      }
      tabbar = tabbar.push(btn);
   }
   let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
   Container::new(tabbar_con).padding(7).width(Length::Fill).center_x()
}
