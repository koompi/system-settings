use iced::{button, checkbox, container, pick_list, progress_bar, slider, text_input, Background, Color, Vector};

pub const BACKGROUND: Color = Color::from_rgb(238.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0);
pub const FOREGROUND: Color = Color::from_rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0);
pub const HOVERED: Color = Color::from_rgb(129.0 / 255.0, 129.0 / 255.0, 129.0 / 255.0);
pub const ACCENT: Color = Color::from_rgb(15.0 / 255.0, 85.0 / 255.0, 179.0 / 255.0);
pub const SUCCESS: Color = Color::from_rgb(31.0 / 255.0, 139.0 / 255.0, 36.0 / 255.0);
pub const WARNING: Color = Color::from_rgb(212.0 / 255.0, 176.0 / 255.0, 17.0 / 255.0);
pub const ERROR: Color = Color::from_rgb(218.0 / 255.0, 16.0 / 255.0, 11.0 / 255.0);

pub enum CustomButton {
    Default,
    Secondary,
    Text,
    Selected,
    Sidebar,
    SelectedSidebar,
    Tab,
    SelectedTab,
    Card,
    SelectedCard,
    Primary,
    Apply,
    Delete,
    Type,
    SelectType,
    Cancel,
    Hovered,
}

impl button::StyleSheet for CustomButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: match self {
                CustomButton::SelectedCard | CustomButton::SelectedSidebar => ACCENT,
                CustomButton::Hovered => Color::WHITE,
                CustomButton::Primary => Color::WHITE,
                CustomButton::Apply => Color::WHITE,
                CustomButton::Delete => Color::WHITE,
                CustomButton::SelectType => Color::BLACK,
                CustomButton::Cancel => Color::WHITE,
                _ => Color::BLACK,
            },
            background: Some(Background::Color(match self {
                CustomButton::Selected | CustomButton::SelectedCard | CustomButton::SelectedSidebar => Color { a: 0.3, ..ACCENT },
                CustomButton::SelectedTab | CustomButton::Card => Color::WHITE,
                CustomButton::Text | CustomButton::Tab | CustomButton::Sidebar => Color::TRANSPARENT,
                CustomButton::Primary => ACCENT,
                CustomButton::Apply => Color::from_rgb8(39, 174, 96),
                CustomButton::Delete => Color::from_rgb8(255, 56, 56),
                CustomButton::Type => Color::WHITE,
                CustomButton::Cancel => Color::from_rgb8(119, 140, 163),
                CustomButton::SelectType => Color::from_rgb8(209, 216, 224),
                CustomButton::Hovered => HOVERED,
                _ => Color::WHITE,
            })),
            border_radius: match self {
                CustomButton::Card | CustomButton::SelectedCard => 12.0,
                CustomButton::Type | CustomButton::SelectType => 0.0,
                CustomButton::Tab | CustomButton::SelectedTab | CustomButton::Hovered | CustomButton::Text | CustomButton::Selected | CustomButton::Sidebar | CustomButton::SelectedSidebar => 7.0,
                _ => 5.0,
            },
            border_color: match self {
                CustomButton::Default | CustomButton::Secondary => Color::BLACK,
                CustomButton::SelectType => Color::from_rgb8(253, 150, 68),
                _ => Color::TRANSPARENT,
            },
            border_width: match self {
                CustomButton::Secondary | CustomButton::Tab | CustomButton::SelectedTab => 1.0,
                CustomButton::SelectType => 2.0,
                _ => 0.0,
            },
            shadow_offset: match self {
                CustomButton::Default | CustomButton::Secondary => Vector::new(0.5, 1.0),
                _ => Vector::new(0.0, 0.0),
            },
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        match self {
            CustomButton::Sidebar | CustomButton::Text | CustomButton::Tab | CustomButton::Card => button::Style {
                background: Some(Color { a: 0.3, ..HOVERED }.into()),
                ..active
            },
            CustomButton::Apply => button::Style {
                background: Some(Background::Color(Color::from_rgb8(46, 204, 113))),
                ..active
            },
            CustomButton::Cancel => button::Style {
                background: Some(Background::Color(Color::from_rgb8(165, 177, 194))),
                ..active
            },
            CustomButton::Type => button::Style {
                background: Some(Background::Color(Color::from_rgb8(209, 216, 224))),
                ..active
            },
            CustomButton::SelectType => button::Style { ..active },
            CustomButton::Delete => button::Style {
                background: Some(Background::Color(Color::from_rgb8(255, 77, 77))),
                ..active
            },
            _ => active,
        }
    }
}

