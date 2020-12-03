#[macro_use]
mod sound_page;
mod general_page;
mod bluetooth_page;

use bluetooth_page::{BluetoothPage, BluetoothMessage};
use sound_page::{SoundPage, SoundMessage};
use iced::{
   Element, Container, Length, Space
}; 

pub struct Pages {
   pages: Vec<PageModel>,
   current: usize
}

#[derive(Debug, Clone)]
pub enum PagesMessage {
   CheckboxToggle(bool),
   BluetoothMessage(BluetoothMessage),
   SoundMessage(SoundMessage)
}

#[derive(Debug, Clone)]
pub enum PageModel {
   HomePage,
   GeneralPage {
      checkbox: bool
   },
   DateTimePage,
   LanguagePage,
   UsersPage,
   AccessPage,
   AccountPage,
   NotiPage,
   SecurityPage,
   UpdatePage,
   NetworkPage,
   BluetoothPageModel {
      bluetooth_page: BluetoothPage
   },
   SoundPageModel {
      sound_page: SoundPage
   },
   PrinterPage,
   CameraPage,
   KeyboardPage,
   TouchpadPage,
   MousePage,
   DisplayPage,
   BatteryPage,
   DiskDrivePage
}

impl Pages {
   pub fn new() -> Self {
      use PageModel::*;
      Self {
         pages: vec![
            HomePage,
            GeneralPage {
               checkbox: false
            },
            DateTimePage,
            LanguagePage,
            UsersPage,
            AccessPage,
            AccountPage,
            NotiPage,
            SecurityPage,
            UpdatePage,
            NetworkPage,
            BluetoothPageModel {
               bluetooth_page: BluetoothPage::new()
            },
            SoundPageModel {
               sound_page: SoundPage::new()
            },
            PrinterPage,
            CameraPage,
            KeyboardPage,
            TouchpadPage,
            MousePage,
            DisplayPage,
            BatteryPage,
            DiskDrivePage
         ],
         current: 0,
      }
   }

   pub fn set_current(&mut self, idx: usize) {
      self.current = idx;
   }

   pub fn update(&mut self, msg: PagesMessage) {
      self.pages[self.current].update(msg);
   }

   pub fn view(&mut self) -> Element<PagesMessage> {
      self.pages[self.current].view()
   }

   pub fn title(&self) -> &str {
      self.pages[self.current].title()
   }
}

impl PageModel {
   fn update(&mut self, msg: PagesMessage) {
      use PagesMessage::*;
      use PageModel::*;
      match msg {
         CheckboxToggle(b) => {
            if let GeneralPage{checkbox} = self {
               *checkbox = b;
            }
         }
         BluetoothMessage(msg) => {
            if let BluetoothPageModel{ bluetooth_page } = self {
               bluetooth_page.update(msg);
            }
         }
         SoundMessage(msg) => {
            if let SoundPageModel{ sound_page } = self {
               sound_page.update(msg);
            }
         }
      }
   }

   fn view(&mut self) -> Element<PagesMessage> {
      use PageModel::*;
      match self {
         HomePage => Container::new(Space::with_width(Length::Shrink)).into(),
         GeneralPage { .. } => Container::new(Space::with_width(Length::Shrink)).into(),
         DateTimePage => Container::new(Space::with_width(Length::Shrink)).into(),
         LanguagePage => Container::new(Space::with_width(Length::Shrink)).into(),
         UsersPage => Container::new(Space::with_width(Length::Shrink)).into(),
         AccessPage => Container::new(Space::with_width(Length::Shrink)).into(),
         AccountPage => Container::new(Space::with_width(Length::Shrink)).into(),
         NotiPage => Container::new(Space::with_width(Length::Shrink)).into(),
         SecurityPage => Container::new(Space::with_width(Length::Shrink)).into(),
         UpdatePage => Container::new(Space::with_width(Length::Shrink)).into(),
         NetworkPage => Container::new(Space::with_width(Length::Shrink)).into(),
         BluetoothPageModel { bluetooth_page } => bluetooth_page.view().map(move |msg| PagesMessage::BluetoothMessage(msg)),
         SoundPageModel{ sound_page } => sound_page.view().map(move |msg| PagesMessage::SoundMessage(msg)),
         PrinterPage => Container::new(Space::with_width(Length::Shrink)).into(),
         CameraPage => Container::new(Space::with_width(Length::Shrink)).into(),
         KeyboardPage => Container::new(Space::with_width(Length::Shrink)).into(),
         TouchpadPage=> Container::new(Space::with_width(Length::Shrink)).into(),
         MousePage=> Container::new(Space::with_width(Length::Shrink)).into(),
         DisplayPage=> Container::new(Space::with_width(Length::Shrink)).into(),
         BatteryPage=> Container::new(Space::with_width(Length::Shrink)).into(),
         DiskDrivePage => Container::new(Space::with_width(Length::Shrink)).into()
      }
   }

   fn title(&self) -> &str {
      use PageModel::*;
      match self {
         HomePage => "System Setting",
         GeneralPage{..} => "General",
         DateTimePage => "Date & Time",
         LanguagePage => "Language & Region",
         UsersPage => "Users & Groups",
         AccessPage => "Accessibility",
         AccountPage => "Accounts",
         NotiPage => "Notifications",
         SecurityPage => "Security & Privacy",
         UpdatePage => "Software Update",
         NetworkPage => "Network",
         BluetoothPageModel {..} => "Bluetooth",
         SoundPageModel {..} => "Sound",
         PrinterPage => "Printers & Scanners",
         CameraPage => "Camera",
         KeyboardPage => "Keyboard",
         TouchpadPage=> "Touchpad",
         MousePage=> "Mouse",
         DisplayPage=> "Display",
         BatteryPage=> "Battery",
         DiskDrivePage => "Disk Drive"
      }
   }
}