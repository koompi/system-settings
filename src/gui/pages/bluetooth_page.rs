use iced::{
   button, scrollable, text_input, Align, Button, Checkbox, Column, Container, Element,
   HorizontalAlignment, Length, Row, Rule, Scrollable, Space, Text, TextInput, VerticalAlignment,
};
use iced_custom_widget as icw;
// use iced_native::window::Event;
// use iced_native::Event::Window;
use icw::components::{Icon, Toggler};
use crate::gui::styles::{
   buttons::ButtonStyle, containers::ContainerStyle, rules::RuleStyle, textinput::InputStyle,
};
#[derive(Default, Debug, Clone)]
pub struct BluetoothPage {
   is_enable: bool,
   is_allowed: bool,
   is_shown: bool,
   is_input: bool,
   is_shown_settings: bool,
   edit_dev: button::State,
   show_settings: button::State,
   refresh: button::State,
   device_name: String,
   dev_name: text_input::State,
   dev_name_val: String,
   bluetooth_settings: BluetoothSettings,
   btn_refresh: button::State,
   vector_bluetooths: Vec<(BluetoothDevType, String, BluetoothStatus)>,
   scroll_area: scrollable::State,
}
#[derive(Debug, Clone)]
pub enum BluetoothStatus {
   Connected,
   Connecting,
   NoConnected,
   DisConnected,
}
#[derive(Debug, Clone)]
pub enum BluetoothDevType {
   SmartPhone,
   Computer,
   Headphone,
   Unknown,
}
impl Default for BluetoothDevType {
   fn default() -> Self {
      BluetoothDevType::SmartPhone
   }
}

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
   DevEdited,
   DevEditedVal(String),
   DevEditedSubmmit,
   DevEnabled(bool),
   DevAllowed(bool),
   DevRefreshed,
   DevSettingsShown,
   DevShowNameless(bool),
   CloseApp,
   // Escape,
   BluetoothSettingsMsg(BluetoothSettingsMsg),
   // WindowResize((u32, u32)),
   // FileDrop(std::path::PathBuf),
}

impl BluetoothPage {
   // type Executor = executor::Default;
   // type Message = BluetoothMessage;
   // type Flags = ();

   pub fn new() -> Self {
      let simpler_code = |b_type: BluetoothDevType, b_ssid: &str, b_status: BluetoothStatus| {
         (b_type, b_ssid.to_string(), b_status)
      };
      let mut init_vec_state: Vec<(BluetoothDevType, String, BluetoothStatus)> = Vec::new();
      for _i in 1..=10 {
         init_vec_state.push(simpler_code(
            BluetoothDevType::Computer,
            "Mi Smart Band 5",
            BluetoothStatus::NoConnected,
         ));
      }
      Self {
         vector_bluetooths: init_vec_state,
         is_input: false,
         device_name: "sna-koompi".to_string(),
         bluetooth_settings: BluetoothSettings::new(),
         ..BluetoothPage::default()
      }
   }
   pub fn update(&mut self, message: BluetoothMessage) {
      use BluetoothMessage::*;
      match message {
         DevEnabled(is_enable) => {
            self.is_enable = is_enable;
         }
         DevAllowed(is_allow) => {
            self.is_allowed = is_allow;
         }
         DevShowNameless(data) => {
            self.is_shown = data;
         }
         CloseApp => {
            println!("Applicaiton close:");
         }
         BluetoothSettingsMsg(msg) => {
            self.bluetooth_settings.update(msg);
         }
         DevEditedSubmmit => {
            self.is_input = !self.is_input;
            self.device_name = self.dev_name_val.to_string();
         }
         DevSettingsShown => {
            self.is_shown_settings = !self.is_shown_settings;
         }
         DevEdited => {
            self.is_input = !self.is_input;
         }
         DevRefreshed => {}
         DevEditedVal(val) => {
            self.dev_name_val = val;
         } // WindowResize((w, h)) => {
           //    println!("width: {} & height: {}", w, h);
           //    if w <= 603 {
           //    } else {
           //    }
           // }
           // FileDrop(path) => {
           //    println!("path: {:?}", path.as_path());
           // }

           // Escape => {
           //    println!("Escape key pressed: ");
           //    self.is_shown_settings = !self.is_shown_settings;
           // }
      }
   }
   // fn subscription(&self) -> Subscription<BluetoothMessage> {
   //    iced_native::subscription::events_with(|event, status| {
   //       if let iced_native::event::Status::Captured = status {
   //          return None;
   //       }

