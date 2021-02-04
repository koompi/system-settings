use super::date_time_page::DateTimeMessage;
use crate::gui::styles::{HOVERED, ERROR};
use chrono::prelude::*;
use iced::{
   canvas::{self, Cache, Cursor, Geometry, LineCap, Path, Stroke},
   text_input, scrollable, button, Color, Rectangle, Point, Vector,
};

#[derive(Debug, Default)]
pub struct DateTimeTab {
   // auto_datetime: bool,
   pub clock: Clock,
   pub txt_time_state: text_input::State,
   pub temp_time_val: String,
   pub is_time_change: bool,
   pub txt_date_state: text_input::State,
   pub temp_date_val: String,
   pub is_date_change: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TimeZoneTab {
   // auto_tz: bool,
   pub search_state: text_input::State,
   pub search_val: String,
   pub ls_continents: Vec<(String, button::State)>,
   pub ls_tz: Vec<(String, button::State)>,
   pub filtered_ls_tz: Vec<(String, button::State)>,
   pub selected_continent: Option<String>,
   pub scroll_tz: scrollable::State,
   pub scroll_continent: scrollable::State,
}

#[derive(Debug)]
pub struct Clock {
   pub now: DateTime<Local>,
   pub clock: Cache,
}

impl Default for Clock {
   fn default() -> Self {
      Self {
         now: Local::now(),
         clock: Default::default(),
      }
   }
}

impl canvas::Program<DateTimeMessage> for Clock {
   fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
      let clock = self.clock.draw(bounds.size(), |frame| {
         let center = frame.center();
         let radius = frame.width().min(frame.height()) / 2.0;

         let background = Path::circle(center, radius);
         let foreground = Path::circle(center, radius * 0.9);
         frame.fill(&background, Color {a: 0.7, ..HOVERED});
         frame.fill(&foreground, Color::WHITE);
         let circle_center = Path::circle(center, radius * 0.05);
         frame.fill(&circle_center, Color::BLACK);

         let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
         let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));
         let thin_stroke = Stroke {
            width: 1.0,
            color: ERROR,
            line_cap: LineCap::Round,
            ..Stroke::default()
         };
         let wide_stroke = Stroke {
            width: thin_stroke.width * 2.5,
            color: Color::BLACK,
            ..thin_stroke
         };
         frame.translate(Vector::new(center.x, center.y));

         frame.with_save(|frame| {
            frame.rotate(hand_rotation(self.now.hour(), 12));
            frame.stroke(&short_hand, wide_stroke);
         });

         frame.with_save(|frame| {
            frame.rotate(hand_rotation(self.now.minute(), 60));
            frame.stroke(&long_hand, wide_stroke);
         });

         frame.with_save(|frame| {
            frame.rotate(hand_rotation(self.now.second(), 60));
            frame.stroke(&long_hand, thin_stroke);
         });
      });

      vec![clock]
   }
}

fn hand_rotation(n: u32, total: u32) -> f32 {
   let turns = n as f32 / total as f32;

   2.0 * std::f32::consts::PI * turns
}