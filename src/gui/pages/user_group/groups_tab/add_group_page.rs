use iced::{
   text_input, TextInput, Text, button, Container, Length, Column, Row, Align, Space, Element,
};
use crate::gui::styles::{CustomTextInput, CustomButton};
use crate::gui::addon_widgets::icon_btn;

#[derive(Debug, Default)]
pub struct AddGroupPage {
   fullname_state: text_input::State,
   fullname_val: String,
   btn_create_state: button::State,
   btn_cancel_state: button::State,
}

#[derive(Debug, Clone)]
pub enum AddGroupMsg {
   FullNameChanged(String),
   CreateClicked(String),
   CancelClicked,
}

impl AddGroupPage {
   pub fn new() -> Self {
      Self::default()
   }

   pub fn update(&mut self, msg: AddGroupMsg) {
      use AddGroupMsg::*;
      match msg {
         FullNameChanged(val) => self.fullname_val = val,
         _ => {}
      }
   }

   pub fn view(&mut self) -> Element<AddGroupMsg> {
      use AddGroupMsg::*;
      let Self {
         fullname_state, fullname_val, btn_create_state, btn_cancel_state,
      } = self;

      let lb_grp_name = Text::new("Group name:");
      let txt_grp_name = TextInput::new(fullname_state, "Group name", &fullname_val, FullNameChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);

      let mut btn_create = icon_btn(btn_create_state, '\u{f00c}', "Create", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      if !fullname_val.is_empty() {
         btn_create = btn_create.on_press(CreateClicked(fullname_val.to_owned()));
      }

      Container::new(
         Column::new().width(Length::Fill).padding(20).spacing(10)
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(lb_grp_name)
            .push(txt_grp_name)
         )
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(btn_cancel)
            .push(btn_create)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}