mod add_group_page;
mod edit_group_page;

use crate::gui::styles::{CustomButton, CustomContainer};
use iced::{button, scrollable, Align, Button, Column, Container, Element, Length, Row, Scrollable, Text};
use iced_custom_widget::Icon;
use users::{get_current_gid, group_access_list, Group, Groups, Users};
use {
    add_group_page::{AddGroupMsg, AddGroupPage},
    edit_group_page::{EditGroupMsg, EditGroupPage},
};

#[derive(Default)]
pub struct GroupsTab {
    pub ls_grps: Vec<(Group, button::State)>,
    pub selected_grp: Option<usize>,
    pub scroll_grps: scrollable::State,
    pub add_state: button::State,
    pub remove_state: button::State,

    // dynamic section
    pub edit_group_page: EditGroupPage,
    pub add_group_page: Option<AddGroupPage>,
}

#[derive(Debug, Clone)]
pub enum GroupsMsg {
    SelecteGroup(usize),
    AddClicked,
    RemoveClicked,
    EditGroupMSG(EditGroupMsg),
    AddGroupMSG(AddGroupMsg),
}

impl GroupsTab {
    pub fn new() -> Self {
        match group_access_list() {
            Ok(ls_groups) => {
                let curr_gid = get_current_gid();
                let selected_idx = ls_groups.iter().position(|grp| grp.gid() == curr_gid);
                Self {
                    ls_grps: ls_groups.into_iter().map(|grp| (grp, button::State::new())).collect(),
                    selected_grp: selected_idx,
                    edit_group_page: EditGroupPage::new(curr_gid),
                    ..Self::default()
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
                Self::default()
            }
        }
    }

    pub fn update(&mut self, msg: GroupsMsg) {
        use GroupsMsg::*;
        let Self { ls_grps, edit_group_page, add_group_page, .. } = self;

        match msg {
            SelecteGroup(idx) => {
                self.selected_grp = Some(idx);
                if let Some((group, _)) = ls_grps.get(idx) {
                    edit_group_page.with_gid(group.gid());
                }
            }
            AddClicked => {
                *add_group_page = Some(AddGroupPage::new());
            }
            RemoveClicked => {
                if let Some(idx) = self.selected_grp {
                    ls_grps.remove(idx);
                }
                self.selected_grp = None;
            }
            EditGroupMSG(edit_group_msg) => edit_group_page.update(edit_group_msg),
            AddGroupMSG(AddGroupMsg::CreateClicked(group_name)) => {
                let new_group = Group::new(2750, &group_name);
                ls_grps.push((new_group, button::State::new()));
                *add_group_page = None;
            }
            AddGroupMSG(AddGroupMsg::CancelClicked) => *add_group_page = None,
            AddGroupMSG(add_group_msg) => {
                if let Some(add_group_page) = add_group_page {
                    add_group_page.update(add_group_msg);
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<GroupsMsg> {
        use GroupsMsg::*;
        let Self {
            ls_grps,
            selected_grp,
            scroll_grps,
            add_state,
            remove_state,
            edit_group_page,
            add_group_page,
            ..
        } = self;

        let scrollable_group = ls_grps
            .iter_mut()
            .enumerate()
            .fold(Scrollable::new(scroll_grps).height(Length::Fill).padding(7).spacing(4).scroller_width(4).scrollbar_width(4), |scrollable, (idx, (group, state))| {
                let btn = Button::new(state, Text::new(group.name().to_str().unwrap_or("")))
                    .width(Length::Fill)
                    .on_press(SelecteGroup(idx))
                    .style(if let Some(selected) = *selected_grp {
                        if selected == idx {
                            CustomButton::Selected
                        } else {
                            CustomButton::Text
                        }
                    } else {
                        CustomButton::Text
                    });
                scrollable.push(btn)
            });
        let btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(AddClicked).style(CustomButton::Text);
        let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
        if selected_grp.is_some() {
            btn_remove = btn_remove.on_press(RemoveClicked);
        }
        let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
        let group_pane = Container::new(Column::new().push(Container::new(Text::new("Groups")).width(Length::Fill).padding(7).style(CustomContainer::Header)).push(scrollable_group).push(btn_group))
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .style(CustomContainer::ForegroundWhite);

        let right_sec = if let Some(add_group_page) = add_group_page {
            add_group_page.view().map(|msg| AddGroupMSG(msg))
        } else {
            edit_group_page.view().map(|msg| EditGroupMSG(msg))
        };

        Container::new(Row::new().width(Length::Fill).spacing(10).push(group_pane).push(right_sec)).width(Length::Fill).height(Length::Fill).into()
    }
}
