use super::super::styles::{CustomButton, CustomCheckbox, CustomContainer, CustomRadio, CustomSlider};
use crate::helpers::ROOT_PATH;
use iced::{button, slider, Align, Button, Checkbox, Column, Container, Element, Length, Radio, Row, Slider, Space, Svg, Text};
use smart_default::SmartDefault;
#[derive(Debug, Clone)]
pub enum MouseMessage {
    PrimaryButtonChanged(PrimaryButton),
    PointerSpeedChanged(u8),
    DoubleClickSpeedChanged(u8),
    ReverseScrollingToggled(bool),
    RestoreDefaultClicked,
}

#[derive(Debug, Clone)]
pub struct MousePage {
    primary_button: PrimaryButton,
    pointer_speed_state: slider::State,
    pointer_speed: u8,
    double_click_speed_state: slider::State,
    double_click_speed: u8,
    reverse_scrolling: bool,
    btn_default_state: button::State,
}

impl MousePage {
    pub fn new() -> Self {
        Self {
            primary_button: PrimaryButton::default(),
            pointer_speed_state: slider::State::new(),
            pointer_speed: 70,
            double_click_speed_state: slider::State::new(),
            double_click_speed: 80,
            reverse_scrolling: false,
            btn_default_state: button::State::new(),
        }
    }

    pub fn update(&mut self, msg: MouseMessage) {
        match msg {
            MouseMessage::PrimaryButtonChanged(val) => self.primary_button = val,
            MouseMessage::PointerSpeedChanged(val) => self.pointer_speed = val,
            MouseMessage::DoubleClickSpeedChanged(val) => self.double_click_speed = val,
            MouseMessage::ReverseScrollingToggled(is_checked) => self.reverse_scrolling = is_checked,
            MouseMessage::RestoreDefaultClicked => *self = Self::new(),
        }
    }

    pub fn view(&mut self) -> Element<MouseMessage> {
        let MousePage {
            primary_button,
            pointer_speed_state,
            pointer_speed,
            double_click_speed_state,
            double_click_speed,
            reverse_scrolling,
            btn_default_state,
        } = self;
        // ផ្ទាំងខាងឆ្វេង
        let logo = Svg::from_path(format!("{}/assets/images/mouse.svg", ROOT_PATH())).width(Length::Units(150)).height(Length::Units(150));
        let left_pane = Container::new(logo).width(Length::FillPortion(4)).padding(15).center_x();

        // ផ្ទាំងខាងស្ដាំ
        let rd_primary_button = PrimaryButton::ALL.iter().fold(Row::new().spacing(15).push(Space::with_width(Length::Units(10))).push(Text::new("Primary Button:")), |row, button| {
            row.push(
                Radio::new(*button, &format!("{:?}", button), Some(*primary_button), MouseMessage::PrimaryButtonChanged)
                    .size(15)
                    .spacing(10)
                    .style(if *primary_button == *button { CustomRadio::Active } else { CustomRadio::Disactive }),
            )
        });

        let lb_pointer_speed = Text::new("Pointer Speed:");
        let slider_pointer_speed = Slider::new(pointer_speed_state, 0..=100, *pointer_speed, MouseMessage::PointerSpeedChanged).width(Length::Units(200)).style(CustomSlider::Default);
        let pointer_speed_row = Row::new().spacing(15).align_items(Align::Center).push(Space::with_width(Length::Units(15))).push(lb_pointer_speed).push(slider_pointer_speed);

        let lb_double_click_speed = Text::new("Double-click Speed:");
        let slider_double_click_speed = Slider::new(double_click_speed_state, 0..=100, *double_click_speed, MouseMessage::DoubleClickSpeedChanged)
            .width(Length::Units(200))
            .style(CustomSlider::Default);
        let double_click_speed_row = Row::new().spacing(15).align_items(Align::Center).push(lb_double_click_speed).push(slider_double_click_speed);

        let chb_reverse = Checkbox::new(*reverse_scrolling, "Reverse Scrolling Direction", MouseMessage::ReverseScrollingToggled).spacing(10).style(CustomCheckbox::Default);
        let reverse_row = Row::new().spacing(15).push(Space::with_width(Length::Units(100))).push(chb_reverse);

        let right_pane = Container::new(Column::new().spacing(15).push(rd_primary_button).push(pointer_speed_row).push(double_click_speed_row).push(reverse_row)).width(Length::FillPortion(6));

        // ផ្នែកខាងក្រោម
        let btn_restore_default = Button::new(btn_default_state, Text::new("  Defaults  ")).on_press(MouseMessage::RestoreDefaultClicked).style(CustomButton::Default);
        let bottom = Row::new().spacing(15).align_items(Align::Center).push(btn_restore_default);
        let bottom_section = Container::new(bottom).width(Length::Fill).padding(20);

        // មាតិកា
        let content = Column::new()
            .spacing(15)
            .width(Length::Fill)
            .push(Row::new().height(Length::Fill).spacing(15).padding(20).push(left_pane).push(right_pane))
            .push(bottom_section);

        Container::new(content).padding(20).width(Length::FillPortion(15)).height(Length::Fill).style(CustomContainer::Background).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
pub enum PrimaryButton {
    #[default]
    Left,
    Right,
}

impl PrimaryButton {
    pub const ALL: [PrimaryButton; 2] = [PrimaryButton::Left, PrimaryButton::Right];
}
