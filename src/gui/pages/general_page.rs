use iced::{
   Container, Checkbox
};
use super::super::pages::PagesMessage;

pub fn general_page<'a>(checkbox: bool) -> Container<'a, PagesMessage> {
   Container::new(
      Checkbox::new(checkbox, "Checkbox", PagesMessage::CheckboxToggle)
   )
}