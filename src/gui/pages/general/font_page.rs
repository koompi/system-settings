use super::super::super::styles::CustomSelect;
use iced::{pick_list, slider, text_input, Align, Column, Container, Element, HorizontalAlignment, Length, PickList, Row, Slider, Text};
use serde::{Deserialize, Serialize};
use std::default;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, Error};
use toml::{from_str, to_string_pretty};
const HOME: &'static str = env!("HOME");
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
pub struct FontStyle {
    value: f32,
    state: slider::State,
    font: pick_list::State<OutputFont>,
    search: text_input::State,
    array_fonts: Vec<OutputFont>,
    font_output: OutputFont,
}
#[derive(Debug, Clone)]
pub enum FontMsg {
    SliderChange(f32),
    FontChanged(OutputFont),
}
impl FontStyle {
    pub fn new() -> Self {
        let mut fonts: Vec<OutputFont> = vec![];
        create_dir();
        match read_font("/usr/share/fonts/TTF") {
            Ok(list) => {
                for f in list {
                    fonts.push(OutputFont { font: f });
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        let first_font = match fonts.get(0) {
            Some(d) => d.font.clone(),
            None => String::from("Font not found"),
        };
        Self {
            state: slider::State::new(),
            font: pick_list::State::default(),
            search: text_input::State::new(),
            array_fonts: fonts,
            font_output: OutputFont { font: first_font },
            
            ..Self::default()
        }
    }
    pub fn update(&mut self, msg: FontMsg) {
        match msg {
            FontMsg::SliderChange(x) => self.value = x,
            FontMsg::FontChanged(font) => {
                let font_conf = to_string_pretty(&font).unwrap();
                writer("font.conf", &font).unwrap();
                self.font_output = font;
            }
        }
    }
    pub fn view(&mut self) -> Element<FontMsg> {
        let FontStyle { value, state, font_output, font, search, .. } = self;
        let font_size = Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(Text::new("Size").size(24))
            .spacing(10)
            .push(
                Row::new().align_items(Align::Center).spacing(10).push(Slider::new(&mut self.state, 0.0..=100.0, self.value, FontMsg::SliderChange).step(1.0)).push(
                    Row::new()
                        .align_items(Align::Center)
                        .push(Text::new(&self.value.to_string()).horizontal_alignment(HorizontalAlignment::Center).width(Length::Units(20)))
                        .push(Text::new("%")),
                ),
            )
            .spacing(20);
        let font_choice = Column::new().width(Length::Fill).padding(20).push(Text::new("Standard Font:").size(24)).spacing(30).push(
            PickList::new(font, &self.array_fonts, Some(self.font_output.clone()), FontMsg::FontChanged)
                .width(Length::Units(250))
                // .height(Length::Units(20))
                .text_size(18)
                .style(CustomSelect::Default),
        );
        let whole_content = Column::new().align_items(Align::Center).push(font_size).push(font_choice).padding(20).spacing(10);
        Container::new(whole_content).width(Length::Fill).height(Length::Fill).into()
    }
}
pub fn reader(name: &str) -> Result<String, io::Error> {
    let path = std::path::Path::new(format!("{}/.config/koompi/font", HOME).as_str()).join(name);

    std::fs::read_to_string(path)
}
pub fn writer(name: &str, data: &OutputFont) -> Result<(), io::Error> {
    let path = std::path::Path::new(format!("{}/.config/koompi/font", HOME).as_str()).join(name);
    let mut file = File::create(path).unwrap();
    match file.write_all(to_string_pretty(data).unwrap().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
pub fn create_dir() -> std::io::Result<()> {
    fs::create_dir_all(format!("{}/.config/koompi/font", HOME))?;
    Ok(())
}
pub fn read_font<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>, io::Error> {
    let mut entries = fs::read_dir(path)?.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    let mut list_font = vec![];
    for path in &entries {
        list_font.push(path.file_name().unwrap().to_str().unwrap().to_string());
    }
    Ok(list_font)
}
use std::io;
#[derive(Debug, Default, Clone,  PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputFont {
    font: String,
    
}
use std::fmt;
impl fmt::Display for OutputFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.font)
        
    }
}

