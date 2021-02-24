use iced::{
   text_input, button, scrollable, Scrollable, TextInput, Text, Container, Length, Column, Row, Align, Checkbox, Element, Space,
};
use libkoompi::system_settings::users_groups::{User, Group};
use crate::gui::styles::{CustomTextInput, CustomCheckbox, CustomContainer, CustomButton};
use crate::gui::addon_widgets::icon_btn;

#[derive(Debug, Default)]
pub struct EditGroupPage {
   group_name_state: text_input::State,
   group_name_val: String,
   ls_members: Vec<(bool, User)>,
   ls_users: Vec<User>,
   scroll_members: scrollable::State,
   is_changed: bool,
   btn_ok_state: button::State,
}

#[derive(Debug, Clone)]
pub enum EditGroupMsg {
   GroupNameChanged(String),
   // GroupNameSubmitted(String),
   MemberToggled(usize, bool),
   OkayClicked(String, Vec<String>),
}

impl EditGroupPage {
   pub fn new(group: &Group, ls_users: Vec<&User>) -> Self {
      let grp_members = group.members();
      Self {
         group_name_val: group.formatted_name(),
         ls_users: ls_users.iter().map(ToOwned::to_owned).map(|usr| usr.clone()).collect(),
         ls_members: ls_users.into_iter().map(|usr| (grp_members.contains(usr.username()), usr.to_owned())).collect(), 
         ..Self::default()
      }
   }

   pub fn with_grp(&mut self, group: &Group) {
      self.group_name_val = group.formatted_name();
      self.ls_members = self.ls_users.iter().map(|usr| (group.members().contains(usr.username()), usr.to_owned())).collect();
      self.is_changed = false;
   }

   pub fn update(&mut self, msg: EditGroupMsg) {
      use EditGroupMsg::*;
      match msg {
         GroupNameChanged(val) => self.group_name_val = val,
         MemberToggled(idx, is_checked) => {
            if let Some(member) = self.ls_members.get_mut(idx) {
               member.0 = is_checked;
               if !self.is_changed { self.is_changed = true; }
            }
         },
         _ => {}
      }
   }

   pub fn view(&mut self) -> Element<EditGroupMsg> {
      use EditGroupMsg::*;
      let Self {
         group_name_state, group_name_val, ls_members, scroll_members, btn_ok_state, ..
      } = self;

      let lb_grp_name = Text::new("Group name:");
      let txt_grp_name = TextInput::new(group_name_state, "Group name", &group_name_val, GroupNameChanged).padding(7).width(Length::Fill).style(CustomTextInput::Default);
         // .on_submit(GroupNameSubmitted(group_name_val.clone()));

      let scrollable_members = ls_members.iter_mut().enumerate().fold(Scrollable::new(scroll_members).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (is_checked, user))| {
         let chb_member = Checkbox::new(*is_checked, user.fullname().as_str(), move |b| MemberToggled(idx, b)).width(Length::Fill).spacing(10).style(CustomCheckbox::Default);
         scrollable.push(chb_member)
      });
      let member_pane = Container::new(
         Column::new()
         .push(
            Container::new(Text::new("Members")).width(Length::Fill).padding(7).style(CustomContainer::Header),
         )
         .push(scrollable_members)
      ).height(Length::Fill).width(Length::Fill).style(CustomContainer::ForegroundWhite);

      let mut btn_okay = icon_btn(btn_ok_state, '\u{f00c}', "Okay", None).style(CustomButton::Primary);
      if self.is_changed {
         btn_okay = btn_okay.on_press(OkayClicked(group_name_val.clone(), ls_members.iter().filter(|(is_checked, _)| *is_checked).map(|(_, usr)| usr.username().to_owned()).collect()));
      }

      Container::new(
         Column::new().width(Length::Fill).padding(20).spacing(10)
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(lb_grp_name)
            .push(txt_grp_name)
         )
         .push(member_pane)
         .push(
            Row::new().spacing(10).align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(btn_okay)
         )
      ).width(Length::FillPortion(7)).height(Length::Fill).into()
   }
}