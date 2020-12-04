#[macro_use]
mod sound_page;
mod bluetooth_page;
mod general_page;
mod printer_page;
mod keyboard_page;

use bluetooth_page::{BluetoothMessage, BluetoothPage};
use general_page::{General, GeneralMessage};
use sound_page::{SoundMessage, SoundPage};
use printer_page::{PrinterPage, PrinterMessage};
use keyboard_page::{KeyboardPage, KeyboardMessage};
use iced::{Container, Element, Length, Space};

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
}

#[derive(Debug, Clone)]
pub enum PageModel {
    HomePage,
    GeneralPage { general_page: General },
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
    PrinterPageModel {
        printer_page: PrinterPage
    },
    KeyboardPageModel {
        keyboard_page: KeyboardPage
    },
    TouchpadPage,
    MousePage,
    DisplayPage,
    BatteryPage,
    DiskDrivePage,
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
                PrinterPageModel {
                printer_page: PrinterPage::new()
                },
                KeyboardPageModel {
                keyboard_page: KeyboardPage::new()
                },
                TouchpadPage,
                MousePage,
                DisplayPage,
                BatteryPage,
                DiskDrivePage,
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
            },
            KeyboardMessage(msg) => {
                if let KeyboardPageModel { keyboard_page } = self {
                    keyboard_page.update(msg);
                }
            },
        }
    }

    fn view(&mut self) -> Element<PagesMessage> {
        use PageModel::*;
        match self {
            HomePage => Container::new(Space::with_width(Length::Shrink)).into(),
            GeneralPage { general_page } => general_page
                .view()
                .map(move |msg| PagesMessage::GeneralMessage(msg)),
            DateTimePage => Container::new(Space::with_width(Length::Shrink)).into(),
            LanguagePage => Container::new(Space::with_width(Length::Shrink)).into(),
            UsersPage => Container::new(Space::with_width(Length::Shrink)).into(),
            AccessPage => Container::new(Space::with_width(Length::Shrink)).into(),
            AccountPage => Container::new(Space::with_width(Length::Shrink)).into(),
            NotiPage => Container::new(Space::with_width(Length::Shrink)).into(),
            SecurityPage => Container::new(Space::with_width(Length::Shrink)).into(),
            UpdatePage => Container::new(Space::with_width(Length::Shrink)).into(),
            NetworkPage => Container::new(Space::with_width(Length::Shrink)).into(),
            BluetoothPageModel { bluetooth_page } => bluetooth_page
                .view()
                .map(move |msg| PagesMessage::BluetoothMessage(msg)),
            SoundPageModel { sound_page } => sound_page
                .view()
                .map(move |msg| PagesMessage::SoundMessage(msg)),
            PrinterPageModel { printer_page } => printer_page.view().map(move |msg| PagesMessage::PrinterMessage(msg)),
            KeyboardPageModel { keyboard_page } => keyboard_page.view().map(move |msg| PagesMessage::KeyboardMessage(msg)),
            TouchpadPage => Container::new(Space::with_width(Length::Shrink)).into(),
            MousePage => Container::new(Space::with_width(Length::Shrink)).into(),
            DisplayPage => Container::new(Space::with_width(Length::Shrink)).into(),
            BatteryPage => Container::new(Space::with_width(Length::Shrink)).into(),
            DiskDrivePage => Container::new(Space::with_width(Length::Shrink)).into(),
        }
    }

    fn title(&self) -> &str {
        use PageModel::*;
        match self {
            HomePage => "System Setting",
            GeneralPage { .. } => "General",
            DateTimePage => "Date & Time",
            LanguagePage => "Language & Region",
            UsersPage => "Users & Groups",
            AccessPage => "Accessibility",
            AccountPage => "Accounts",
            NotiPage => "Notifications",
            SecurityPage => "Security & Privacy",
            UpdatePage => "Software Update",
            NetworkPage => "Network",
            BluetoothPageModel { .. } => "Bluetooth",
            SoundPageModel { .. } => "Sound",
            PrinterPageModel { .. } => "Printers & Scanners",
            KeyboardPageModel { .. } => "Keyboard",
            TouchpadPage => "Touchpad",
            MousePage => "Mouse",
            DisplayPage => "Display",
            BatteryPage => "Battery",
            DiskDrivePage => "Disk Drive",
        }
    }
}
