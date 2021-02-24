use crate::gui::styles::{buttons::ButtonStyle, containers::ContainerStyle, picklist::PickListStyle};
use iced::{pick_list, Align, Column, Container, Element, Font, HorizontalAlignment, Length, PickList, Row, Text};
use libkoompi::system_settings::sounds::controllers::{DeviceControl, SinkController};
use libkoompi::system_settings::SoundCard;
use std::fmt;
#[derive(Default)]
pub struct ConfigureAudio {
    pick_profiles: pick_list::State<ProfileList>,
    select_pick: ProfileList,
    sink_control: SinkController,
    profile_data: Vec<ProfileList>,
}
#[derive(Debug, Clone)]
pub enum ConfigureAudioMsg {
    ProfileChanged(ProfileList),
}
impl ConfigureAudio {
    pub fn new() -> Self {
        let mut list_profiles = Vec::new();
        let mut first_select = ProfileList::default();
        match SinkController::create().get_card_info_list() {
            Ok(vec_of_dev) => {
                for dev in vec_of_dev {
                    for profile_list in dev.profiles {
                        list_profiles.push(match profile_list.description {
                            Some(descr) => ProfileList { profile: descr },
                            None => ProfileList { profile: String::from("") },
                        });
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
        match list_profiles.get(2) {
            Some(name) => first_select = name.clone(),
            None => {}
        }

        Self {
            sink_control: SinkController::create(),
            profile_data: list_profiles,
            select_pick: first_select,
            ..Self::default()
        }
    }
    pub fn update(&mut self, msg: ConfigureAudioMsg) {
        match msg {
            ConfigureAudioMsg::ProfileChanged(profie) => {
                self.select_pick = profie;
            }
        }
    }
    pub fn view(&mut self) -> Element<ConfigureAudioMsg> {
        let content_view = Container::new(Text::new("Device Profile")).width(Length::Fill);
        Container::new(
            Column::new()
                .spacing(10)
                .push(content_view.center_x())
                .push(Row::new().align_items(Align::Center).push(card_icon()).push(Text::new("Built-in Audio Card")))
                .push(
                    Row::new()
                        .spacing(10)
                        .align_items(Align::Center)
                        .push(Text::new("Profile"))
                        // .style(ButtonStyle::Transparent)
                        .push(
                            PickList::new(&mut self.pick_profiles, &self.profile_data, Some(self.select_pick.clone()), ConfigureAudioMsg::ProfileChanged)
                                .width(Length::Fill)
                                .padding(10)
                                .style(PickListStyle {}),
                        ),
                ),
        )
        .style(ContainerStyle::LightGrayCircle)
        .padding(10)
        .width(Length::Fill)
        .into()
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ProfileList {
    pub profile: String,
}
impl fmt::Display for ProfileList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.profile)
    }
}
const ICONS: Font = Font::External {
    name: "Line Awesome",
    bytes: include_bytes!("../../../../assets/fonts/la-solid-900.woff"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string()).font(ICONS).width(Length::Units(20)).horizontal_alignment(HorizontalAlignment::Center).size(20)
}

fn card_icon() -> Text {
    icon('\u{f7c2}')
}