   //       match event {
   //          Window(Event::FileDropped(path)) => Some(BluetoothMessage::FileDrop(path)),
   //          Window(Event::Resized { width, height }) => {
   //             Some(BluetoothMessage::WindowResize((width, height)))
   //          }
   //          iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
   //             modifiers,
   //             key_code,
   //          }) => match key_code {
   //             iced::keyboard::KeyCode::W => {
   //                if modifiers.control {
   //                   Some(BluetoothMessage::CloseApp)
   //                } else {
   //                   None
   //                }
   //             }
   //             iced::keyboard::KeyCode::Escape => Some(BluetoothMessage::Escape),
   //             _ => None,
   //          },
   //          _ => None,
   //       }
   //    })
   // }

   pub fn view(&mut self) -> Element<BluetoothMessage> {
      let inner_layout = Container::new(
         Column::new()
            .spacing(10)
            .push(
               Row::new()
                  .push(
                     Row::new()
                        .width(Length::FillPortion(1))
                        .align_items(Align::Center)
                        .spacing(4)
                        .push(Text::new(&self.device_name))
                        .push(if self.is_input {
                           Row::new().push(
                              TextInput::new(
                                 &mut self.dev_name,
                                 "",
                                 &self.dev_name_val,
                                 BluetoothMessage::DevEditedVal,
                              )
                              .on_submit(BluetoothMessage::DevEditedSubmmit)
                              .padding(6)
                              .style(InputStyle::InkBorder),
                           )
                        } else {
                           Row::new().push(
                              Button::new(&mut self.edit_dev, Icon::new('\u{f304}'))
                                 .on_press(BluetoothMessage::DevEdited)
                                 .style(ButtonStyle::Transparent),
                           )
                        }),
                  )
                  .push(
                     Toggler::new(
                        self.is_enable,
                        String::from(""),
                        BluetoothMessage::DevEnabled,
                     )
                     .width(Length::FillPortion(1)),
                  ),
            )
            .push(Rule::horizontal(10).style(RuleStyle {}))
            .push(if self.is_enable {
               Row::new()
                  .push(Text::new(
                     "Allow other Bluetooth devices to find this device",
                  ))
                  .push(Toggler::new(
                     self.is_allowed,
                     String::from(""),
                     BluetoothMessage::DevAllowed,
                  ))
            } else {
               Row::new().push(Text::new(
                  "Enable Bluetooth for devices (Mouse, Keyboard, Headphone)",
               ))
            }),
      )
      .width(Length::Fill)
      .padding(10)
      .style(ContainerStyle::LightGrayCircle);
      let know_devices = Column::new()
         .spacing(10)
         .push(Text::new("My Devices").size(24))
         .push(
            Column::new()
               .padding(10)
               .width(Length::Fill)
               .height(Length::Shrink)
               .push(
                  Row::new()
                     .spacing(6)
                     .push(Icon::new('\u{f10b}'))
                     .push(Text::new("Linux"))
                     .push(Space::with_width(Length::Fill))
                     .push(
                        Row::new()
                           .align_items(Align::Center)
                           .spacing(4)
                           .push(Text::new("Not Connected"))
                           .push(
                              Button::new(&mut self.show_settings, Icon::new('\u{f105}'))
                                 .on_press(BluetoothMessage::DevSettingsShown)
                                 .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                           ),
                     ),
               ),
         );
      let other_devices = Column::new()
         .spacing(10)
         .push(Text::new("Other Devices").size(24))
         .push(
            Row::new()
               .push(Checkbox::new(
                  self.is_shown,
                  "Show Bluetooth devices without names",
                  BluetoothMessage::DevShowNameless,
               ))
               .push(Space::with_width(Length::Fill))
               .push(
                  Button::new(&mut self.btn_refresh, Icon::new('\u{f021}'))
                     .on_press(BluetoothMessage::DevRefreshed)
                     .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
               ),
         )
         .push(self.vector_bluetooths.iter_mut().fold(
            Column::new().padding(10).spacing(16),
            |column, (b_type, b_ssid, b_status)| {
               column.push(
                  Row::new()
                     .align_items(Align::Center)
                     .spacing(4)
                     .push(
                        Icon::new(match b_type {
                           BluetoothDevType::Computer => '\u{f108}',
                           BluetoothDevType::Headphone => '\u{f3cd}',
                           BluetoothDevType::SmartPhone => '\u{f58f}',
                           BluetoothDevType::Unknown => '\u{f17c}',
                        })
                        .size(24),
                     )
                     .push(Text::new(b_ssid.as_str()))
                     .push(Space::with_width(Length::Fill))
                     .push(Text::new(match b_status {
                        BluetoothStatus::Connected => "Connected",
                        BluetoothStatus::Connecting => "Connecting",
                        BluetoothStatus::DisConnected => "Disconnected",
                        BluetoothStatus::NoConnected => "Not connected",
                     })),
               )
            },
         ));
      let scroll_conent = Scrollable::new(&mut self.scroll_area)
         .width(Length::FillPortion(2))
         .height(Length::Fill)
         .push(
            Column::new()
               .spacing(20)
               .push(inner_layout)
               .push(if self.is_enable {
                  Column::new().push(know_devices).push(other_devices)
               } else {
                  Column::new()
               }),
         );
      let embbeded_layout = Row::new()
         .width(Length::Fill)
         .height(Length::Fill)
         .push(
            scroll_conent
               .padding(10)
               .scroller_width(4)
               .scrollbar_width(4),
         )
         .push(if self.is_shown_settings {
            self
               .bluetooth_settings
               .view()
               .map(move |msg| BluetoothMessage::BluetoothSettingsMsg(msg))
         } else {
            Space::with_width(Length::Shrink).into()
         });
      let inner_container = Container::new(embbeded_layout)
         .style(ContainerStyle::White)
         .padding(10);
      Container::new(inner_container)
         .padding(10)
         .width(Length::Fill)
         .height(Length::Fill)
         .style(ContainerStyle::LightGray)
         .into()
   }
}

