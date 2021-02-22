use super::super::styles::{CustomButton, CustomContainer, CustomRadio, CustomSelect};
use iced::{Align, Button, Checkbox, Column, Command, Container, Element, Length, PickList, Radio, Row, Rule, Scrollable, Space, Svg, Text, button, pick_list, scrollable, slider,Slider, TextInput,text_input};
use iced_custom_widget as icw;
use icw::components::Tab;
use icw::components::Icon;
use icw::styles::{
     containers::ContainerStyle};
use serde::de::value;

use crate::helpers::ROOT_PATH;
#[macro_export]
macro_rules! select_display {
    ($name:ident, $($key:path => $value:expr),+ ) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", match self {
                       $($key => $value),+
                })
            }
        }
    };
}
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
                Choice::B => Text::new("B").into(),
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

#[derive(Debug, Default, Clone)]
pub struct Theme{
    light_btn: button::State,
    dark_btn: button::State,
    selected: Option<ColorAccent>,
    scroll_content: scrollable::State,
}
#[derive(Debug, Clone)]
pub enum ThemeMsg {
    DarkButton,
    LightButton,
    RadioSwtich(ColorAccent),
}
impl Theme{
    pub fn new() -> Self{
        Self{
            light_btn: button::State::new(),
            dark_btn: button::State::new(),
            selected: Some(ColorAccent::Purple),
            scroll_content: scrollable::State::new(),
            ..Self::default()   
        }
        
    }
    pub fn update(&mut self, msg: ThemeMsg){
        match msg{
            ThemeMsg::DarkButton => {}
            ThemeMsg::LightButton => {}
            ThemeMsg::RadioSwtich(color) => {
                self.selected = Some(color);
            }
        
        }
    }
    pub fn view(&mut self) -> Element<ThemeMsg>{
        let Theme {
            light_btn,
            dark_btn,
            selected,
            scroll_content,
        } = self;
        let radio_field = ColorAccent::all().iter().cloned().enumerate().fold(
            Row::new(),
            |choices, (index, color)| {
                choices.push(
                    Radio::new(color, "", *selected, ThemeMsg::RadioSwtich)
                        .size(18)
                        .style(match index {
                            0 => CustomRadio::Purple,
                            1 => CustomRadio::Green,
                            2 => CustomRadio::Blue,
                            3 => CustomRadio::Yellow,
                            4 => CustomRadio::Pink,
                            5 => CustomRadio::Gray,
                            6 => CustomRadio::Orange,
                            _ => CustomRadio::Default,
                        }),
                )
            },
        );
        let appearent = Column::new()
            .width(Length::Fill)
            .push(  
            Text::new("Theme").size(24),
            )
            .push(
                Row::new()
                    .spacing(16)
                    .push(
                        Column::new()
                            .align_items(Align::Center)
                            .push(
                                Button::new(
                                    light_btn,
                                    Svg::from_path(format!(
                                        "{}/assets/images/light.svg",
                                        ROOT_PATH()
                                    ))
                                    .width(Length::Units(64))
                                    .height(Length::Units(64)),
                                )
                                .on_press(ThemeMsg::LightButton)
                                .min_width(80)
                                .min_height(50)
                                .style(CustomButton::Selected),
                            )
                            .spacing(5)
                            .push(Text::new("Light")),
                    )
                    .push(
                        Column::new()
                            .align_items(Align::Center)
                            .push(
                                Button::new(
                                    dark_btn,
                                    Svg::from_path(format!(
                                        "{}/assets/images/dark.svg",
                                        ROOT_PATH()
                                    ))
                                    .width(Length::Units(64))
                                    .height(Length::Units(64)),
                                )
                                .on_press(ThemeMsg::DarkButton)
                                .min_width(80)
                                .min_height(50)
                                .style(CustomButton::Selected),
                            )
                            .spacing(5)
                            .push(Text::new("Dark")),
                    ),
            )
            .push(
                Text::new("Accent Color").size(24)
            )
            .push(radio_field)
            .spacing(15)
            .align_items(Align::Start);

            let whole_contetnt = Column::new()
            .align_items(Align::Center)
            .push(appearent)
            .padding(10)
            .spacing(10);

        let scroll_list = Scrollable::new(scroll_content).push(whole_contetnt);
        Container::new(scroll_list)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorAccent {
    Blue,
    Purple,
    Pink,
    Orange,
    Yellow,
    Green,
    Gray,
}
impl Default for ColorAccent {
    fn default() -> Self {
        ColorAccent::Green
    }
}
impl From<ColorAccent> for String {
    fn from(language: ColorAccent) -> String {
        String::from(match language {
            ColorAccent::Purple => "Purple",
            ColorAccent::Green => "Green",
            ColorAccent::Blue => "Blue",
            ColorAccent::Yellow => "Yellow",
            ColorAccent::Pink => "Pink",
            ColorAccent::Gray => "Gray",
            ColorAccent::Orange => "Orange",
        })
    }
} 
impl ColorAccent {
    fn all() -> [ColorAccent; 7] {
        [
            ColorAccent::Purple,
            ColorAccent::Green,
            ColorAccent::Blue,
            ColorAccent::Yellow,
            ColorAccent::Pink,
            ColorAccent::Gray,
            ColorAccent::Orange,
        ]
    }
}
#[derive(Default, Debug, Clone)]
pub struct FontStyle{
    value : f32,
    state:slider::State,
    selected_font: FontList,
    font: pick_list::State<FontList>,
    search: text_input::State,
}
#[derive(Debug,Clone, Copy)]
pub enum FontMsg{
    SliderChange(f32),
    FontChanged(FontList),

}
impl FontStyle{
    pub fn new() -> Self{
        Self{
            state:slider::State::new(),
            selected_font: FontList::default(),
            font: pick_list::State::default(),
            search: text_input::State::new(),
            ..Self::default()
        }
        
    }
    pub fn update(&mut self, msg: FontMsg){
        match msg{
            FontMsg::SliderChange(x) => self.value = x,   
            FontMsg::FontChanged(font) => {
                self.selected_font = font;
            }
            
        }
            
    }
    pub fn view(&mut self) -> Element<FontMsg>{
        let FontStyle { 
            value, 
            state, 
            selected_font, 
            font,
            search,
        } = self;
        let font_size=Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(
                Text::new("Size").size(24),
            )
            .spacing(10)
            .push(
                
                Slider::new(
                    &mut self.state,
                    0.0..=100.0,
                    self.value,
                    FontMsg::SliderChange,
                )
                .step(0.01),
            )
            .spacing(20);
        let font_choice=
            Column::new()
            .width(Length::Fill)
            .padding(20)  
            .push (Text::new("Standard Font:").size(24))
            .spacing(20)
            .push(
                PickList::new(
                    font,
                    &FontList::ALL[..],
                    Some(*selected_font),
                    FontMsg::FontChanged,
                )
                .width(Length::Units(250))
                // .height(Length::Units(20))
                .text_size(18)
                .style(CustomSelect::Default),
            );
         

        let whole_content = Column::new()
        .align_items(Align::Center)
        .push(font_size)
        .push(font_choice)
        .padding(20)
        .spacing(10);
        Container::new(whole_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
        
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum FontList {
    Monospace,
    Arial,
    Serif,  
}
impl FontList {
    const ALL: [FontList; 3] = [
        FontList::Monospace,
        FontList::Arial,
        FontList::Serif,
    ];
}
select_display!(FontList,
    FontList::Monospace => "Monospace",
    FontList::Arial => "Arial",
    FontList::Serif => "Serif"
);
impl Default for FontList {
    fn default() -> Self {
        FontList::Monospace
    }
}

//icon part

#[derive(Default, Debug, Clone)]
pub struct IconStyle{
    
}
#[derive(Debug,Clone, Copy)]
pub enum IconMsg{
  

}
impl IconStyle{
    pub fn new() -> Self{
        Self{
            
            ..Self::default()
        }
        
    }
    pub fn update(&mut self, msg: IconMsg){
        match msg{
            
            }
            
    }
            
    // pub fn view(&mut self) -> Element<IconMsg>{
    //     // let IconStyle { 
            
    //     // } = self;
            
        
        
    // }
}
