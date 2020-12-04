use iced::{
   button, container, text_input, Color, Background, Vector
};

pub enum CustomButton {
   Default,
   Secondary,
   Sidebar,
   Text,
   Selected,
   Tab,
   SelectedTab
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      button::Style {
         text_color: match self {
            CustomButton::Sidebar => Color::from_rgb8(97, 97, 97),
            CustomButton::Selected => Color::from_rgb8(15, 86, 179),
            _ => Color::BLACK
         },
         background: Some(Background::Color(match self {
            CustomButton::Selected | CustomButton::SelectedTab => Color::from_rgba8(15, 86, 179, 0.3),
            CustomButton::Text | CustomButton::Tab => Color::TRANSPARENT,
            _ => Color::WHITE,
         })),
         border_radius: match self {
            CustomButton::Text => 10.0,
            _ => 5.0
         },
         border_color: match self {
            CustomButton::Default | CustomButton::Secondary => Color::BLACK,
            _ => Color::TRANSPARENT
         },
         border_width: match self {
            CustomButton::Secondary => 1.0,
            _ => 0.0
         },
         shadow_offset: match self {
            CustomButton::Default | CustomButton::Secondary => Vector::new(0.5, 1.0),
            _ => Vector::new(0.0, 0.0),
         }
      }
   }

   fn hovered(&self) -> button::Style {
      let active = self.active();

      match self {
         CustomButton::Sidebar | CustomButton::Text | CustomButton::Tab => button::Style {
            background: Some(Background::Color(Color {
               a: 0.2,
               ..active.text_color
            })),
            ..active
         },
         _ => active
      }
   }
}

pub enum CustomContainer {
   Background,
   ForegroundWhite,
   ForegroundGray,
   Header,
   Segment,
   FadedBrightForeground
}

impl container::StyleSheet for CustomContainer {
   fn style(&self) -> container::Style {
      container::Style {
         background: Some(Background::Color(match self {
            CustomContainer::Background => Color::from_rgb8(238, 238, 238),
            CustomContainer::ForegroundWhite => Color::WHITE,
            CustomContainer::ForegroundGray => Color::from_rgb8(224, 224, 224),
            CustomContainer::Segment => Color::TRANSPARENT,
            CustomContainer::Header => Color::from_rgb8(238, 238, 238),
            CustomContainer::FadedBrightForeground => Color::from_rgba8(238, 238, 238, 0.8)
         })),
         border_radius: match self {
            CustomContainer::ForegroundGray | CustomContainer::Segment => 7.0,
            CustomContainer::FadedBrightForeground => 4.0,
            _ => 0.0
         },
         border_width: match self {
            CustomContainer::Header | CustomContainer::Segment => 0.5,
            _ => 0.0
         },
         border_color: match self {
            CustomContainer::Header => Color::TRANSPARENT,
            CustomContainer::Segment => Color::from_rgb8(15, 86, 179),
            _ => Color::from_rgb8(238, 238, 238),
         },
         ..container::Style::default()
      }
   }
}

pub enum CustomTextInput {
   Default
}

impl text_input::StyleSheet for CustomTextInput {
   fn active(&self) -> text_input::Style {
      text_input::Style {
         background: Background::Color(match self {
            Self::Default => Color::from_rgb8(238, 238, 238),
         }),
         border_radius: 12.0,
         border_width: 0.0,
         border_color: match self {
            Self::Default => Color::from_rgb8(41, 98, 255),
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
         Self::Default => Color::from_rgb8(189, 195, 199),
      }
   }

   fn value_color(&self) -> Color {
      self.active().border_color
   }

   fn selection_color(&self) -> Color {
      match self {
         Self::Default => Color::from_rgb8(189, 195, 199)
      }
   }

   fn hovered(&self) -> text_input::Style {
      self.focused()
   }
}