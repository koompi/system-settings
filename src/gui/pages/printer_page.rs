use iced::{
   pick_list, Container, PickList, button, Row, Length, Text, Button, Column, Align, Space, Element
};
use iced_custom_widget::Icon;
use super::super::styles::{CustomButton, CustomContainer};
use smart_default::SmartDefault;

#[derive(Debug, Clone)]
pub enum PrinterMessage {
   BtnAddClicked,
   BtnRemoveClicked,
   DeviceSelected(usize),
   PrinterChanged(Printer),
   PaperSizeChanged(PaperSize)
}

#[derive(Debug, Clone, Default)]
pub struct PrinterPage {
   btn_add_state: button::State, 
   btn_remove_state: button::State, 
   ls_devices: Vec<(String, button::State)>,
   selected_device: Option<usize>,
   pl_printer_state: pick_list::State<Printer>,
   default_printer: Printer,
   pl_paper_size_state: pick_list::State<PaperSize>,
   default_paper_size: PaperSize,
}

impl PrinterPage {
   pub fn new() -> Self {
      Self::default()
   }

   pub fn update(&mut self, msg: PrinterMessage) {
      match msg {
         PrinterMessage::BtnAddClicked => self.ls_devices.push(("New Device".to_string(), button::State::new())),
         PrinterMessage::BtnRemoveClicked => {
            if let Some(idx) = self.selected_device {
               self.ls_devices.remove(idx);
            }
            self.selected_device = None;
         },
         PrinterMessage::DeviceSelected(idx) => self.selected_device = Some(idx),
         PrinterMessage::PrinterChanged(printer) => self.default_printer = printer,
         PrinterMessage::PaperSizeChanged(size) => self.default_paper_size = size,
      }
   }

   pub fn view(&mut self) -> Element<PrinterMessage> {
      let Self {
         btn_add_state,
         btn_remove_state, 
         ls_devices,
         selected_device,
         pl_printer_state,
         default_printer,
         pl_paper_size_state,
         default_paper_size,
      } = self;
      // ផ្ទាំងខាងឆ្វេង
      let btn_add = Button::new(btn_add_state, Icon::new('\u{f0fe}').size(27)).padding(0).on_press(PrinterMessage::BtnAddClicked).style(CustomButton::Text);
      let mut btn_remove = Button::new(btn_remove_state, Icon::new('\u{f146}').size(27)).padding(0).style(CustomButton::Text);
      if selected_device.is_some() {
         btn_remove = btn_remove.on_press(PrinterMessage::BtnRemoveClicked);
      }
      let btn_group = Container::new(
         Row::new().push(btn_add).push(btn_remove)
      ).width(Length::Fill).style(CustomContainer::Header);

      let device_group = ls_devices.iter_mut().enumerate().fold(Column::new().height(Length::Fill).padding(7).spacing(4), |col, (idx, (title, state))| {
         col.push(
            if let Some(selected_idx) = selected_device {
               Button::new(state, Text::new(title.as_str())).width(Length::Fill).on_press(PrinterMessage::DeviceSelected(idx)).style(if *selected_idx == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
            } else {
               Button::new(state, Text::new(title.as_str())).width(Length::Fill).on_press(PrinterMessage::DeviceSelected(idx)).style(CustomButton::Sidebar)
            }
         )
      });
      
      let left_pane = Column::new().width(Length::Fill).height(Length::Fill).spacing(10)
         .push(device_group)
         .push(btn_group);

      // ផ្ទាំងខាងស្ដាំ
      let right_pane = Container::new(
            Column::new().spacing(10).align_items(Align::Center)
            .push(Text::new("No printers are available."))
            .push(Text::new("Click Add (+) to set up a printer."))
         ).height(Length::Fill).width(Length::FillPortion(12)).center_x().center_y().style(CustomContainer::ForegroundGray);

      // ផ្នែកខាងក្រោម
      let lb_printer = Text::new("Default printer:");
      let pl_printer = PickList::new(pl_printer_state, &Printer::ALL[..], Some(*default_printer), PrinterMessage::PrinterChanged);
      let printer_row = Row::new().spacing(10).align_items(Align::Center).push(lb_printer).push(pl_printer);

      let lb_paper_size = Text::new("Default paper size:");
      let pl_paper_size = PickList::new(pl_paper_size_state, &PaperSize::ALL[..], Some(*default_paper_size), PrinterMessage::PaperSizeChanged);
      let paper_size_row = Row::new().spacing(10).align_items(Align::Center).push(lb_paper_size).push(pl_paper_size).push(Space::with_width(Length::Units(167)));

      let bottom_col = Column::new().spacing(10).width(Length::Fill).align_items(Align::Center)
         .push(printer_row)
         .push(paper_size_row);

      // មាតិកា   
      let content = Column::new().spacing(15)
         .push(
            Row::new().height(Length::Fill).spacing(15)
            .push(Container::new(left_pane).width(Length::FillPortion(4)).style(CustomContainer::ForegroundWhite))
            .push(right_pane)
         )
         .push(bottom_col);

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, SmartDefault)]
pub enum Printer {
   #[default]
   RecentUsed,
   BrotherMFCL3770CDW
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, SmartDefault)]
pub enum PaperSize {
   USLetter,
   USLegal,
   #[default]
   A4,
   A5,
   JisB5,
   B5,
   Tabloid,
   A3
}

impl Printer {
   const ALL: [Printer; 2] = [
      Printer::RecentUsed,
      Printer::BrotherMFCL3770CDW,
   ];
}

impl std::fmt::Display for Printer {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            Printer::RecentUsed => "Recently Used Printer",
            Printer::BrotherMFCL3770CDW => "Brother MFC-L3770 CDW Laser Printer",
         }
      )
   }
}

impl PaperSize {
   const ALL: [PaperSize; 8] = [
      PaperSize::USLetter,
      PaperSize::USLegal,
      PaperSize::A4,
      PaperSize::A5,
      PaperSize::JisB5,
      PaperSize::B5,
      PaperSize::Tabloid,
      PaperSize::A3
   ];
}

impl std::fmt::Display for PaperSize {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(
         f,
         "{}",
         match self {
            PaperSize::USLetter => "US Letter",
            PaperSize::USLegal => "US Legal",
            PaperSize::A4 => "A4",
            PaperSize::A5 => "A5",
            PaperSize::JisB5 => "JIS B5",
            PaperSize::B5 => "B5",
            PaperSize::Tabloid => "Tabloid",
            PaperSize::A3 => "A3"
         }
      )
   }
}