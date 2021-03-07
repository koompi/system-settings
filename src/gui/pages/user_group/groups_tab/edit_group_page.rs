use crate::gui::addon_widgets::icon_btn;
use crate::gui::styles::{CustomButton, CustomCheckbox, CustomContainer, CustomTextInput, FOREGROUND};
use iced::{button, scrollable, text_input, Align, Checkbox, Column, Container, Element, Length, Row, Scrollable, Space, Text, TextInput};
use iced_custom_widget::Icons;
use iced_custom_widget::Stack;
use libkoompi::system_settings::users_groups::{Group, User};
#[derive(Debug, Default)]
pub struct EditGroupPage {
   group_name_state: text_input::State,
   group_name_val: String,
   ls_members: Vec<(bool, User)>,
   ls_users: Vec<User>,
   scroll_members: scrollable::State,
   allow_admin_access: bool,
   is_changed: bool,
   btn_ok_state: button::State,
}

#[derive(Debug, Clone)]
pub enum EditGroupMsg {
   GroupNameChanged(String),
   GroupNameSubmitted(String),
   MemberToggled(usize, bool),
   OkayClicked(String, Vec<String>),
}

impl EditGroupPage {
   pub fn new(group: &Group, ls_users: &[User], allow_admin_access: bool) -> Self {
      let grp_members = group.members();
      Self {
         group_name_val: group.formatted_name(),
         ls_users: ls_users.iter().map(ToOwned::to_owned).map(|usr| usr.clone()).collect(),
         ls_members: ls_users.into_iter().map(|usr| (grp_members.contains(usr.username()), usr.to_owned())).collect(),
         allow_admin_access,
         ..Self::default()
      }
   }

   pub fn with_grp(&mut self, group: &Group, ls_users: &[User]) {
      self.group_name_val = group.formatted_name();
      self.ls_members = self.ls_users.iter().map(|usr| (group.members().contains(usr.username()), usr.to_owned())).collect();
      self.ls_users = ls_users.to_vec();
      self.is_changed = false;
   }

   pub fn update(&mut self, msg: EditGroupMsg) {
      use EditGroupMsg::*;
      match msg {
         GroupNameChanged(val) => {
            self.group_name_val = val;
            if !self.is_changed {
               self.is_changed = true;
            }
         }
         MemberToggled(idx, is_checked) => {
            if let Some(member) = self.ls_members.get_mut(idx) {
               member.0 = is_checked;
               if !self.is_changed {
                  self.is_changed = true;
               }
            }
         }
         _ => {}
      }
   }

   pub fn view(&mut self) -> Element<EditGroupMsg> {
      use EditGroupMsg::*;
      let Self {
         group_name_state,
         group_name_val,
         ls_members,
         scroll_members,
         btn_ok_state,
         ..
      } = self;

      let is_admin = self.allow_admin_access;
      let lb_grp_name = Text::new("Group name:");
      let txt_grp_name = TextInput::new(group_name_state, "Group name", &group_name_val, GroupNameChanged)
         .padding(7)
         .width(Length::Fill)
         .style(CustomTextInput::Default)
         .on_submit(GroupNameSubmitted(group_name_val.clone()));
      let con_grp_name: Element<_> = if is_admin {
         txt_grp_name.into()
      } else {
         Stack::new()
            .push(txt_grp_name, None)
            .push(Container::new(Row::new()).width(Length::Units(120)).height(Length::Units(25)).style(CustomContainer::Transparent(FOREGROUND)), None)
            .into()
      };

      let scrollable_members = ls_members
         .iter_mut()
         .enumerate()
         .fold(Scrollable::new(scroll_members).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (is_checked, user))| {
            let chb_member = Checkbox::new(*is_checked, user.fullname().as_str(), move |b| MemberToggled(idx, b)).width(Length::Fill).spacing(10).style(CustomCheckbox::Default);
            let con_member: Element<_> = if is_admin {
               chb_member.into()
            } else {
               Stack::new()
                  .push(chb_member, None)
                  .push(Container::new(Row::new()).width(Length::Units(120)).height(Length::Units(25)).style(CustomContainer::Transparent(FOREGROUND)), None)
                  .into()
            };
            scrollable.push(con_member)
         });
      let member_pane = Container::new(Column::new().push(Container::new(Text::new("Members")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_members))
         .height(Length::Fill)
         .width(Length::Fill)
         .style(CustomContainer::ForegroundWhite);

      let mut btn_okay = icon_btn(btn_ok_state, Icons::CheckCircle, "Okay", None).style(CustomButton::Primary);
      if self.is_changed {
         btn_okay = btn_okay.on_press(OkayClicked(group_name_val.clone(), ls_members.iter().filter(|(is_checked, _)| *is_checked).map(|(_, usr)| usr.username().to_owned()).collect()));
      }

      Container::new(
         Column::new()
            .width(Length::Fill)
            .spacing(10)
            .push(Row::new().spacing(10).align_items(Align::Center).push(lb_grp_name).push(con_grp_name))
            .push(member_pane)
            .push(Row::new().spacing(10).align_items(Align::Center).push(Space::with_width(Length::Fill)).push(btn_okay)),
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
   }
}
