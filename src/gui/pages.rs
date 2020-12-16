#[macro_use]
mod general_page;
mod battery_page;
mod bluetooth_page;
mod display_page;
mod keyboard_page;
mod mouse_page;
mod network_page;
mod printer_page;
mod sound_page;
mod sys_info_page;
mod touchpad_page;
mod user_page;
mod date_time_page;
mod lang_region_page;
mod access_page;

use battery_page::{BatteryMessage, BatteryPage};
use bluetooth_page::{BluetoothMessage, BluetoothPage};
use display_page::{DisplayMessage, DisplayPage};
use general_page::{General, GeneralMessage};
use keyboard_page::{KeyboardMessage, KeyboardPage};
use mouse_page::{MouseMessage, MousePage};
use network_page::{NetMessage, NetworkPage};
use printer_page::{PrinterMessage, PrinterPage};
use sound_page::{SoundMessage, SoundPage};
use touchpad_page::{TouchpadPage, TouchpadMessage};
use sys_info_page::{InfoPage, InfoMessage};
use user_page::{UserPage, UserPageMsg};
use date_time_page::{DateTimePage, DateTimeMessage};
use lang_region_page::{LangRegionPage, LangRegionMessage};
use access_page::{AccessPage, AccessMessage};
use iced::{Container, Element, Length, Space, Subscription};

pub struct Pages {
   pages: Vec<PageModel>,
   current: usize,
}

#[derive(Debug, Clone)]
pub enum PagesMessage {
   BluetoothMessage(BluetoothMessage),
   SoundMessage(SoundMessage),
   GeneralMessage(GeneralMessage),
   PrinterMessage(PrinterMessage),
   KeyboardMessage(KeyboardMessage),
   TouchpadMessage(TouchpadMessage),
   NetMessage(NetMessage),
   MouseMessage(MouseMessage),
   DisplayMessage(DisplayMessage),
   BatteryMessage(BatteryMessage),
   InfoMessage(InfoMessage),
   UserPageMsg(UserPageMsg),
   DateTimeMessage(DateTimeMessage),
   LangRegionMessage(LangRegionMessage),
   AccessMessage(AccessMessage),
}

// #[derive(Debug)]
pub enum PageModel {
   HomePage,
   GeneralPage { general_page: General },
   DateTimePageModel { datetime_page: DateTimePage },
   LanguagePageModel { lang_region_page: LangRegionPage },
   UsersPageModel { user_page: UserPage },
   AccessPageModel { access_page: AccessPage },
   AccountPage,
   NotiPage,
   SecurityPage,
   UpdatePage,
   NetworkPageModel { network_page: NetworkPage },
   BluetoothPageModel { bluetooth_page: BluetoothPage },
   SoundPageModel { sound_page: SoundPage },
   PrinterPageModel { printer_page: PrinterPage },
   KeyboardPageModel { keyboard_page: KeyboardPage },
   TouchpadPageModel { touchpad_page: TouchpadPage },
   MousePageModel { mouse_page: MousePage },
   DisplayPageModel { display_page: DisplayPage },
   BatteryPageModel { battery_page: BatteryPage },
   InfoPageModel { info_page: InfoPage },
}

