use iced::{
   Container, button, Row, Svg, Length, Text, Button, Column, Align, Element, Space,
};
use crate::helpers::ROOT_PATH;

use super::super::styles::{CustomButton, CustomContainer, ACCENT};

#[derive(Debug, Clone)]
pub enum InfoMessage {
   OpenUrl,
}

#[derive(Debug, Clone)]
pub struct InfoPage {
   os_pretty_name: String,
   os_url_state: button::State,
   os_url: String,
   origami_ui_vers: String,
   origami_framework_vers: String,
   iced_vers: String,
   kernel_vers: String,
   os_type: String,
   cpu_num: u32,
   // cpu_brand: String,
   cpu_speed: u64,
   storage_total: u64,
   storage_free: u64,
   mem_total: u64,
   mem_free: u64,
   mem_available: u64,
}

impl InfoPage {
   pub fn new() -> Self {
      let default_name = "Koompi OS 3.0.0".to_owned();
      let default_url = "https://www.koompi.com".to_owned();

      let (os_pretty_name, os_url) = match sys_info::linux_os_release() {
         Ok(linux_os_release) => {
            let os_pretty_name = linux_os_release.pretty_name.unwrap_or(default_name);
            let os_url = linux_os_release.home_url.unwrap_or(default_url);
   
            (os_pretty_name, os_url)
         },
         Err(_) => {
            (default_name, default_url)
         }
      };

      let (storage_total, storage_free) = match sys_info::disk_info() {
         Ok(disk_info) => {
            (disk_info.total, disk_info.free)
         },
         Err(_) => {
            (0, 0)
         }
      };

      let (mem_total, mem_free, mem_available) = match sys_info::mem_info() {
         Ok(mem_info) => {
            (mem_info.total, mem_info.free, mem_info.avail)
         },
         Err(_) => {
            (0, 0, 0)
         }
      };

      // let cpu_brand = match cpuid::identify() {
      //    Ok(cpu) => cpu.brand,
      //    Err(s) => s
      // };

      Self {
         os_pretty_name,
         os_url_state: button::State::new(),
         os_url,
         origami_ui_vers: "1.0.1".to_owned(),
         origami_framework_vers: "1.1.2".to_owned(),
         iced_vers: "1.2.0".to_owned(),
         kernel_vers: sys_info::os_release().unwrap_or("5.9.11".to_owned()),
         os_type: sys_info::os_type().unwrap_or("KFS".to_owned()),
         cpu_num: sys_info::cpu_num().unwrap_or(8),
         // cpu_brand,
         cpu_speed: sys_info::cpu_speed().unwrap_or(2500),
         storage_total,
         storage_free,
         mem_total,
         mem_free,
         mem_available,
      }
   }

   pub fn update(&mut self, msg: InfoMessage) {
      match msg {
         InfoMessage::OpenUrl => {
            let _ = opener::open(self.os_url.as_str());
         },
      }
   }

   pub fn view(&mut self) -> Element<InfoMessage> {
      let InfoPage {
         os_pretty_name,
         os_url_state,
         os_url,
         origami_ui_vers,
         origami_framework_vers,
         iced_vers,
         kernel_vers,
         os_type,
         cpu_num,
         // cpu_brand,
         cpu_speed,
         storage_total,
         storage_free,
         mem_total,
         mem_available,
         ..
      } = self;
      
      // ផ្នែកក្បាល
      let logo = Svg::from_path(format!("{}/assets/images/koompi-logo.svg", ROOT_PATH())).width(Length::Units(100)).height(Length::Units(100));
      let txt_os_name = Text::new(os_pretty_name.as_str()).size(20);
      let btn_os_url = Button::new(os_url_state, Text::new(os_url.as_str()).color(ACCENT)).on_press(InfoMessage::OpenUrl).style(CustomButton::Text);
      let header_sec = Container::new(
         Row::new().spacing(15).align_items(Align::Center)
         .push(Container::new(logo).width(Length::FillPortion(5)).align_x(Align::End))
         .push(
            Column::new().spacing(15).align_items(Align::Start).width(Length::FillPortion(5))
            .push(txt_os_name)
            .push(btn_os_url)
         )
      ).padding(27).center_x();

      // ផ្នែកស្លាក
      let lb_origami_ui = Text::new("Origami UI (KOUI) Version:");
      let lb_origami_frameworks = Text::new("Origami Frameworks Version:");
      let lb_iced = Text::new("Iced Version:");
      let lb_kernel = Text::new("Kernel Version:");
      let lb_os_type = Text::new("OS Type:");
      let label_soft_col = Column::new().spacing(10).align_items(Align::End)
         .push(lb_origami_ui)
         .push(lb_origami_frameworks)
         .push(lb_iced)
         .push(lb_kernel)
         .push(lb_os_type);
      let label_soft_sec = Container::new(label_soft_col).align_x(Align::End).width(Length::FillPortion(5));

      // ផ្នែកព័ត៌មាន
      let txt_origami_ui = Text::new(origami_ui_vers.as_str());
      let txt_origami_frameworks = Text::new(origami_framework_vers.as_str());
      let txt_iced = Text::new(iced_vers.as_str());
      let txt_kernel = Text::new(kernel_vers.as_str());
      let txt_os_type = Text::new(os_type.as_str());
      let info_soft_col = Column::new().spacing(10).align_items(Align::Start)
         .push(txt_origami_ui)
         .push(txt_origami_frameworks)
         .push(txt_iced)
         .push(txt_kernel)
         .push(txt_os_type);
      let info_soft_sec = Container::new(info_soft_col).align_x(Align::Start).width(Length::FillPortion(5));

      // ផ្នែកស្លាក
      let lb_cpu = Text::new("Processors:");
      let lb_mem = Text::new("Memory:");
      let lb_storage = Text::new("Storage:");
      let label_hard_col = Column::new().spacing(10).align_items(Align::End)
         .push(lb_cpu)
         .push(lb_mem)
         .push(lb_storage);
      let label_hard_sec = Container::new(label_hard_col).align_x(Align::End).width(Length::FillPortion(5));

      // ផ្នែកព័ត៌មាន
      let txt_cpu = Text::new(format!("{} X {} {} Mhz", cpu_num, "Intel Core i9", cpu_speed));
      let txt_mem = Text::new(format!("{:.1} GB of {:.1} GB", f32::from(*mem_available as f32/1024000.0), f32::from(*mem_total as f32/1024000.0)));
      let txt_storage = Text::new(format!("{:.1} GB available of {:.1} GB", f32::from(*storage_free as f32/1024000.0), f32::from(*storage_total as f32/1024000.0)));
      let info_hard_col = Column::new().spacing(10).align_items(Align::Start)
         .push(txt_cpu)
         .push(txt_mem)
         .push(txt_storage);
      let info_hard_sec = Container::new(info_hard_col).align_x(Align::Start).width(Length::FillPortion(5));

      // មាតិកា   
      let content = Column::new().spacing(15).width(Length::Fill).align_items(Align::Center)
         .push(header_sec)
         .push(
            Row::new().spacing(10).push(Space::with_width(Length::FillPortion(5))).push(Text::new("Software").size(15).width(Length::FillPortion(5)))
         )
         .push(
            Row::new().spacing(10)
            .push(label_soft_sec)
            .push(info_soft_sec)
         )
         .push(
            Row::new().spacing(10).push(Space::with_width(Length::FillPortion(5))).push(Text::new("Hardware").size(15).width(Length::FillPortion(5)))
         )
         .push(
            Row::new().spacing(10)
            .push(label_hard_sec)
            .push(info_hard_sec)
         );

      Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
   }
}