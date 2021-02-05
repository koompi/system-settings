use iced::{
   pick_list, button, Text, PickList, Checkbox, Length, Align, Row, Column, Container, Element, Space
};
use crate::gui::styles::{CustomSelect, CustomButton, CustomCheckbox, CustomContainer};
use crate::gui::addon_widgets::icon_btn;

#[derive(Debug, Clone, Default)]
pub struct ConfigInputSrcSec {
   pub prev_candidate_state: pick_list::State<String>,
   pub prev_candidate_val: Option<String>,
   pub next_candidate_state: pick_list::State<String>,
   pub next_candidate_val: Option<String>,
   pub enable_emoji: bool,
   pub key_modifier_state: pick_list::State<String>,
   pub key_modifier_val: Option<String>,
   pub btn_add_state: button::State,
   pub btn_cancel_state: button::State,
   pub is_changed: bool,
}

#[derive(Debug, Clone)]
pub enum ConfigInputSrcMessage {
   PrevCandidateChanged(String),
   NextCandidateChanged(String),
   EnableEmoji(bool),
   KeyModChanged(String),
   AddClicked,
   CancelClicked,
}

#[allow(non_upper_case_globals)]
impl ConfigInputSrcSec {
   pub const ls_com_keys: [&'static str; 2] = ["Shift+Tab", "Tab"];
   pub const ls_key_mods: [&'static str; 4] = ["None", "Alt", "Control", "Super"];

   pub fn new() -> Self {
      Self {
         prev_candidate_val: Some(Self::ls_com_keys[0].to_owned()),
         next_candidate_val: Some(Self::ls_com_keys[1].to_owned()),
         enable_emoji: true,
         key_modifier_val: Some(Self::ls_key_mods[1].to_owned()),
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: ConfigInputSrcMessage) {
      use ConfigInputSrcMessage::*;
      match msg {
         PrevCandidateChanged(val) => self.prev_candidate_val = Some(val),
         NextCandidateChanged(val) => self.next_candidate_val = Some(val),
         EnableEmoji(is_checked) => self.enable_emoji = is_checked,
         KeyModChanged(val) => self.key_modifier_val = Some(val),
         AddClicked | CancelClicked => println!("Exit Config"),
      }
      self.is_changed = true;
   }

   pub fn view(&mut self) -> Element<ConfigInputSrcMessage> {
      use ConfigInputSrcMessage::*;
      let ConfigInputSrcSec {
         prev_candidate_state,
         prev_candidate_val,
         next_candidate_state,
         next_candidate_val,
         enable_emoji,
         key_modifier_state,
         key_modifier_val,
         btn_add_state,
         btn_cancel_state,
         is_changed,
      } = self;

      let lb_prev_candidate = Text::new("Prev Candidate:");
      let lb_next_candidate = Text::new("Next Candidate:");
      let lb_enable_emoji = Text::new("Enable emoji:");
      let lb_key_modifier = Text::new("Key Modifier:");
      let ls_com_keys: Vec<String> = Self::ls_com_keys.iter().map(ToString::to_string).collect();
      let ls_key_mods: Vec<String> = Self::ls_key_mods.iter().map(ToString::to_string).collect();
      let select_prev_candidate = PickList::new(prev_candidate_state, ls_com_keys.clone(), prev_candidate_val.clone(), PrevCandidateChanged).style(CustomSelect::Primary);
      let select_next_candidate = PickList::new(next_candidate_state, ls_com_keys.clone(), next_candidate_val.clone(), NextCandidateChanged).style(CustomSelect::Primary);
      let chb_enable_emoji = Checkbox::new(*enable_emoji, "", EnableEmoji).spacing(10).style(CustomCheckbox::Default);
      let select_key_modifier = PickList::new(key_modifier_state, ls_key_mods.clone(), key_modifier_val.clone(), KeyModChanged).style(CustomSelect::Primary);
      let content = Row::new().spacing(10)
         .push(
            Column::new().spacing(15).align_items(Align::End)
            .push(lb_prev_candidate)
            .push(lb_next_candidate)
            .push(lb_enable_emoji)
            .push(lb_key_modifier)
         )
         .push(
            Column::new().spacing(5)
            .push(select_prev_candidate)
            .push(select_next_candidate)
            .push(chb_enable_emoji)
            .push(select_key_modifier)
         );
      let mut btn_add = icon_btn(btn_add_state, '\u{f067}', "Add", None).style(CustomButton::Primary);
      let btn_cancel = icon_btn(btn_cancel_state, '\u{f05e}', "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      if *is_changed {
         btn_add = btn_add.on_press(AddClicked);
      }

      Container::new(
         Column::new().spacing(15).width(Length::Fill).align_items(Align::Center)
         .push(Space::with_height(Length::Fill))
         .push(content)
         .push(Space::with_height(Length::Fill))
         .push(
            Row::new().spacing(10)
            .push(Space::with_width(Length::Fill))
            .push(btn_cancel)
            .push(btn_add)
         )
      ).width(Length::FillPortion(6)).height(Length::Fill).padding(20).center_x()
      .style(CustomContainer::ForegroundWhite).into()
   }
}