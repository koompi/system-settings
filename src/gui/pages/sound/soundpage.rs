use super::audio::{AudioTab, AudioTabMsg};
use super::input::{SoundInput, SoundInputMsg};
use super::output::{SoundOutput, SoundOutputMsg};
use super::soundeffect::{SndEffect, SndEffectMsg};
use crate::gui::styles::containers::ContainerStyle;
use iced::{scrollable, Align, Column, Container, Element, Length, Row, Rule, Scrollable, Text};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Tab;
#[derive(Default)]
pub struct SoundPage {
    choice: Choice,
    scroll_content: scrollable::State,
    output_content: SoundOutput,
    input_content: SoundInput,
    auddio_tab: AudioTab,
    sound_effects: SndEffect,
}
impl SoundPage {
    pub fn new() -> Self {
        Self {
            output_content: SoundOutput::new(),
            input_content: SoundInput::new(),
            sound_effects: SndEffect::new(),
            ..Default::default()
        }
    }
    pub fn update(&mut self, msg: SoundMessage) {
        match msg {
            SoundMessage::TabSelect(choice) => self.choice = choice,
            SoundMessage::SoundOutputMsg(msg) => self.output_content.update(msg),
            SoundMessage::SoundInputMsg(msg) => self.input_content.update(msg),
            SoundMessage::SndEffectMsg(msg) => self.sound_effects.update(msg),
            SoundMessage::AudioTabMsg(msg) => self.auddio_tab.update(msg),
        }
    }
    pub fn view(&mut self) -> Element<SoundMessage> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(Tab::new(Choice::A, Some(self.choice), SoundMessage::TabSelect, tab_content('\u{f028}', "Output")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::B, Some(self.choice), SoundMessage::TabSelect, tab_content('\u{f130}', "Input")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::C, Some(self.choice), SoundMessage::TabSelect, tab_content('\u{f5fd}', "SoundPage Effects")).width(Length::Fill).height(Length::Units(50)))
            .push(Tab::new(Choice::D, Some(self.choice), SoundMessage::TabSelect, tab_content('\u{f5fd}', "Audio")).width(Length::Fill).height(Length::Units(50)));
        let contnet = Column::new().height(Length::Fill).align_items(Align::Center).padding(20).push(match self.choice {
            Choice::A => Container::new(self.output_content.view().map(move |msg| SoundMessage::SoundOutputMsg(msg))),
            Choice::B => Container::new(self.input_content.view().map(move |msg| SoundMessage::SoundInputMsg(msg))),
            Choice::C => Container::new(self.sound_effects.view().map(move |msg| SoundMessage::SndEffectMsg(msg))),
            Choice::D => Container::new(self.auddio_tab.view().map(move |msg| SoundMessage::AudioTabMsg(msg))),
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
fn tab_content<'l>(unicode: char, name: &str) -> Row<'l, SoundMessage> {
    Row::new().push(Icon::new(unicode).size(24)).push(Text::new(name).size(16)).align_items(Align::Center).spacing(8)
}
#[derive(Debug, Clone)]
pub enum SoundMessage {
    TabSelect(Choice),
    SoundOutputMsg(SoundOutputMsg),
    SoundInputMsg(SoundInputMsg),
    SndEffectMsg(SndEffectMsg),
    AudioTabMsg(AudioTabMsg),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
}
impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}
