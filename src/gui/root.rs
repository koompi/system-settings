use super::pages::{Pages, PagesMessage};
use super::pref::{Category, Pref, PrefMessage};
use super::styles::{CustomButton, CustomContainer, CustomTextInput};
use crate::helpers::ROOT_PATH;
use iced::{button, executor, scrollable, text_input, window, Align, Application, Button, Column, Command, Container, Element, Length, Row, Scrollable, Settings, Space, Subscription, Text, TextInput};
use iced_custom_widget::{Grid, Icon};

pub struct SystemSetting {
   input_search: text_input::State,
   search_text: String,
   prefs: Vec<Pref>,
   selected_pref: Option<usize>,
   pages: Pages,
   back_btn_state: button::State,
   sidebar_scroll: scrollable::State,
   scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
   SearchChanged(String),
   ActionSearch,
   PrefMessage(usize, PrefMessage),
   PagesMessage(PagesMessage),
   NavigateBack,
}

impl Application for SystemSetting {
   type Executor = executor::Default;
   type Message = SystemMessage;
   type Flags = ();

   fn new(_flags: ()) -> (Self, Command<Self::Message>) {
      use Category::*;
      let pref = |file_name: &str, name: &str, category: Category| Pref::new(format!("{}/assets/images/{}.svg", ROOT_PATH(), file_name), String::from(name), category);
      let prefs = vec![
         pref("window", "General", System),
         pref("time", "Date & Time", System),
         pref("language", "Language & Region", System),
         pref("users", "Users & Groups", System),
         pref("accessibility", "Accessibility", System),
         pref("screen-saver", "Desktop & Screen Saver", System),
         pref("notification", "Notifications", System),
         pref("privacy", "Security & Privacy", System),
         pref("update", "Software Update", System),
         pref("network", "Network", Hardware),
         pref("bluetooth", "Bluetooth", Hardware),
         pref("sound", "Sound", Hardware),
         pref("printer", "Printers & Scanners", Hardware),
         pref("keyboard", "Keyboard", Hardware),
         pref("touchpad", "Touchpad", Hardware),
         pref("mouse", "Mouse", Hardware),
         pref("display", "Display", Hardware),
         pref("battery", "Battery", Hardware),
         pref("sys-info", "System Info", Hardware),
      ];

      (
         Self {
            input_search: text_input::State::new(),
            search_text: String::new(),
            prefs,
            pages: Pages::new(),
            selected_pref: None,
            back_btn_state: button::State::new(),
            sidebar_scroll: scrollable::State::new(),
            scroll: scrollable::State::new(),
         },
         Command::none(),
      )
   }

   fn title(&self) -> String {
      self.pages.title().to_string()
   }

   fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
      match message {
         Self::Message::SearchChanged(text) => self.search_text = text,
         Self::Message::ActionSearch => println!("search submited"),
         Self::Message::PrefMessage(i, pref_message) => {
            if let Some(pref) = self.prefs.get_mut(i) {
               pref.update(pref_message);
               self.selected_pref = Some(i);
               self.pages.set_current(i + 1);
            }
            // Command::perform(future, SystemMessage::Navigation())
         }
         // Self::Message::Navigation(page) => {
         //    let next_page = match page {
         //       Page::HomePage => PageModel::HomePage
         //    };
         //    self.pages_stack.push(next_page);
         // },
         Self::Message::NavigateBack => {
            // if self.pages_stack.len() > 1 {
            //    self.pages_stack.pop();
            // }
            self.selected_pref = None;
            self.pages.set_current(0)
         }
         Self::Message::PagesMessage(page_msg) => {
            self.pages.update(page_msg);
         }
      }
      Command::none()
   }

   fn subscription(&self) -> Subscription<Self::Message> {
      self.pages.subscription().map(Self::Message::PagesMessage)
   }

   fn view(&mut self) -> Element<Self::Message> {
      let search = TextInput::new(&mut self.input_search, "Search", &mut self.search_text, Self::Message::SearchChanged).padding(10).max_width(800).width(Length::Units(500)).size(17)
         .style(CustomTextInput::Default).on_submit(Self::Message::ActionSearch);
      let search_section = Container::new(search).center_x().center_y().width(Length::Fill);
      let mut search_bar = Row::new().spacing(20).padding(30);
      if self.selected_pref.is_some() {
         search_bar = search_bar.push(Button::new(&mut self.back_btn_state, Icon::new('\u{f0ce}').size(20)).on_press(SystemMessage::NavigateBack).padding(7).style(CustomButton::Text));
      }
      search_bar = search_bar.push(search_section);

      let sidebar = if let Some(selected_pref) = &self.selected_pref {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
            .fold((Column::new().spacing(10), Column::new().spacing(10)), |(personal_prefs, device_prefs), (idx, pref)| match pref.category {
               Category::System => (personal_prefs.push(pref.view_sidebar(idx == *selected_pref).map(move |message| SystemMessage::PrefMessage(idx, message))), device_prefs),
               Category::Hardware => (personal_prefs, device_prefs.push(pref.view_sidebar(idx == *selected_pref).map(move |message| SystemMessage::PrefMessage(idx, message)))),
            });
         let personal_section = Column::new().width(Length::Fill).spacing(15).align_items(Align::Center)
            .push(Container::new(Text::new("System").size(15)).padding(7).style(CustomContainer::FadedBrightForeground))
            .push(personal_prefs);
         let device_section = Column::new().width(Length::Fill).spacing(15).align_items(Align::Center)
            .push(Container::new(Text::new("Hardware").size(15)).padding(7).style(CustomContainer::FadedBrightForeground))
            .push(device_prefs);

         Container::new(
            Scrollable::new(&mut self.sidebar_scroll).padding(15).spacing(20).scroller_width(4).scrollbar_width(4)
            .push(personal_section).push(device_section)
         ).width(Length::Units(127)).style(CustomContainer::Background)
      } else {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
            .fold((Grid::new().column_width(125), Grid::new().column_width(125)), |(personal_prefs, device_prefs), (idx, pref)| match pref.category {
               Category::System => (personal_prefs.push(pref.view_main().map(move |message| SystemMessage::PrefMessage(idx, message))), device_prefs),
               Category::Hardware => (personal_prefs, device_prefs.push(pref.view_main().map(move |message| SystemMessage::PrefMessage(idx, message)))),
            });
         let personal_section = Container::new(
            Column::new().spacing(15)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(Container::new(Text::new("System").size(15)).padding(7).style(CustomContainer::FadedBrightForeground)),
            )
            .push(personal_prefs),
         ).width(Length::Fill).center_x();
         let device_section = Container::new(
            Column::new().spacing(15)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(Container::new(Text::new("Hardware").size(15)).padding(7).style(CustomContainer::FadedBrightForeground)),
            )
            .push(device_prefs),
         ).width(Length::Fill).center_x();

         Container::new(
            Scrollable::new(&mut self.scroll).spacing(30).width(Length::Fill).align_items(Align::Center).scroller_width(4).scrollbar_width(4)
            .push(personal_section).push(device_section)
         ).width(Length::Fill)
      };

      let content = self.pages.view().map(SystemMessage::PagesMessage);
      let mut main_sec = Row::new().spacing(27).push(sidebar);
      if self.selected_pref.is_some() {
         main_sec = main_sec.push(content);
      }
      Container::new(
         Column::new().spacing(15).width(Length::Fill)
         .push(search_bar)
         .push(main_sec)
      ).into()
   }
}

impl SystemSetting {
   pub fn init() -> iced::Result {
      let image = image::open(format!("{}/assets/images/icon.png", ROOT_PATH())).expect("Failed to open icon path").into_rgba8();
      let (width, height) = image.dimensions();
      let rgba = image.into_raw();

      SystemSetting::run(Settings {
         default_text_size: 13,
         default_font: Some(include_bytes!("../../assets/fonts/Inter-Bold.otf")),
         window: window::Settings {
            size: (850, 750),
            min_size: Some((750, 700)),
            icon: Some(window::Icon::from_rgba(rgba, width, height).expect("Failed to open icon")),
            ..window::Settings::default()
         },
         ..Settings::default()
      })
   }
}
