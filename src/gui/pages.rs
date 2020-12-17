#[macro_use]
mod general_page;
mod battery_page;
mod bluetooth_page;
mod date_time_page;
mod display_page;
mod keyboard_page;
mod mouse_page;
mod network_page;
mod notification_page;
mod printer_page;
mod sound_page;
mod sys_info_page;
mod touchpad_page;
mod update_page;
mod user_page;
use battery_page::{BatteryMessage, BatteryPage};
use bluetooth_page::{BluetoothMessage, BluetoothPage};
use date_time_page::{DateTimeMessage, DateTimePage};
use display_page::{DisplayMessage, DisplayPage};
use general_page::{General, GeneralMessage};
use iced::{Container, Element, Length, Space, Subscription};
use keyboard_page::{KeyboardMessage, KeyboardPage};
use mouse_page::{MouseMessage, MousePage};
use network_page::{NetMessage, NetworkPage};
use notification_page::{NotifyMsg, NotifyPage};
use printer_page::{PrinterMessage, PrinterPage};
use sound_page::{SoundMessage, SoundPage};
use sys_info_page::{InfoMessage, InfoPage};
use touchpad_page::{TouchpadMessage, TouchpadPage};
use update_page::{SoftUpdateMsg, SoftwareUpdate};
use user_page::{UserPage, UserPageMsg};
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
   NotifyMsg(NotifyMsg),
   SoftUpdateMsg(SoftUpdateMsg),
}

// #[derive(Debug)]
pub enum PageModel {
   HomePage,
   GeneralPage { general_page: General },
   DateTimePageModel { datetime_page: DateTimePage },
   LanguagePage,
   UsersPageModel { user_page: UserPage },
   AccessPage,
   AccountPage,
   NotificationsModel { noti_page: NotifyPage },
   SecurityPage,
   UpdatePageModel { update_page: SoftwareUpdate },
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
               datetime_page: DateTimePage::new(),
            },
            LanguagePage,
            UsersPageModel {
               user_page: UserPage::new(),
            },
            AccessPage,
            AccountPage,
            NotificationsModel {
               noti_page: NotifyPage::new(),
            },
            SecurityPage,
            UpdatePageModel {
               update_page: SoftwareUpdate::new(),
            },
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
         NotifyMsg(msg) => {
            if let NotificationsModel { noti_page } = self {
               noti_page.update(msg);
            }
         }
         SoftUpdateMsg(msg) => {
            if let UpdatePageModel { update_page } = self {
               update_page.update(msg);
            }
         }
      }
   }

   fn subscription(&self) -> Subscription<PagesMessage> {
      use PageModel::*;
      match self {
         DateTimePageModel { datetime_page } => datetime_page
            .subscription()
            .map(PagesMessage::DateTimeMessage),
         UpdatePageModel { update_page } => {
            update_page.subscription().map(PagesMessage::SoftUpdateMsg)
         }
         _ => Subscription::none(),
      }
   }

   fn view(&mut self) -> Element<PagesMessage> {
      use PageModel::*;
      match self {
         HomePage => Container::new(Space::with_width(Length::Shrink)).into(),
         GeneralPage { general_page } => general_page
            .view()
            .map(move |msg| PagesMessage::GeneralMessage(msg)),
         DateTimePageModel { datetime_page } => datetime_page
            .view()
            .map(move |msg| PagesMessage::DateTimeMessage(msg)),
         LanguagePage => Container::new(Space::with_width(Length::Shrink)).into(),
         UsersPageModel { user_page } => user_page
            .view()
            .map(move |msg| PagesMessage::UserPageMsg(msg)),
         AccessPage => Container::new(Space::with_width(Length::Shrink)).into(),
         AccountPage => Container::new(Space::with_width(Length::Shrink)).into(),
         NotificationsModel { noti_page } => noti_page
            .view()
            .map(move |msg| PagesMessage::NotifyMsg(msg)),
         SecurityPage => Container::new(Space::with_width(Length::Shrink)).into(),
         UpdatePageModel { update_page } => update_page
            .view()
            .map(move |msg| PagesMessage::SoftUpdateMsg(msg)),
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
         LanguagePage => "Language & Region",
         UsersPageModel { .. } => "Users & Groups",
         AccessPage => "Accessibility",
         AccountPage => "Accounts",
         NotificationsModel { .. } => "Notifications",
         SecurityPage => "Security & Privacy",
         UpdatePageModel { .. } => "Software Update",
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
