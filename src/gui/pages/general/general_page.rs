// use super::super::super::styles::{CustomButton, CustomContainer, CustomRadio, CustomSelect};
use iced::{Align, Button, Checkbox, Column, Command, Container, Element, Length, PickList, Radio, Row, Rule, Scrollable, Space, Svg, Text, button, pick_list, scrollable, slider,Slider, TextInput,text_input};
use iced_custom_widget as icw;
use icw::components::Tab;
use icw::components::Icon;
use crate::gui::styles::containers::ContainerStyle;
// use serde::de::value;
use super::theme_page::{Theme, ThemeMsg};
use super::font_page::{FontStyle, FontMsg};
use super::icon_page::{IconStyle,IconMsg};

// use crate::helpers::ROOT_PATH;

#[derive(Default, Debug, Clone)]
pub struct General {
    choice: Choice,
    theme: Theme,
    icon_style: IconStyle,
    // Cursor: Cursor,
    font_style: FontStyle,
    is_active: bool,
    scroll_content: scrollable::State,
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
#[derive(Debug, Clone)]
pub enum GeneralMessage {
    TabSelect(Choice),
    ThemeMsg(ThemeMsg),
    IconMsg(IconMsg),
    FontMsg(FontMsg),
}

impl General {
    pub fn new() -> Self {
        Self {
        //    choice: Choice::new(),
           ..Default::default()
        }
        
    }
    pub fn update(&mut self, msg: GeneralMessage) {
        match msg {
            GeneralMessage::TabSelect(select)=>{
                self.choice = select;
            }
            GeneralMessage::ThemeMsg(msg)=>{
                self.theme.update(msg);
            }
            GeneralMessage::FontMsg(msg)=>{
                self.font_style.update(msg);
            }
            GeneralMessage::IconMsg(msg)=>{
                self.icon_style.update(msg);
            }
        }
    }
    pub fn view(&mut self) -> Element<GeneralMessage> {
        let General {
            choice,
            theme,
            icon_style,
            font_style,
            is_active,
            scroll_content,
        } = self;
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Tab::new(
                    Choice::A,
                    Some(self.choice),
                    GeneralMessage::TabSelect,
                    tab_content('\u{f53f}', "General"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::B,
                    Some(self.choice),
                    GeneralMessage::TabSelect,
                    tab_content('\u{f86d}', "Icon"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::C,
                    Some(self.choice),
                    GeneralMessage::TabSelect,
                    tab_content('\u{f245}', "Cursor"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::D,
                    Some(self.choice),
                    GeneralMessage::TabSelect,
                    tab_content('\u{f031}', "Font"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            );
        let contnet = Column::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .padding(20)
            .push(match self.choice {
                Choice::A => self.theme.view().map(move |msg| GeneralMessage::ThemeMsg(msg)),
                Choice::B => self.icon_style.view().map(move |msg|GeneralMessage::IconMsg(msg)),
                Choice::C => Text::new("C").into(),
                Choice::D => self.font_style.view().map(move |msg| GeneralMessage::FontMsg(msg)),
            });
        let netsidebar_scroll = Scrollable::new(&mut self.scroll_content)
            .push(row)
            .padding(10)
            .scrollbar_width(4)
            .scroller_width(4);
        let whole_content: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(netsidebar_scroll.height(Length::Fill))
                    .style(ContainerStyle::White)
                    .width(Length::FillPortion(4))
                    .height(Length::Fill),
            )
            .push(Rule::vertical(10))
            .push(
                Container::new(contnet.height(Length::Fill))
                    .width(Length::FillPortion(9))
                    .height(Length::Fill)
                    .style(ContainerStyle::White), // .padding(10),
            )
            .into();
        let container = Container::new(whole_content)
            .width(Length::Fill)
            .center_x()
            .center_y();
        Container::new(container)
            .style(ContainerStyle::LightGray)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
    
fn tab_content(unicode: char, name: &str) -> Row<'static, GeneralMessage> {
    Row::new()
        .push(Icon::new(unicode).size(24))
        .push(Text::new(name).size(16))
        .align_items(Align::Center)
        .spacing(8)
}



