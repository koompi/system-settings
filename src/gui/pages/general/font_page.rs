use iced::{Text, text_input, Column, PickList, Slider,slider, pick_list, Element, Align, Length, Container,HorizontalAlignment, Row};
use super::super::super::styles::{CustomSelect};
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
            // FontMsg::FontChanged(value) => self.font_size = value,
            
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
                Row::new()
                    .align_items(Align::Center)
                    .spacing(10)
                    .push(     
                        Slider::new(
                            &mut self.state,
                            0.0..=100.0,
                            self.value,
                            FontMsg::SliderChange,
                        )
                        .step(1.0),
                        
                    )
                    .push(
                        Row::new()
                            .align_items(Align::Center)
                            .push(Text::new(&self.value.to_string()).horizontal_alignment(HorizontalAlignment::Center).width(Length::Units(20)))
                            .push(Text::new("%")),
                    )
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
