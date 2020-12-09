use iced::{
   button, checkbox, container, progress_bar, slider, text_input, Background, Color, Vector,
};

const BACKGROUND: Color = Color::from_rgb(238.0/255.0, 238.0/255.0, 238.0/255.0);
const FOREGROUND: Color = Color::from_rgb(224.0/255.0, 224.0/255.0, 224.0/255.0);
pub const ACCENT: Color = Color::from_rgb(15.0/255.0, 86.0/255.0, 179.0/255.0);
const ACTIVE: Color = Color::from_rgb(41.0/255.0, 98.0/255.0, 1.0);
const HOVERED: Color = Color::from_rgb(189.0/255.0, 195.0/255.0, 199.0/255.0);

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
   Primary
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      button::Style {
         text_color: match self {
            CustomButton::SelectedCard | CustomButton::SelectedSidebar => ACCENT,
            CustomButton::Sidebar => Color::from_rgb8(97, 97, 97),
            CustomButton::Primary => Color::WHITE,
            _ => Color::BLACK,
         },
         background: Some(Background::Color(match self {
            CustomButton::Selected | CustomButton::SelectedCard | CustomButton::SelectedSidebar => {
               Color { a: 0.3, ..ACCENT }
            }
            CustomButton::SelectedTab | CustomButton::Card => Color::WHITE,
            CustomButton::Text | CustomButton::Tab | CustomButton::Sidebar => Color::TRANSPARENT,
            CustomButton::Primary => ACCENT,
            _ => Color::WHITE,
         })),
         border_radius: match self {
            CustomButton::Card | CustomButton::SelectedCard => 12.0,
            CustomButton::Tab | CustomButton::SelectedTab => 7.0,
            _ => 5.0,
         },
         border_color: match self {
            CustomButton::Default | CustomButton::Secondary => Color::BLACK,
            _ => Color::TRANSPARENT,
         },
         border_width: match self {
            CustomButton::Secondary | CustomButton::Tab | CustomButton::SelectedTab => 1.0,
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
         CustomButton::Sidebar | CustomButton::Text | CustomButton::Tab | CustomButton::Card => {
            button::Style {
               background: Some(Color { a: 0.3, ..HOVERED }.into()),
               ..active
            }
         }
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
}

impl container::StyleSheet for CustomContainer {
   fn style(&self) -> container::Style {
      container::Style {
         background: Some(Background::Color(match self {
            CustomContainer::Background | CustomContainer::Header => BACKGROUND,
            CustomContainer::ForegroundWhite => Color::WHITE,
            CustomContainer::ForegroundGray | CustomContainer::Segment => FOREGROUND,
            CustomContainer::Hovered => Color {
               a: 0.2,
               ..Color::BLACK
            },
            CustomContainer::FadedBrightForeground => Color {
               a: 0.8,
               ..FOREGROUND
            },
            CustomContainer::Primary => Color { a: 0.7, ..ACCENT },
         })),
         border_radius: match self {
            CustomContainer::Segment => 10.0,
            CustomContainer::ForegroundGray | CustomContainer::Hovered => 7.0,
            CustomContainer::FadedBrightForeground => 4.0,
            _ => 0.0,
         },
         border_width: match self {
            CustomContainer::Header | CustomContainer::Segment => 1.0,
            CustomContainer::Primary => 0.5,
            _ => 0.0,
         },
         border_color: match self {
            CustomContainer::Header => Color::TRANSPARENT,
            CustomContainer::Primary => Color::BLACK,
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
      text_input::Style {
         border_width: 1.0,
         ..self.active()
      }
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
            background: HOVERED.into(),
            dot_color: Color::WHITE,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
         },
         CustomRadio::Active => radio::Style {
            background: ACTIVE.into(),
            dot_color: Color::WHITE,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
         },
      }
   }
   fn hovered(&self) -> radio::Style {
      radio::Style {
         background: match self {
            CustomRadio::Active => ACTIVE.into(),
            CustomRadio::Disactive => HOVERED.into(),
            _ => Background::Color(Color::from_rgb8(75, 101, 132)),
         },
         ..self.active()
      }
   }
}

use iced::pick_list;
pub enum CustomSelect {
   Default,
}
use iced_style::menu;
impl pick_list::StyleSheet for CustomSelect {
   fn menu(&self) -> menu::Style {
      menu::Style {
         ..Default::default()
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
      }
   }
   fn hovered(&self) -> pick_list::Style {
      pick_list::Style {
         background: Background::Color(Color::from_rgb8(75, 101, 132)),
         ..self.active()
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
}

impl progress_bar::StyleSheet for CustomProgressBar {
   fn style(&self) -> progress_bar::Style {
      progress_bar::Style {
         background: FOREGROUND.into(),
         bar: ACCENT.into(),
         border_radius: 7.0,
      }
   }
}

pub enum CustomCheckbox {
   Default,
}

impl checkbox::StyleSheet for CustomCheckbox {
   fn active(&self, is_checked: bool) -> checkbox::Style {
      checkbox::Style {
         background: if is_checked { ACCENT } else { HOVERED }.into(),
         checkmark_color: Color::WHITE,
         border_radius: 5.0,
         border_width: 0.0,
         border_color: ACCENT,
      }
   }

   fn hovered(&self, is_checked: bool) -> checkbox::Style {
      self.active(is_checked)
   }
}