pub enum CustomContainer {
    Background,
    ForegroundWhite,
    ForegroundGray,
    Header,
    Segment,
    FadedBrightForeground,
    Hovered,
    Primary,
    Success,
    Warning,
    Transparent(Color),
}

impl container::StyleSheet for CustomContainer {
    fn style(&self) -> container::Style {
        use CustomContainer::*;
        container::Style {
            background: Some(
                match self {
                    Background | Header => BACKGROUND,
                    ForegroundWhite => Color::WHITE,
                    ForegroundGray | Segment => FOREGROUND,
                    Hovered => Color { a: 0.2, ..Color::BLACK },
                    FadedBrightForeground => Color { a: 0.8, ..FOREGROUND },
                    Primary => Color { a: 0.7, ..ACCENT },
                    Success => SUCCESS,
                    Warning => WARNING,
                    Transparent(color) => Color { a: 0.3, ..(*color) },
                }
                .into(),
            ),
            border_radius: match self {
                Segment => 10.0,
                ForegroundGray | Hovered => 7.0,
                FadedBrightForeground => 4.0,
                Success | Warning | Primary => 5.0,
                _ => 0.0,
            },
            border_width: match self {
                Header | Segment => 1.0,
                Primary => 0.5,
                _ => 0.0,
            },
            border_color: match self {
                Header => Color::TRANSPARENT,
                Primary => Color::BLACK,
                _ => BACKGROUND,
            },
            ..container::Style::default()
        }
    }
}

pub enum CustomTextInput {
    Default,
}

impl text_input::StyleSheet for CustomTextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(match self {
                Self::Default => BACKGROUND,
            }),
            border_radius: 12.0,
            border_width: 0.0,
            border_color: match self {
                Self::Default => ACCENT,
            },
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style { border_width: 1.0, ..self.active() }
    }

    fn placeholder_color(&self) -> Color {
        match self {
            Self::Default => HOVERED,
        }
    }

    fn value_color(&self) -> Color {
        self.active().border_color
    }

    fn selection_color(&self) -> Color {
        match self {
            Self::Default => HOVERED,
        }
    }

    fn hovered(&self) -> text_input::Style {
        self.focused()
    }
}

use iced::radio;
#[allow(dead_code)]
pub enum CustomRadio {
    Default,
    Blue,
    Purple,
    Pink,
    BoldPink,
    Orange,
    Yellow,
    Green,
    Gray,
    Disactive,
    Active,
}

