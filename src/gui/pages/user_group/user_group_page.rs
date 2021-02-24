use super::{
    groups_tab::{GroupsMsg, GroupsTab},
    users_tab::{UsersMsg, UsersTab},
};
use crate::gui::addon_widgets::tabbar;
use crate::gui::styles::{CustomButton, CustomContainer};
use iced::{button, Align, Button, Column, Container, Element, Length, Row, Space, Text};
use libkoompi::system_settings::users_groups::UsersGroupsManager;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Default)]
pub struct UserGroupPage {
    usrgrp_mn: Rc<RefCell<UsersGroupsManager>>,
    tabbar_state: Vec<(&'static str, button::State)>,
    curr_tab_idx: usize,
    users_tab: UsersTab,
    groups_tab: GroupsTab,
    btn_option_state: button::State,
}

#[derive(Debug, Clone)]
pub enum UserGroupMsg {
    TabChanged(usize),
    UsersMSG(UsersMsg),
    GroupsMSG(GroupsMsg),
    OptionClicked,
}

impl UserGroupPage {
    pub fn new() -> Self {
        let tabs = vec![("Users", button::State::new()), ("Groups", button::State::new())];
        match UsersGroupsManager::new() {
            Ok(usrgrp_mn) => {
                let usrgrp_mn = Rc::new(RefCell::new(usrgrp_mn));

                Self {
                    users_tab: UsersTab::new(Rc::clone(&usrgrp_mn)),
                    groups_tab: GroupsTab::new(Rc::clone(&usrgrp_mn)),
                    usrgrp_mn,
                    tabbar_state: tabs,
                    curr_tab_idx: 0,
                    btn_option_state: button::State::new(),
                }
            }
            Err(err) => {
                eprintln!("{:?}", err);
                Self::default()
            }
        }
    }
    pub fn update(&mut self, msg: UserGroupMsg) {
        use UserGroupMsg::*;
        let Self { users_tab, groups_tab, .. } = self;

        match msg {
            TabChanged(idx) => self.curr_tab_idx = idx,
            UsersMSG(users_msg) => users_tab.update(users_msg),
            GroupsMSG(groups_msg) => groups_tab.update(groups_msg),
            OptionClicked => {}
        }
    }
    pub fn view(&mut self) -> Element<UserGroupMsg> {
        use UserGroupMsg::*;
        let Self {
            tabbar_state,
            users_tab,
            groups_tab,
            btn_option_state,
            ..
        } = self;

        // របារផ្ទាំង
        let tabbar_sec = tabbar(tabbar_state, self.curr_tab_idx, TabChanged);

        // ទិដ្ឋភាពទូទៅ
        let tabview = match self.curr_tab_idx {
            0 => users_tab.view().map(|msg| UsersMSG(msg)),
            1 => groups_tab.view().map(|msg| GroupsMSG(msg)),
            _ => Row::new().into(),
        };

        // ផ្នែកខាងក្រោម
        let btn_opt = Button::new(btn_option_state, Text::new("Options")).on_press(OptionClicked).style(CustomButton::Default);

        let bottom_sec = Container::new(Row::new().padding(15).spacing(10).align_items(Align::Center).push(Space::with_width(Length::Fill)).push(btn_opt))
            .width(Length::Fill)
            .align_x(Align::End);

        // មាតិកា
        let content = Column::new()
            .width(Length::Fill)
            .push(tabbar_sec)
            .push(Container::new(tabview).width(Length::Fill).height(Length::Fill).padding(10).style(CustomContainer::ForegroundGray))
            .push(bottom_sec);

        Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
    }
}
