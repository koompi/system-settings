use iced::{
   button, text_input, scrollable, TextInput, Text, Button, Scrollable, Length, Align, Row, Column, Container, Element, Space
};
use crate::gui::styles::{CustomContainer, CustomButton, CustomTextInput};
use crate::gui::addon_widgets::icon_btn;

#[derive(Debug, Clone, Default)]
pub struct AddInputSrcSec {
   pub search_state: text_input::State,
   pub search_val: String,
   pub ls_langs: Vec<(String, button::State)>,
   pub filtered_ls_langs: Vec<(String, button::State)>,
   pub ls_layouts: Vec<(String, button::State)>,
   pub selected_lang: Option<String>,
   pub selected_layout: Option<String>,
   pub btn_add_state: button::State,
   pub btn_cancel_state: button::State,
   pub scroll_lang: scrollable::State,
   pub scroll_layout: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum AddInputSrcMessage {
   SearchChanged(String),
   LangChanged(String),
   LayoutChanged(String),
   AddClicked(String),
   CancelClicked,
}

#[allow(non_upper_case_globals)]
impl AddInputSrcSec {
   const ls_langs: [&'static str; 5] = ["English", "Khmer", "Chinese", "Japenese", "French"];
   const ls_layouts: [&'static str; 6] = ["QWERTY", "QWERTZ", "AZERTY", "QZERTY", "QUERTY", "AWERTY"];

   pub fn new() -> Self {
      Self {
         ls_langs: Self::ls_langs.iter().map(|lang| (lang.to_string(), button::State::new())).collect(),
         filtered_ls_langs: Self::ls_langs.iter().map(|lang| (lang.to_string(), button::State::new())).collect(),
         ls_layouts: Self::ls_layouts.iter().map(|layout| (layout.to_string(), button::State::new())).collect(),
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: AddInputSrcMessage) {
      use AddInputSrcMessage::*;
      match msg {
         SearchChanged(val) => {
            self.search_val = val;
            self.filtered_ls_langs = self.ls_langs.iter().filter(|lang| lang.0.to_lowercase().contains(&self.search_val.to_lowercase())).cloned().collect();
         },
         LangChanged(val) => {
            self.selected_lang = Some(val); 
            self.selected_layout = Some(Self::ls_layouts[0].to_owned()); 
         },
         LayoutChanged(val) => self.selected_layout = Some(val),
         AddClicked(..) | CancelClicked => self.reset_setting(),
      }
   }

   fn reset_setting(&mut self) {
      self.selected_lang = None;
      self.selected_layout = None;
   }

   pub fn view(&mut self) -> Element<AddInputSrcMessage> {
      let AddInputSrcSec {
         search_state,
         search_val,
         filtered_ls_langs,
         ls_layouts,
         selected_lang,
         selected_layout,
         btn_add_state,
         btn_cancel_state,
         scroll_lang,
         scroll_layout,
         ..
      } = self;

      let input_search = TextInput::new(search_state, "Search language", &search_val, AddInputSrcMessage::SearchChanged).padding(10).style(CustomTextInput::Default);
      let scrollable_langs = filtered_ls_langs.iter_mut().fold(Scrollable::new(scroll_lang).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (lang, state)| {
         let btn = Button::new(state, Text::new(lang.as_str())).width(Length::Fill).on_press(AddInputSrcMessage::LangChanged(lang.clone())).style(
            if let Some(selected) = selected_lang {
               if selected == lang {CustomButton::Selected}
               else {CustomButton::Text}
            }
            else {CustomButton::Text}
         );
         scrollable.push(btn)
      });
      let langs_pane = Container::new(
         Column::new()
            .push(
               Container::new(Text::new("Languages")).width(Length::Fill).padding(7).style(CustomContainer::Header),
            )
            .push(scrollable_langs),
      ).height(Length::Fill).style(CustomContainer::ForegroundWhite);

      let scrollable_layouts = ls_layouts.iter_mut().fold(Scrollable::new(scroll_layout).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (layout, state)| {
         let mut btn = Button::new(state, Text::new(layout.as_str())).width(Length::Fill).style(
            if let Some(selected) = selected_layout {
               if selected == layout {CustomButton::Selected}
               else {CustomButton::Text}
            }
            else {CustomButton::Text}
         );
         if selected_lang.is_some() {
            btn = btn.on_press(AddInputSrcMessage::LayoutChanged(layout.clone()));
         }
         scrollable.push(btn)
      });
      let layouts_pane = Container::new(
         Column::new()
         .push(
            Container::new(Text::new("Layouts")).width(Length::Fill).padding(7).style(CustomContainer::Header),
         )
         .push(scrollable_layouts),
      ).height(Length::Fill).style(CustomContainer::ForegroundWhite);
      let mut btn_add = icon_btn(btn_add_state, '\u{f067}', "Add", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(AddInputSrcMessage::CancelClicked).style(CustomButton::Hovered);
      if let Some(layout) = selected_layout {
         btn_add = btn_add.on_press(AddInputSrcMessage::AddClicked(layout.clone()));
      }

      Container::new(
         Row::new().spacing(10).align_items(Align::Center).height(Length::Fill)
         .push(
            Column::new().spacing(10).width(Length::FillPortion(4))
            .push(input_search)
            .push(langs_pane)
         )
         .push(
            Column::new().spacing(10).width(Length::FillPortion(6))
            .push(layouts_pane)
            .push(
               Row::new().spacing(10)
               .push(Space::with_width(Length::Fill))
               .push(btn_cancel)
               .push(btn_add)
            )
         )
      ).width(Length::FillPortion(6)).into()
   }
}