impl radio::StyleSheet for CustomRadio {
    fn active(&self) -> radio::Style {
        match self {
            CustomRadio::Default => radio::Style {
                background: Background::Color(Color::from_rgb8(95, 39, 205)),
                dot_color: Color::from_rgb8(87, 101, 116),
                border_width: 2.0,
                border_color: Color::from_rgb8(95, 39, 205),
            },
            CustomRadio::Blue => radio::Style {
                background: Background::Color(Color::from_rgb8(9, 132, 227)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Purple => radio::Style {
                background: Background::Color(Color::from_rgb8(142, 68, 173)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Pink => radio::Style {
                background: Background::Color(Color::from_rgb8(253, 121, 168)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::BoldPink => radio::Style {
                background: Background::Color(Color::from_rgb8(232, 67, 147)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Orange => radio::Style {
                background: Background::Color(Color::from_rgb8(255, 118, 117)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Yellow => radio::Style {
                background: Background::Color(Color::from_rgb8(254, 202, 87)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Green => radio::Style {
                background: Background::Color(Color::from_rgb8(32, 191, 107)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Gray => radio::Style {
                background: Background::Color(Color::from_rgb8(119, 140, 163)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Disactive => radio::Style {
                background: Color::WHITE.into(),
                dot_color: ACCENT,
                border_width: 1.5,
                border_color: HOVERED,
            },
            CustomRadio::Active => radio::Style {
                background: Color::WHITE.into(),
                dot_color: ACCENT,
                border_width: 1.5,
                border_color: ACCENT,
            },
        }
    }
    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: match self {
                CustomRadio::Active | CustomRadio::Disactive => self.active().background,
                _ => Background::Color(Color::from_rgb8(75, 101, 132)),
            },
            ..self.active()
        }
    }
}

pub enum CustomSelect {
    Default,
    Primary,
}

use iced_style::menu;
impl pick_list::StyleSheet for CustomSelect {
    fn menu(&self) -> menu::Style {
        let default: menu::Style = Default::default();
        menu::Style {
            selected_background: match self {
                CustomSelect::Primary => ACCENT.into(),
                _ => default.selected_background,
            },
            ..default
        }
    }
    fn active(&self) -> pick_list::Style {
        match self {
            CustomSelect::Default => pick_list::Style {
                text_color: Color::WHITE,
                border_radius: 3.0,
                border_width: 1.0,
                icon_size: 0.5,
                border_color: Color::from_rgb8(119, 140, 163),
                background: Background::Color(Color::from_rgb8(165, 177, 194)),
            },
            CustomSelect::Primary => pick_list::Style {
                text_color: Color::BLACK,
                background: Color { a: 0.3, ..ACCENT }.into(),
                icon_size: 0.5,
                border_color: ACCENT,
                border_radius: 5.0,
                border_width: 0.,
            },
        }
    }
    fn hovered(&self) -> pick_list::Style {
        let active = self.active();

        pick_list::Style {
            background: match self {
                CustomSelect::Default => Background::Color(Color::from_rgb8(75, 101, 132)),
                _ => active.background,
            },
            ..active
        }
    }
}

pub enum CustomSlider {
    Default,
}

impl slider::StyleSheet for CustomSlider {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (ACCENT, Color::TRANSPARENT),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 9.0 },
                color: ACCENT,
                border_width: match self {
                    CustomSlider::Default => 0.0,
                },
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        self.active()
    }

    fn dragging(&self) -> slider::Style {
        self.hovered()
    }
}

pub enum CustomProgressBar {
    Default,
    ForegroundGrey,
}

impl progress_bar::StyleSheet for CustomProgressBar {
    fn style(&self) -> progress_bar::Style {
        match self {
            CustomProgressBar::Default => progress_bar::Style {
                background: FOREGROUND.into(),
                bar: ACCENT.into(),
                border_radius: 7.0,
            },
            CustomProgressBar::ForegroundGrey => progress_bar::Style {
                background: Background::Color(Color::from_rgb8(178, 190, 195)),
                bar: ACCENT.into(),
                border_radius: 7.0,
            },
        }
    }
}

pub enum CustomCheckbox {
    Default,
}

impl checkbox::StyleSheet for CustomCheckbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: if is_checked { ACCENT } else { Color::WHITE }.into(),
            checkmark_color: Color::WHITE,
            border_radius: 5.0,
            border_width: 1.5,
            border_color: if is_checked { ACCENT } else { HOVERED }.into(),
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        self.active(is_checked)
    }
}

pub mod buttons {
    use iced::{button, Background, Color, Vector};
    pub enum ButtonStyle {
        Default,
        Circular(u8, u8, u8, f32),
        BigCircular(u8, u8, u8, f32),
        CircleRadius(u8, u8, u8, f32, f32, Color),
        Transparent,
    }

    impl button::StyleSheet for ButtonStyle {
        fn active(&self) -> button::Style {
            button::Style {
                shadow_offset: Vector::new(0.0, 0.0),
                background: match self {
                    ButtonStyle::Default => Some(Background::Color([0.87, 0.87, 0.87].into())),
                    ButtonStyle::Circular(c1, c2, c3, p) | ButtonStyle::CircleRadius(c1, c2, c3, p, _, _) | ButtonStyle::BigCircular(c1, c2, c3, p) => Some(Background::Color(Color::from_rgba8(*c1, *c2, *c3, *p))),
                    ButtonStyle::Transparent => Some(Background::Color(Color::TRANSPARENT)),
                },
                border_radius: match self {
                    ButtonStyle::Default | ButtonStyle::Circular(_, _, _, _) => 4.0,
                    ButtonStyle::BigCircular(_, _, _, _) => 25.0,
                    ButtonStyle::Transparent => 0.0,
                    ButtonStyle::CircleRadius(_, _, _, _, r, _) => *r,
                },
                border_width: 0.0,
                border_color: [0.7, 0.7, 0.7].into(),
                text_color: match self {
                    ButtonStyle::Default | ButtonStyle::BigCircular(_, _, _, _) | ButtonStyle::Circular(_, _, _, _) => Color::WHITE,
                    ButtonStyle::Transparent => Color::BLACK,
                    ButtonStyle::CircleRadius(_, _, _, _, _, color) => *color,
                },
            }
        }
    }
}

pub mod containers {
    use iced::{container, Background, Color};
    pub enum ContainerStyle {
        Custom,
        InkColor,
        LightGray,
        White,
        LightGrayCircle,
        Black,
    }
    impl container::StyleSheet for ContainerStyle {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: None,
                background: match self {
                    ContainerStyle::Custom => Some(Background::Color(Color::from_rgba8(223, 228, 234, 1.0))),
                    ContainerStyle::InkColor => Some(Background::from(Color::from_rgba8(206, 214, 224, 1.0))),
                    ContainerStyle::LightGray => Some(Background::from(Color::from_rgba8(215, 219, 221, 1.0))),
                    ContainerStyle::White => Some(Background::from(Color::from_rgba8(255, 255, 255, 1.0))),
                    ContainerStyle::LightGrayCircle => Some(Background::from(Color::from_rgba8(215, 219, 221, 0.5))),
                    ContainerStyle::Black => Some(Background::from(Color::BLACK)),
                },
                border_radius: match self {
                    ContainerStyle::Custom | ContainerStyle::LightGrayCircle | ContainerStyle::White | ContainerStyle::InkColor | ContainerStyle::Black => 10.0,
                    ContainerStyle::LightGray => 0.0,
                },
                border_width: 0.0,
                border_color: Color::from_rgba8(255, 255, 255, 1.0),
            }
        }
    }
}

pub mod rules {
    use iced::{rule, Color};
    pub struct RuleStyle;

    impl rule::StyleSheet for RuleStyle {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: Color::WHITE,
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Percent(100.0),
            }
        }
    }
}

pub mod textinput {
    use iced::{text_input, Background, Color};
    pub enum InputStyle {
        Default,
        CircularBorder,
        InkBorder,
    }

    impl text_input::StyleSheet for InputStyle {
        fn active(&self) -> text_input::Style {
            text_input::Style {
                background: Background::Color(Color::from_rgba8(215, 219, 221, 0.5)),
                border_radius: 8.0,
                border_width: 0.0,
                border_color: Color::from_rgb(0.7, 0.7, 0.7),
            }
        }

        fn focused(&self) -> text_input::Style {
            text_input::Style {
                border_color: Color::from_rgb(0.5, 0.5, 0.5),
                background: Background::from(Color::from_rgba8(215, 219, 221, 0.5)),
                border_width: match self {
                    InputStyle::Default => 1.0,
                    InputStyle::CircularBorder | InputStyle::InkBorder => 2.0,
                },
                ..self.active()
            }
        }

        fn placeholder_color(&self) -> Color {
            Color::from_rgb(0.7, 0.7, 0.7)
        }

        fn value_color(&self) -> Color {
            Color::from_rgba8(86, 101, 115, 1.0)
        }

        fn selection_color(&self) -> Color {
            Color::from_rgba(1.0, 1.0, 1.0, 1.0)
        }
    }
}

pub mod progressbar {
    use iced::{progress_bar, Background, Color};
    pub enum SliderStyle {
        Default,
        Circle(u8, u8, u8, f32, f32),
        BigCircle(u8, u8, u8, f32, f32),
        WhiteGrayCircle(u8, u8, u8, f32, f32),
    }

    impl progress_bar::StyleSheet for SliderStyle {
        fn style(&self) -> progress_bar::Style {
            progress_bar::Style {
                background: Background::Color(Color::from_rgb(0.6, 0.6, 0.6)),
                bar: match self {
                    SliderStyle::WhiteGrayCircle(r, b, g, alpha, _) | SliderStyle::Circle(r, b, g, alpha, _) | SliderStyle::BigCircle(r, b, g, alpha, _) => Background::Color(Color::from_rgba8(*r, *b, *g, *alpha)),
                    SliderStyle::Default => Background::Color(Color::from_rgb(0.3, 0.9, 0.3)),
                },
                border_radius: match self {
                    SliderStyle::WhiteGrayCircle(_, _, _, _, r) | SliderStyle::BigCircle(_, _, _, _, r) | SliderStyle::Circle(_, _, _, _, r) => *r,
                    SliderStyle::Default => 5.0,
                },
            }
        }
    }
}

pub mod picklist {
    use iced::{
        pick_list::{self, Menu},
        Background, Color,
    };
    pub struct PickListStyle;

    impl pick_list::StyleSheet for PickListStyle {
        fn menu(&self) -> Menu {
            Menu {
                text_color: Color::BLACK,
                background: Background::Color(Color::from_rgba8(215, 219, 221, 1.0)),
                border_width: 0.5,
                border_color: [0.7, 0.7, 0.7].into(),
                selected_text_color: Color::WHITE,
                selected_background: Background::Color(Color::from_rgba8(86, 101, 115, 1.0)),
            }
        }
        fn active(&self) -> pick_list::Style {
            pick_list::Style {
                text_color: Color::BLACK,
                background: Background::Color(Color::from_rgba8(215, 219, 221, 0.5)),
                border_radius: 10.0,
                border_width: 0.0,
                border_color: Color::from_rgba(1.0, 1.0, 1.0, 1.0),
                icon_size: 0.5,
            }
        }

        fn hovered(&self) -> pick_list::Style {
            pick_list::Style { border_color: Color::BLACK, ..self.active() }
        }
    }
}

pub mod sliders {
    use iced::{
        slider::{self, Handle, HandleShape},
        Background, Color,
    };
    pub enum SliderStyle {
        Default,
        Circle(f32),
    }

    impl slider::StyleSheet for SliderStyle {
        fn active(&self) -> slider::Style {
            slider::Style {
                rail_colors: (Color::from_rgba8(128, 139, 150, 1.0), Color::from_rgba8(128, 139, 150, 1.0)),
                handle: Handle {
                    shape: match self {
                        SliderStyle::Default => HandleShape::Rectangle { width: 24, border_radius: 8.0 },
                        SliderStyle::Circle(input_radius) => HandleShape::Circle { radius: *input_radius },
                    },
                    color: Color::from_rgba8(128, 139, 150, 1.5),
                    border_color: Color::from_rgba8(44, 62, 80, 1.0),
                    border_width: 1.0,
                },
            }
        }
        fn hovered(&self) -> slider::Style {
            let active = self.active();
            slider::Style {
                handle: Handle {
                    color: Color::from_rgba8(205, 213, 203, 1.0),
                    ..active.handle
                },
                ..active
            }
        }
        fn dragging(&self) -> slider::Style {
            let active = self.active();

            slider::Style {
                handle: Handle {
                    color: Color::from_rgba8(205, 213, 203, 1.0),
                    ..active.handle
                },
                ..active
            }
        }
    }
}