#[derive(Default, Debug, Clone)]
pub struct BluetoothSettings {
   connected_host: text_input::State,
   connected_host_val: String,
   disconn_btn: button::State,
   ignore_dev: button::State,
   send_file: button::State,
   hide_btn: button::State,
}
#[derive(Debug, Clone)]
pub enum BluetoothSettingsMsg {
   BluetothNameChanged(String),
   Disconnected,
   Ignoranced,
   SendFile,
   HideSettings,
   SubmitChanged,
}

impl BluetoothSettings {
   fn new() -> Self {
      Self {
         connected_host_val: String::from("sna-koompi"),
         ..Default::default()
      }
   }
   fn update(&mut self, msg: BluetoothSettingsMsg) {
      match msg {
         BluetoothSettingsMsg::BluetothNameChanged(val) => {
            self.connected_host_val = val;
         }
         BluetoothSettingsMsg::Disconnected => {}
         BluetoothSettingsMsg::Ignoranced => {}
         BluetoothSettingsMsg::SendFile => {}
         BluetoothSettingsMsg::HideSettings => {}
         BluetoothSettingsMsg::SubmitChanged => {
            println!("data submit");
         }
      }
   }
   fn view(&mut self) -> Element<BluetoothSettingsMsg> {
      let blue_settings_layout = Column::new()
         .spacing(10)
         .padding(10)
         .height(Length::Fill)
         .push(
            Button::new(&mut self.hide_btn, Icon::new('\u{f104}'))
               .on_press(BluetoothSettingsMsg::HideSettings)
               .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
         )
         .push(
            Column::new()
               .align_items(Align::Center)
               .width(Length::Fill)
               .push(Text::new("Connected Host Bluetooth").size(16)),
         )
         .push(
            TextInput::new(
               &mut self.connected_host,
               &self.connected_host_val,
               "",
               BluetoothSettingsMsg::BluetothNameChanged,
            )
            .on_submit(BluetoothSettingsMsg::SubmitChanged)
            .padding(6)
            .style(InputStyle::InkBorder),
         )
         .push(
            Button::new(
               &mut self.disconn_btn,
               Text::new("Disconnect")
                  .width(Length::Fill)
                  .horizontal_alignment(HorizontalAlignment::Center)
                  .vertical_alignment(VerticalAlignment::Center),
            )
            .width(Length::Fill)
            .style(ButtonStyle::Circular(86, 101, 115, 1.0))
            .on_press(BluetoothSettingsMsg::Disconnected),
         )
         .push(
            Button::new(
               &mut self.ignore_dev,
               Text::new("Ignore this device")
                  .width(Length::Fill)
                  .horizontal_alignment(HorizontalAlignment::Center)
                  .vertical_alignment(VerticalAlignment::Center),
            )
            .width(Length::Fill)
            .style(ButtonStyle::Circular(86, 101, 115, 1.0))
            .on_press(BluetoothSettingsMsg::Ignoranced),
         )
         .push(
            Button::new(
               &mut self.send_file,
               Text::new("Send Files")
                  .width(Length::Fill)
                  .horizontal_alignment(HorizontalAlignment::Center)
                  .vertical_alignment(VerticalAlignment::Center),
            )
            .width(Length::Fill)
            .style(ButtonStyle::Circular(86, 101, 115, 1.0))
            .on_press(BluetoothSettingsMsg::SendFile),
         );
      Container::new(blue_settings_layout)
         .center_x()
         .center_y()
         .width(Length::FillPortion(1))
         .style(ContainerStyle::LightGrayCircle)
         .into()
   }
}