impl Pages {
   pub fn new() -> Self {
      use PageModel::*;
      Self {
         pages: vec![
            HomePage,
            GeneralPage {
               general_page: General::new(),
            },
            DateTimePageModel {
               datetime_page: DateTimePage::new()
            },
            LanguagePageModel {
               lang_region_page: LangRegionPage::new()
            },
            UsersPageModel {
               user_page: UserPage::new(),
            },
            AccessPageModel {
               access_page: AccessPage::new()
            },
            AccountPage,
            NotiPage,
            SecurityPage,
            UpdatePage,
            NetworkPageModel {
               network_page: NetworkPage::new(),
            },
            BluetoothPageModel {
               bluetooth_page: BluetoothPage::new(),
            },
            SoundPageModel {
               sound_page: SoundPage::new(),
            },
            PrinterPageModel {
               printer_page: PrinterPage::new(),
            },
            KeyboardPageModel {
               keyboard_page: KeyboardPage::new(),
            },
            TouchpadPageModel {
               touchpad_page: TouchpadPage::new(),
            },
            MousePageModel {
               mouse_page: MousePage::new(),
            },
            DisplayPageModel {
               display_page: DisplayPage::new(),
            },
            BatteryPageModel {
               battery_page: BatteryPage::new(),
            },
            InfoPageModel {
               info_page: InfoPage::new(),
            },
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

   pub fn subscription(&self) -> Subscription<PagesMessage> {
      self.pages[self.current].subscription()
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
      use PageModel::*;
      use PagesMessage::*;
      match msg {
         BluetoothMessage(msg) => {
            if let BluetoothPageModel { bluetooth_page } = self {
               bluetooth_page.update(msg);
            }
         }
         SoundMessage(msg) => {
            if let SoundPageModel { sound_page } = self {
               sound_page.update(msg);
            }
         }
         GeneralMessage(msg) => {
            if let GeneralPage { general_page } = self {
               general_page.update(msg);
            }
         }
         PrinterMessage(msg) => {
            if let PrinterPageModel { printer_page } = self {
               printer_page.update(msg);
            }
         }
         KeyboardMessage(msg) => {
            if let KeyboardPageModel { keyboard_page } = self {
               keyboard_page.update(msg);
            }
         }
         TouchpadMessage(msg) => {
            if let TouchpadPageModel { touchpad_page } = self {
               touchpad_page.update(msg);
            }
         }
         NetMessage(msg) => {
            if let NetworkPageModel { network_page } = self {
               network_page.update(msg);
            }
         }
         MouseMessage(msg) => {
            if let MousePageModel { mouse_page } = self {
               mouse_page.update(msg);
            }
         }
         DisplayMessage(msg) => {
            if let DisplayPageModel { display_page } = self {
               display_page.update(msg);
            }
         }
         BatteryMessage(msg) => {
            if let BatteryPageModel { battery_page } = self {
               battery_page.update(msg);
            }
         }
         InfoMessage(msg) => {
            if let InfoPageModel { info_page } = self {
               info_page.update(msg);
            }
         }
         UserPageMsg(msg) => {
            if let UsersPageModel { user_page } = self {
               user_page.update(msg);
            }
         }
         DateTimeMessage(msg) => {
            if let DateTimePageModel { datetime_page } = self {
               datetime_page.update(msg);
            }
         }
         LangRegionMessage(msg) => {
            if let LanguagePageModel { lang_region_page } = self {
               lang_region_page.update(msg);
            }
         }
         AccessMessage(msg) => {
            if let AccessPageModel { access_page } = self {
               access_page.update(msg);
            }
         }
      }
   }

   fn subscription(&self) -> Subscription<PagesMessage> {
      use PageModel::*;
      match self {
         DateTimePageModel { datetime_page } => datetime_page.subscription().map(PagesMessage::DateTimeMessage),
         LanguagePageModel { lang_region_page } => lang_region_page.subscription().map(PagesMessage::LangRegionMessage),
         _ => Subscription::none()
      }
   }

   fn view(&mut self) -> Element<PagesMessage> {
      use PageModel::*;
      match self {
         HomePage => Container::new(Space::with_width(Length::Shrink)).into(),
         GeneralPage { general_page } => general_page
            .view()
            .map(move |msg| PagesMessage::GeneralMessage(msg)),
         DateTimePageModel { datetime_page } => datetime_page.view().map(move |msg| PagesMessage::DateTimeMessage(msg)),
         LanguagePageModel { lang_region_page } => lang_region_page.view().map(move |msg| PagesMessage::LangRegionMessage(msg)),
         UsersPageModel { user_page } => user_page
            .view()
            .map(move |msg| PagesMessage::UserPageMsg(msg)),
         AccessPageModel { access_page } => access_page.view().map(move |msg| PagesMessage::AccessMessage(msg)),
         AccountPage => Container::new(Space::with_width(Length::Shrink)).into(),
         NotiPage => Container::new(Space::with_width(Length::Shrink)).into(),
         SecurityPage => Container::new(Space::with_width(Length::Shrink)).into(),
         UpdatePage => Container::new(Space::with_width(Length::Shrink)).into(),
         NetworkPageModel { network_page } => network_page
            .view()
            .map(move |msg| PagesMessage::NetMessage(msg)),
         BluetoothPageModel { bluetooth_page } => bluetooth_page
            .view()
            .map(move |msg| PagesMessage::BluetoothMessage(msg)),
         SoundPageModel { sound_page } => sound_page
            .view()
            .map(move |msg| PagesMessage::SoundMessage(msg)),
         PrinterPageModel { printer_page } => printer_page
            .view()
            .map(move |msg| PagesMessage::PrinterMessage(msg)),
         KeyboardPageModel { keyboard_page } => keyboard_page
            .view()
            .map(move |msg| PagesMessage::KeyboardMessage(msg)),
         TouchpadPageModel { touchpad_page } => touchpad_page
            .view()
            .map(move |msg| PagesMessage::TouchpadMessage(msg)),
         MousePageModel { mouse_page } => mouse_page
            .view()
            .map(move |msg| PagesMessage::MouseMessage(msg)),
         DisplayPageModel { display_page } => display_page
            .view()
            .map(move |msg| PagesMessage::DisplayMessage(msg)),
         BatteryPageModel { battery_page } => battery_page
            .view()
            .map(move |msg| PagesMessage::BatteryMessage(msg)),
         InfoPageModel { info_page } => info_page
            .view()
            .map(move |msg| PagesMessage::InfoMessage(msg)),
      }
   }

   fn title(&self) -> &str {
      use PageModel::*;
      match self {
         HomePage => "System Setting",
         GeneralPage { .. } => "General",
         DateTimePageModel { .. } => "Date & Time",
         LanguagePageModel { .. } => "Language & Region",
         UsersPageModel { .. } => "Users & Groups",
         AccessPageModel { .. } => "Accessibility",
         AccountPage => "Accounts",
         NotiPage => "Notifications",
         SecurityPage => "Security & Privacy",
         UpdatePage => "Software Update",
         NetworkPageModel { .. } => "Network",
         BluetoothPageModel { .. } => "Bluetooth",
         SoundPageModel { .. } => "Sound",
         PrinterPageModel { .. } => "Printers & Scanners",
         KeyboardPageModel { .. } => "Keyboard",
         TouchpadPageModel { .. } => "Touchpad",
         MousePageModel { .. } => "Mouse",
         DisplayPageModel { .. } => "Display",
         BatteryPageModel { .. } => "Battery",
         InfoPageModel { .. } => "System Information",
      }
   }
}
