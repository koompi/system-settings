use super::audio::{AudioTab, AudioTabMsg};
use super::configure::{ConfigureAudio, ConfigureAudioMsg};
use super::soundeffect::{SndEffect, SndEffectMsg};
use crate::gui::styles::containers::ContainerStyle;
use iced::{scrollable, Align, Column, Container, Element, Length, Row, Rule, Scrollable, Text};
use iced_custom_widget as icw;
use icw::components::Tab;
use icw::components::{Icon, Icons};
#[derive(Default)]
pub struct SoundPage {
    choice: Choice,
    scroll_content: scrollable::State,
    auddio_tab: AudioTab,
    configure: ConfigureAudio,
    sound_effects: SndEffect,
}
impl SoundPage {
    pub fn new() -> Self {
        Self {
            sound_effects: SndEffect::new(),
            auddio_tab: AudioTab::new(),
            configure: ConfigureAudio::new(),
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: SoundMessage) {
        match msg {
            SoundMessage::TabSelect(choice) => self.choice = choice,
            SoundMessage::SndEffectMsg(msg) => self.sound_effects.update(msg),
            SoundMessage::AudioTabMsg(msg) => self.auddio_tab.update(msg),
            SoundMessage::ConfigureAudioMsg(msg) => self.configure.update(msg),
        }
    }
    pub fn view(&mut self) -> Element<SoundMessage> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(Tab::new(Choice::A, Some(self.choice), SoundMessage::TabSelect, tab_content(Icons::AudioDescription, "Audio")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::B, Some(self.choice), SoundMessage::TabSelect, tab_content(Icons::AudioFile, "SoundEffect")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::C, Some(self.choice), SoundMessage::TabSelect, tab_content(Icons::BarChart, "Configure")).width(Length::Fill).height(Length::Units(50)));
        let contnet = Column::new().height(Length::Fill).align_items(Align::Center).padding(20).push(match self.choice {
            Choice::A => Container::new(self.auddio_tab.view().map(move |msg| SoundMessage::AudioTabMsg(msg))),
            Choice::B => Container::new(self.sound_effects.view().map(move |msg| SoundMessage::SndEffectMsg(msg))),
            Choice::C => Container::new(self.configure.view().map(move |msg| SoundMessage::ConfigureAudioMsg(msg))),
        });
        let netsidebar_scroll = Scrollable::new(&mut self.scroll_content).push(row).padding(10).scrollbar_width(4).scroller_width(4);
        let whole_content = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Container::new(netsidebar_scroll.height(Length::Fill)).style(ContainerStyle::White).width(Length::FillPortion(4)).height(Length::Fill))
            .push(Rule::vertical(10))
            .push(
                Container::new(contnet.height(Length::Fill)).width(Length::FillPortion(9)).height(Length::Fill).style(ContainerStyle::White), // .padding(10),
            );
        let container = Container::new(whole_content).width(Length::Fill).center_x().center_y();
        Container::new(container).style(ContainerStyle::LightGray).width(Length::Fill).height(Length::Fill).padding(10).center_x().center_y().into()
    }
}
fn tab_content<'l>(unicode: Icons, name: &str) -> Row<'l, SoundMessage> {
    Row::new().push(Icon::new(unicode).size(24)).push(Text::new(name).size(16)).align_items(Align::Center).spacing(8)
}
#[derive(Debug, Clone)]
pub enum SoundMessage {
    TabSelect(Choice),
    SndEffectMsg(SndEffectMsg),
    AudioTabMsg(AudioTabMsg),
    ConfigureAudioMsg(ConfigureAudioMsg),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
}
impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}
