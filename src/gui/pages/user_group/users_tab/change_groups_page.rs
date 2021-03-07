use crate::gui::addon_widgets::icon_btn;
use crate::gui::styles::{CustomButton, CustomCheckbox, CustomContainer, CustomTextInput};
use iced::{button, scrollable, text_input, Align, Checkbox, Column, Container, Element, Length, Row, Scrollable, Space, Text, TextInput};
use iced_custom_widget::Icons;
use libkoompi::system_settings::users_groups::{Group, User};
#[derive(Debug, Default)]
pub struct ChangeGroupsPage {
   grpname_state: text_input::State,
   grpname: String,
   ls_grps: Vec<(bool, Group)>,
   filtered_ls_grps: Vec<(bool, Group)>,
   ls_all_grps: Vec<Group>,
   scroll_grps: scrollable::State,
   is_changed: bool,
   btn_ok_state: button::State,
   btn_cancel_state: button::State,
}

#[derive(Debug, Clone)]
pub enum ChangeGroupsMsg {
   GroupNameChanged(String),
   GroupToggled(usize, bool),
   OkayClicked(Vec<String>),
   CancelClicked,
}

impl ChangeGroupsPage {
   pub fn new(usr: &User, ls_al_grps: &[Group]) -> Self {
      let ls_grps: Vec<(bool, Group)> = ls_al_grps.iter().map(ToOwned::to_owned).map(|grp| (usr.groups().contains(grp.name()), grp.to_owned())).collect();
      Self {
         filtered_ls_grps: ls_grps.clone(),
         ls_grps,
         ls_all_grps: ls_al_grps.into_iter().map(ToOwned::to_owned).collect(),
         ..Self::default()
      }
   }

   pub fn with_user(&mut self, usr: &User) {
      let ls_grps: Vec<(bool, Group)> = self.ls_all_grps.iter().map(|grp| (usr.groups().contains(grp.name()), grp.to_owned())).collect();
      self.filtered_ls_grps = ls_grps.clone();
      self.ls_grps = ls_grps;
      self.is_changed = false;
   }

   pub fn update(&mut self, msg: ChangeGroupsMsg) {
      use ChangeGroupsMsg::*;
      match msg {
         GroupNameChanged(val) => {
            self.grpname = val.clone();
            self.filtered_ls_grps = self.ls_grps.iter().filter(|(_, grp)| grp.name().contains(&val) || grp.formatted_name().contains(&val)).cloned().collect();
         }
         GroupToggled(idx, is_checked) => {
            if let Some(grp) = self.filtered_ls_grps.get_mut(idx) {
               grp.0 = is_checked;
               if !self.is_changed {
                  self.is_changed = true;
               }
            }
         }
         _ => {}
      }
   }

   pub fn view(&mut self) -> Element<ChangeGroupsMsg> {
      use ChangeGroupsMsg::*;
      let Self {
         grpname_state,
         grpname,
         filtered_ls_grps,
         scroll_grps,
         btn_cancel_state,
         btn_ok_state,
         ..
      } = self;

      let txt_search = TextInput::new(grpname_state, "Search name...", &grpname, GroupNameChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
      let scrollable_grps = filtered_ls_grps
         .iter_mut()
         .enumerate()
         .fold(Scrollable::new(scroll_grps).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (is_checked, grp))| {
            let chb_grp = Checkbox::new(*is_checked, grp.name().as_str(), move |b| GroupToggled(idx, b)).width(Length::Fill).spacing(10).style(CustomCheckbox::Default);
            scrollable.push(chb_grp)
         });
      let grps_pane = Container::new(Column::new().push(Container::new(Text::new("Groups")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_grps))
         .height(Length::Fill)
         .width(Length::Fill)
         .style(CustomContainer::ForegroundWhite);

      let btn_cancel = icon_btn(btn_cancel_state, Icons::Minus, "Cancel", None).on_press(CancelClicked).style(CustomButton::Hovered);
      let mut btn_okay = icon_btn(btn_ok_state, Icons::Ad, "Okay", None).style(CustomButton::Primary);
      if self.is_changed {
         btn_okay = btn_okay.on_press(OkayClicked(filtered_ls_grps.iter().filter(|(is_checked, _)| *is_checked).map(|(_, grp)| grp.name().to_owned()).collect()));
      }

      Container::new(
         Column::new()
            .width(Length::Fill)
            .spacing(10)
            .push(txt_search)
            .push(grps_pane)
            .push(Row::new().spacing(10).align_items(Align::Center).push(Space::with_width(Length::Fill)).push(btn_cancel).push(btn_okay)),
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
   }
}
