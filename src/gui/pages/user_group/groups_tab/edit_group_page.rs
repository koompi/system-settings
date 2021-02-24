use crate::gui::styles::{CustomCheckbox, CustomContainer, CustomTextInput};
use iced::{scrollable, text_input, Align, Checkbox, Column, Container, Element, Length, Row, Scrollable, Text, TextInput};
use users::{all_users, get_group_by_gid, get_user_by_name, gid_t, os::unix::GroupExt, Groups, User, Users};

#[derive(Default)]
pub struct EditGroupPage {
    pub group_name_state: text_input::State,
    pub group_name_val: String,
    pub ls_members: Vec<(bool, User)>,
    pub scroll_members: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum EditGroupMsg {
    GroupNameChanged(String),
    GroupNameSubmitted,
    MemberToggled(usize, bool),
}

impl EditGroupPage {
    pub fn new(curr_gid: gid_t) -> Self {
        if let Some(group) = get_group_by_gid(curr_gid) {
            let group_name = group.name().to_str().unwrap_or("");
            let group_members: Vec<User> = group.members().to_vec().into_iter().filter_map(|name| get_user_by_name(&name)).collect();
            // let allusers = unsafe { all_users() };
            Self {
                group_name_state: text_input::State::focused(),
                group_name_val: String::from(group_name),
                ls_members: group_members.into_iter().map(|user| (true, user)).collect(), // !filter group member
                scroll_members: scrollable::State::new(),
            }
        } else {
            Self::default()
        }
    }

    pub fn with_gid(&mut self, gid: gid_t) {
        if let Some(group) = get_group_by_gid(gid) {
            let group_name = group.name().to_str().unwrap_or("");
            let group_members: Vec<User> = group.members().to_vec().iter().filter_map(|name| get_user_by_name(&name)).collect();
            // let allusers = unsafe { all_users() };
            self.group_name_val = String::from(group_name);
            self.ls_members = group_members.into_iter().map(|user| (true, user)).collect();
            // !filter group member
        }
    }

    pub fn update(&mut self, msg: EditGroupMsg) {
        use EditGroupMsg::*;
        match msg {
            GroupNameChanged(val) => self.group_name_val = val,
            GroupNameSubmitted => {}
            MemberToggled(idx, is_checked) => {
                if let Some(member) = self.ls_members.get_mut(idx) {
                    member.0 = is_checked;
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<EditGroupMsg> {
        use EditGroupMsg::*;
        let Self {
            group_name_state,
            group_name_val,
            ls_members,
            scroll_members,
            ..
        } = self;

        let lb_grp_name = Text::new("Group name:");
        let txt_grp_name = TextInput::new(group_name_state, "Group name", &group_name_val, GroupNameChanged)
            .padding(7)
            .width(Length::Fill)
            .style(CustomTextInput::Default)
            .on_submit(GroupNameSubmitted);

        let scrollable_members = ls_members
            .iter_mut()
            .enumerate()
            .fold(Scrollable::new(scroll_members).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (is_checked, user))| {
                let chb_member = Checkbox::new(*is_checked, user.name().to_str().unwrap_or(""), move |b| MemberToggled(idx, b))
                    .width(Length::Fill)
                    .spacing(10)
                    .style(CustomCheckbox::Default);
                scrollable.push(chb_member)
            });
        let member_pane = Container::new(Column::new().push(Container::new(Text::new("Members")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_members))
            .height(Length::Fill)
            .width(Length::Fill)
            .style(CustomContainer::ForegroundWhite);

        Container::new(Column::new().width(Length::Fill).spacing(10).push(Row::new().spacing(10).align_items(Align::Center).push(lb_grp_name).push(txt_grp_name)).push(member_pane))
            .width(Length::FillPortion(7))
            .height(Length::Fill)
            .into()
    }
}
