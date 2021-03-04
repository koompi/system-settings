use iced::{Button,button, Scrollable,scrollable, Row,Element,Radio, Column,Text, Length,Align, Container, Svg};
use super::super::super::styles::{CustomButton, CustomRadio};
// use crate::gui::styles::containers::ContainerStyle;
use crate::helpers::ROOT_PATH;
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