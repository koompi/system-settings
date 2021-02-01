use super::super::styles::{CustomButton, CustomContainer, CustomRadio, CustomSelect};
use iced::{
    button, pick_list, scrollable, Align, Button, Checkbox, Column, Container, Element, Length,
    PickList, Radio, Row, Rule, Scrollable, Space, Svg, Text,
};
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
    hide_show_menubar: bool,
    permis: [bool; 4],
    selected_list1: IconSize,
    selected_list2: Hightight,
    // selected_web: WebBrowsers,
    light_btn: button::State,
    dark_btn: button::State,
    icon_size: pick_list::State<IconSize>,
    // highlight: pick_list::State<Hightight>,
    // web: pick_list::State<WebBrowsers>,
    selected: Option<ColorAccent>,
    scroller: Option<ShowSrollBar>,
    scroll_content: scrollable::State,
}
#[derive(Debug, Clone)]
pub enum GeneralMessage {
    CheckboxToggle(bool),
    AskKeepChanged(bool),
    CloseWindowChanged(bool),
    AllowData(bool),
    AllowSmoothFnt(bool),
    SelectIconSize(IconSize),
    // SelectColor(Hightight),
    DarkButton,
    LightButton,
    RadioSwtich(ColorAccent),
    ScrollChanged(ShowSrollBar),
    // BrowserChanged(WebBrowsers),
}

impl General {
    pub fn new() -> Self {
        Self {
            hide_show_menubar: false,
            permis: [true, false, false, true],
            selected_list1: IconSize::default(),
            selected_list2: Hightight::default(),
            // selected_web: WebBrowsers::default(),
            light_btn: button::State::new(),
            dark_btn: button::State::new(),
            icon_size: pick_list::State::default(),
            // highlight: pick_list::State::default(),
            // web: pick_list::State::default(),
            selected: Some(ColorAccent::Purple),
            scroller: Some(ShowSrollBar::MouseTouchPad),
            scroll_content: scrollable::State::new(),
        }
    }
    pub fn update(&mut self, msg: GeneralMessage) {
        match msg {
            GeneralMessage::CheckboxToggle(value) => {
                self.hide_show_menubar = value;
            }
            GeneralMessage::SelectIconSize(icon) => {
                self.selected_list1 = icon;
            }
            // GeneralMessage::SelectColor(color) => {
            //     self.selected_list2 = color;
            // }
            GeneralMessage::DarkButton => {}
            GeneralMessage::LightButton => {}
            // GeneralMessage::BrowserChanged(web) => {
            //     self.selected_web = web;
            // }
            GeneralMessage::AskKeepChanged(value) => {
                self.permis[0] = value;
            }
            GeneralMessage::CloseWindowChanged(value) => {
                self.permis[1] = value;
            }
            GeneralMessage::RadioSwtich(color) => {
                self.selected = Some(color);
            }
            GeneralMessage::AllowData(value) => {
                self.permis[2] = value;
            }
            GeneralMessage::ScrollChanged(scroll) => {
                self.scroller = Some(scroll);
            }
            GeneralMessage::AllowSmoothFnt(value) => {
                self.permis[3] = value;
            }
        }
    }
    pub fn view(&mut self) -> Element<GeneralMessage> {
        let General {
            hide_show_menubar,
            permis,
            selected_list1,
            selected_list2,
            // selected_web,
            light_btn,
            dark_btn,
            icon_size,
            // highlight,
            // web,
            selected,
            scroller,
            scroll_content,
        } = self;
        let checkbox = Checkbox::new(
            *hide_show_menubar,
            "Automatically hide and show the menu bar",
            GeneralMessage::CheckboxToggle,
        );
        let radio_field = ColorAccent::all().iter().cloned().enumerate().fold(
            Row::new(),
            |choices, (index, color)| {
                choices.push(
                    Radio::new(color, "", *selected, GeneralMessage::RadioSwtich)
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
        let last_col = Column::new()
            .width(Length::Fill)
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
                                .on_press(GeneralMessage::LightButton)
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
                                .on_press(GeneralMessage::DarkButton)
                                .min_width(80)
                                .min_height(50)
                                .style(CustomButton::Selected),
                            )
                            .spacing(5)
                            .push(Text::new("Dark")),
                    ),
            )
            .push(radio_field)
            // .push(
            //     PickList::new(
            //         highlight,
            //         &Hightight::ALL[..],
            //         Some(*selected_list2),
            //         GeneralMessage::SelectColor,
            //     )
            //     .style(CustomSelect::Default)
            //     .text_size(16)
            //     .width(Length::Units(150)),
            // )
            .spacing(15)
            .align_items(Align::Start)
            .push(
                PickList::new(
                    icon_size,
                    &IconSize::ALL[..],
                    Some(*selected_list1),
                    GeneralMessage::SelectIconSize,
                )
                .style(CustomSelect::Default)
                .text_size(16)
                .width(Length::Units(150)),
            )
            .push(checkbox);

        let header_section = Row::new()
            .spacing(10)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::End)
                    .spacing(10)
                    .push(Text::new("Appearance:").size(16))
                    .push(Space::with_height(Length::Units(35)))
                    .push(Text::new("Accent color:").size(16))
                    .push(Space::with_height(Length::Units(2)))
                    // .push(Text::new("Hightight color:").size(16))
                    .push(Space::with_height(Length::Units(10)))
                    .push(Text::new("Sidebar icon size:").size(16)),
            )
            .push(last_col);
        // let middle_section = Row::new()
        //     .width(Length::Fill)
        //     .spacing(10)
        //     .align_items(Align::Start)
        //     .push(
        //         Column::new()
        //             .width(Length::Fill)
        //             .align_items(Align::End)
        //             .push(Text::new("Show scroll bars:").size(16)),
        //     )
        //     .push(Column::new().width(Length::Fill).push(
        //         ShowSrollBar::all().iter().cloned().fold(
        //             Column::new().align_items(Align::Start).spacing(4),
        //             |column, choice| {
        //                 column.push(
        //                     Radio::new(choice, choice, *scroller, GeneralMessage::ScrollChanged)
        //                         .size(18),
        //                 )
        //             },
        //         ),
        //     ));
        // let second_section = Row::new()
        //     .width(Length::Fill)
        //     .spacing(10)
        //     .height(Length::Units(40))
        //     .align_items(Align::Center)
        //     .push(
        //         Column::new()
        //             .width(Length::Fill)
        //             .push(Text::new("Default web browser:").size(16))
        //             .align_items(Align::End),
        //     )
        //     .push(
        //         Column::new()
        //             .width(Length::Fill)
        //             .align_items(Align::Start)
        //             .push(
        //                 PickList::new(
        //                     web,
        //                     &WebBrowsers::ALL[..],
        //                     Some(*selected_web),
        //                     GeneralMessage::BrowserChanged,
        //                 )
        //                 .width(Length::Units(150))
        //                 .text_size(16)
        //                 .style(CustomSelect::Default),
        //             ),
        //     );
        // let last_section = Row::new().padding(10)
        //     .width(Length::Fill)
        //     .push(
        //         Column::new()
        //             .width(Length::Fill)
        //             .align_items(Align::End)
        //             .push(Text::new("Others: ").size(16)),
        //     )
        //     .push(Space::with_width(Length::Units(10)))
        //     .push(Column::new().width(Length::Fill).align_items(Align::Start).spacing(10)
        //     .push(Checkbox::new(
        //         permis[0],
        //         "Ask to keep changes when closing documents",
        //         GeneralMessage::AskKeepChanged,
        //     ))
        //     .push(Checkbox::new(
        //         permis[1],
        //         "Close windows when quiting an app",
        //         GeneralMessage::CloseWindowChanged)).push(Row::new().push(Space::with_width(Length::Units(40))).push(Text::new("When selected, open documents and windows will not be restored when you re-open app."))).push(Checkbox::new(permis[2], "Allow external data to transmit through the computer", GeneralMessage::AllowData)));

        // let final_section = Row::new()
        //     .width(Length::Fill)
        //     .spacing(10)
        //     .push(
        //         Column::new()
        //             .width(Length::Fill)
        //             .push(Text::new("Font:").size(16))
        //             .align_items(Align::End),
        //     )
        //     .push(
        //         Column::new()
        //             .width(Length::Fill)
        //             .align_items(Align::Start)
        //             .push(Checkbox::new(
        //                 permis[3],
        //                 "Use Smooth Font when available",
        //                 GeneralMessage::AllowSmoothFnt,
        //             )),
        //     );
        let whole_contetnt = Column::new()
            .align_items(Align::Center)
            .push(header_section)
            // .push(Rule::horizontal(6))
            // .push(middle_section)
            // .push(Rule::horizontal(6))
            // .push(second_section)
            // // .push(Rule::horizontal(6))
            // .push(last_section)
            // .push(Rule::horizontal(6))
            // .push(final_section)
            .padding(10)
            .spacing(10);

        let scroll_list = Scrollable::new(scroll_content).push(whole_contetnt);
        Container::new(scroll_list)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .style(CustomContainer::ForegroundGray)
            .into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum ShowSrollBar {
    MouseTouchPad,
    Scrolling,
    Always,
}

