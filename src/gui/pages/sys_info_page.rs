use crate::gui::styles::{CustomButton, CustomContainer, ACCENT};
use crate::helpers::ROOT_PATH;
use iced::{button, Align, Button, Column, Container, Element, Length, Row, Space, Svg, Text};
use sysinfo::{DiskExt, System, SystemExt};

const BYTES_PER_KB: f32 = 1024.0;
const BYTES_PER_GB: f32 = BYTES_PER_KB * 1_000_000.0;

#[derive(Debug, Clone)]
pub enum InfoMessage {
    OpenUrl,
}

#[derive(Debug, Clone, Default)]
pub struct InfoPage {
    os_name: String,
    os_url_state: button::State,
    os_url: String,
    libkoompi_vers: String,
    iced_vers: String,
    kernel_vers: String,
    os_type: String,
    cpu_num: u32,
    cpu_speed: f32,
    storage_total: f32,
    storage_available: f32,
    mem_total: u64,
    mem_used: u64,
    swap_total: u64,
    swap_used: u64,
}

impl InfoPage {
    pub fn new() -> Self {
        let system = System::new_all();
        let (storage_total_in_bytes, storage_available_in_bytes) = system.get_disks().iter().fold((0, 0), |(st_total, st_avai), disk| (st_total + disk.get_total_space(), st_avai + disk.get_available_space()));

        Self {
            os_name: system.get_name().unwrap_or_default(),
            os_url_state: button::State::new(),
            os_url: "https://www.koompi.com".to_owned(),
            libkoompi_vers: "1.0.1".to_owned(),
            iced_vers: "1.2.0".to_owned(),
            kernel_vers: sys_info::os_release().unwrap_or_default(),
            os_type: sys_info::os_type().unwrap_or_default(),
            cpu_num: sys_info::cpu_num().unwrap_or_default(),
            cpu_speed: sys_info::cpu_speed().unwrap_or_default() as f32 / 1000.0,
            storage_total: storage_total_in_bytes as f32 / BYTES_PER_GB,
            storage_available: storage_available_in_bytes as f32 / BYTES_PER_GB,
            mem_total: system.get_total_memory() / 1000,
            mem_used: system.get_used_memory() / 1000,
            swap_total: system.get_total_swap() / 1000,
            swap_used: system.get_used_swap() / 1000,
        }
    }

    pub fn update(&mut self, msg: InfoMessage) {
        match msg {
            InfoMessage::OpenUrl => {
                let _ = opener::open(self.os_url.as_str());
            }
        }
    }

    pub fn view(&mut self) -> Element<InfoMessage> {
        let InfoPage {
            os_name,
            os_url_state,
            os_url,
            os_type,
            libkoompi_vers,
            iced_vers,
            kernel_vers,
            cpu_num,
            cpu_speed,
            storage_total,
            storage_available,
            mem_total,
            mem_used,
            swap_total,
            swap_used,
        } = self;

        // ផ្នែកក្បាល
        let logo = Svg::from_path(format!("{}/assets/images/koompi-logo.svg", ROOT_PATH())).width(Length::Units(100)).height(Length::Units(100));
        let txt_os_name = Text::new(os_name.as_str()).size(20);
        let btn_os_url = Button::new(os_url_state, Text::new(os_url.as_str()).color(ACCENT)).on_press(InfoMessage::OpenUrl).style(CustomButton::Text);
        let header_sec = Container::new(
            Row::new()
                .spacing(15)
                .align_items(Align::Center)
                .push(Container::new(logo).width(Length::FillPortion(5)).align_x(Align::End))
                .push(Column::new().spacing(15).align_items(Align::Start).width(Length::FillPortion(5)).push(txt_os_name).push(btn_os_url)),
        )
        .padding(27)
        .center_x();

        // ផ្នែកស្លាក
        let lb_libkoompi = Text::new("LibKoompi Version:");
        let lb_iced = Text::new("Iced Version:");
        let lb_kernel = Text::new("Kernel Version:");
        let lb_os_type = Text::new("OS Type:");
        let label_soft_col = Column::new().spacing(10).align_items(Align::End).push(lb_libkoompi).push(lb_iced).push(lb_kernel).push(lb_os_type);
        let label_soft_sec = Container::new(label_soft_col).align_x(Align::End).width(Length::FillPortion(5));

        // ផ្នែកព័ត៌មាន
        let txt_libkoompi = Text::new(libkoompi_vers.as_str());
        let txt_iced = Text::new(iced_vers.as_str());
        let txt_kernel = Text::new(kernel_vers.as_str());
        let txt_os_type = Text::new(os_type.as_str());
        let info_soft_col = Column::new().spacing(10).align_items(Align::Start).push(txt_libkoompi).push(txt_iced).push(txt_kernel).push(txt_os_type);
        let info_soft_sec = Container::new(info_soft_col).align_x(Align::Start).width(Length::FillPortion(5));

        // ផ្នែកស្លាក
        let lb_cpu = Text::new("Processors:");
        let lb_mem = Text::new("Memory:");
        let lb_swap = Text::new("Swap:");
        let lb_storage = Text::new("Storage:");
        let label_hard_col = Column::new().spacing(10).align_items(Align::End).push(lb_cpu).push(lb_mem).push(lb_swap).push(lb_storage);
        let label_hard_sec = Container::new(label_hard_col).align_x(Align::End).width(Length::FillPortion(5));

        // ផ្នែកព័ត៌មាន
        let txt_cpu = Text::new(format!("{} X {:.1} Ghz", cpu_num, cpu_speed));
        let txt_mem = Text::new(format!("Used {:.1} MB of {:.1} MB", *mem_used, *mem_total));
        let txt_swap = Text::new(format!("Used {:.1} MB of {:.1} MB", *swap_used, *swap_total));
        let txt_storage = Text::new(format!("{:.1} GB available of {:.1} GB", *storage_available, *storage_total));
        let info_hard_col = Column::new().spacing(10).align_items(Align::Start).push(txt_cpu).push(txt_mem).push(txt_swap).push(txt_storage);
        let info_hard_sec = Container::new(info_hard_col).align_x(Align::Start).width(Length::FillPortion(5));

        // មាតិកា
        let content = Column::new()
            .spacing(15)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(header_sec)
            .push(Row::new().spacing(10).push(Space::with_width(Length::FillPortion(5))).push(Text::new("Software").size(15).width(Length::FillPortion(5))))
            .push(Row::new().spacing(10).push(label_soft_sec).push(info_soft_sec))
            .push(Row::new().spacing(10).push(Space::with_width(Length::FillPortion(5))).push(Text::new("Hardware").size(15).width(Length::FillPortion(5))))
            .push(Row::new().spacing(10).push(label_hard_sec).push(info_hard_sec));

        Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
    }
}