// #[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
// pub enum WebBrowsers {
//     Chrome,
//     Firefox,
//     Brave,
// }
// // impl WebBrowsers {
// //     const ALL: [WebBrowsers; 3] = [
// //         WebBrowsers::Chrome,
// //         WebBrowsers::Firefox,
// //         WebBrowsers::Brave,
// //     ];
// // }
// select_display!(WebBrowsers,
//     WebBrowsers::Chrome => "Chrome",
//     WebBrowsers::Firefox => "Firefox",
//     WebBrowsers::Brave => "Brave"
// );

impl ShowSrollBar {
    fn all() -> [ShowSrollBar; 3] {
        [
            ShowSrollBar::MouseTouchPad,
            ShowSrollBar::Scrolling,
            ShowSrollBar::Always,
        ]
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum Hightight {
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Big,
    Large,
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
        ColorAccent::Purple
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
impl From<ShowSrollBar> for String {
    fn from(language: ShowSrollBar) -> String {
        String::from(match language {
            ShowSrollBar::MouseTouchPad => "Automatically scroll based on Mouse and TouchPad",
            ShowSrollBar::Scrolling => "On Scrolling",
            ShowSrollBar::Always => "Always",
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

// impl Hightight {
//     const ALL: [Hightight; 4] = [
//         Hightight::Blue,
//         Hightight::Green,
//         Hightight::Red,
//         Hightight::Yellow,
//     ];
// }
// select_display!(Hightight,
//     Hightight::Blue => "Blue",
//     Hightight::Green => "Green",
//     Hightight::Red => "Red",
//     Hightight::Yellow => "Yello"
// );

select_display!(IconSize,
    IconSize::Small=> "Small",
    IconSize::Medium => "Medium",
    IconSize::Big => "Big",
    IconSize::Large => "Large"
);

impl Default for IconSize {
    fn default() -> Self {
        IconSize::Medium
    }
}
// impl Default for WebBrowsers {
//     fn default() -> Self {
//         WebBrowsers::Firefox
//     }
// }
impl Default for Hightight {
    fn default() -> Self {
        Hightight::Blue
    }
}
impl IconSize {
    const ALL: [IconSize; 4] = [
        IconSize::Small,
        IconSize::Medium,
        IconSize::Big,
        IconSize::Large,
    ];
}
