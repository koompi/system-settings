use iced::{
   slider, button,  Element, Align, Space, Length, Svg, Rule, Color,
   Container, Checkbox, Row, Text, Button, Column, Slider,
};
use iced_custom_widget::Icon;
use vedas_core::svg;
use super::super::styles::{CustomButton, CustomContainer, CustomSlider, CustomCheckbox};

#[derive(Debug, Clone)]
pub enum TouchpadMessage {
   TabChanged(usize),
   ClickChanged(u8),
   SpeedChanged(u8),
   LeftTabPointClickSelected(usize),
   PointClickTabChanged(bool),
   LeftTabScrollZoomSelected(usize),
   ScrollZoomTabChanged(bool),
   LeftTabMoreGesturesSelected(usize),
   MoreGesturesTabChanged(bool),
   SetUpBluetoothTouchpad(bool),
}

#[derive(Debug, Clone)]
pub struct TouchpadPage {
   tabbar_state: Vec<(String, button::State)>,
   current_tab_idx: usize,
   point_click: PointClick,
   scroll_zoom: ScrollZoom,
   more_gestures: MoreGestures,
   btn_setup_bt_touchpad: button::State,
   is_setup_bt_touchpad: bool
}

impl TouchpadPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("  Point & Click  ".to_string(), button::State::new()),
            ("  Scroll & Zoom  ".to_string(), button::State::new()),
            ("  More Gestures  ".to_string(), button::State::new()),
         ],
         current_tab_idx: 0,
         point_click: PointClick::new(),
         scroll_zoom: ScrollZoom::new(),
         more_gestures: MoreGestures::new(),
         btn_setup_bt_touchpad: button::State::new(),
         is_setup_bt_touchpad: false,
      }
   }

   pub fn update(&mut self, msg: TouchpadMessage) {
      match msg {
         TouchpadMessage::TabChanged(idx) => self.current_tab_idx = idx,
         TouchpadMessage::ClickChanged(val) => self.point_click.click_val = val,
         TouchpadMessage::SpeedChanged(val) => self.point_click.speed_val = val,
         TouchpadMessage::LeftTabPointClickSelected(idx) => self.point_click.tab_selected = idx,
         TouchpadMessage::PointClickTabChanged(is_checked) => {
            self.point_click.point_click_tab.get_mut(self.point_click.tab_selected).unwrap().0 = is_checked;
         },
         TouchpadMessage::LeftTabScrollZoomSelected(idx) => self.scroll_zoom.tab_selected = idx,
         TouchpadMessage::ScrollZoomTabChanged(is_checked) => {
            self.scroll_zoom.scroll_zoom_tab.get_mut(self.scroll_zoom.tab_selected).unwrap().0 = is_checked;
         },
         TouchpadMessage::LeftTabMoreGesturesSelected(idx) => self.more_gestures.tab_selected = idx,
         TouchpadMessage::MoreGesturesTabChanged(is_checked) => {
            self.more_gestures.more_gestures_tab.get_mut(self.more_gestures.tab_selected).unwrap().0 = is_checked;
         },
         TouchpadMessage::SetUpBluetoothTouchpad(is_clicked) => self.is_setup_bt_touchpad = is_clicked,
      }
   }

   pub fn view(&mut self) -> Element<TouchpadMessage> {
      let TouchpadPage {
         tabbar_state,
         current_tab_idx,
         point_click,
         scroll_zoom,
         more_gestures,
         btn_setup_bt_touchpad,
         is_setup_bt_touchpad,
      } = self;

      // របារផ្ទាំង
      let mut tabbar = Row::new().spacing(2).align_items(Align::Center);
      for (idx, (name, btn_state)) in tabbar_state.iter_mut().enumerate() {
         let mut btn = Button::new(btn_state, Text::new(name.as_str())).padding(5).on_press(TouchpadMessage::TabChanged(idx));
         if *current_tab_idx == idx {
            btn = btn.style(CustomButton::SelectedTab);
         } else {
            btn = btn.style(CustomButton::Tab);
         }
         tabbar = tabbar.push(btn);
      }
      let tabbar_con = Container::new(tabbar).padding(2).center_x().style(CustomContainer::Segment);
      let tabbar_section = Container::new(tabbar_con).padding(7).width(Length::Fill).center_x();

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let PointClick {
               point_click_tab,
               tab_selected,
               click_state,
               click_val,
               speed_state,
               speed_val,
            } = point_click;

            let left_tabs = point_click_tab.iter_mut().enumerate().fold(Column::new().spacing(4), |col, (idx, (is_checked, title, tip, state))| {
               let content = Row::new()
                  .push(
                     Column::new().spacing(3).width(Length::Fill)
                     .push(Checkbox::new(*is_checked, title.as_str(), TouchpadMessage::PointClickTabChanged).spacing(10).style(CustomCheckbox::Default))
                     .push(Row::new().push(Space::with_width(Length::Units(30))).push(Text::new(tip.as_str()).size(12).color(Color::from_rgb8(97, 97, 97))))
                  )
                  .push(Button::new(state, Icon::new('\u{f138}').size(20)).on_press(TouchpadMessage::LeftTabPointClickSelected(idx)).style(CustomButton::Text));
               col.push(
                  Container::new(content).padding(10).style(if *tab_selected == idx {CustomContainer::Hovered} else {CustomContainer::ForegroundGray})
               )
            });

            let right_view = match tab_selected {
               0 => {
                  let look_up = svg!("assets/images/gestures/look_up.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(look_up).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               },
               1 => {
                  let tab_2_fingers = svg!("assets/images/gestures/tap-2.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(tab_2_fingers).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               },
               2 => {
                  let tab_1_finger = svg!("assets/images/gestures/single-tap.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(tab_1_finger).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               _ => Container::new(Space::with_height(Length::Fill))
            };

            let lb_click = Text::new("Click").size(14);
            let slider_click = Slider::new(click_state, 1..=3, *click_val, TouchpadMessage::ClickChanged).width(Length::Units(127)).style(CustomSlider::Default);
            let lb_speed = Text::new("Tracking Speed").size(14);
            let slider_speed = Slider::new(speed_state, 1..=10, *speed_val, TouchpadMessage::SpeedChanged).width(Length::Units(127)).style(CustomSlider::Default);
            let key_repeat_row = Row::new().width(Length::Shrink).spacing(50).align_items(Align::Center)
               .push(
                  Column::new().spacing(10).align_items(Align::Center)
                  .push(lb_click)
                  .push(slider_click)
               )
               .push(
                  Column::new().spacing(10).align_items(Align::Center)
                  .push(lb_speed)
                  .push(slider_speed)
               );
            let key_repeat_con = Container::new(key_repeat_row).width(Length::Fill).center_x();

            Container::new(
               Row::new().width(Length::Fill).spacing(15)
               .push(
                  Container::new(
                     Column::new().spacing(15)
                     .push(left_tabs)
                     .push(Rule::horizontal(1))
                     .push(key_repeat_con)
                  ).width(Length::FillPortion(4))
               )
               .push(
                  Container::new(right_view).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite)
               )
            ).width(Length::Fill).height(Length::Fill)
         },
         1 => {
            let ScrollZoom {
               scroll_zoom_tab,
               tab_selected,
            } = scroll_zoom;

            let left_tabs = scroll_zoom_tab.iter_mut().enumerate().fold(Column::new().height(Length::Fill).spacing(4), |col, (idx, (is_checked, title, tip, state))| {
               let content = Row::new()
                  .push(
                     Column::new().spacing(3).width(Length::Fill)
                     .push(Checkbox::new(*is_checked, title.as_str(), TouchpadMessage::ScrollZoomTabChanged).spacing(10).style(CustomCheckbox::Default))
                     .push(Row::new().push(Space::with_width(Length::Units(30))).push(Text::new(tip.as_str()).size(12).color(Color::from_rgb8(97, 97, 97))))
                  )
                  .push(Button::new(state, Icon::new('\u{f138}').size(20)).on_press(TouchpadMessage::LeftTabScrollZoomSelected(idx)).style(CustomButton::Text));
               col.push(
                  Container::new(content).padding(10).style(if *tab_selected == idx {CustomContainer::Hovered} else {CustomContainer::ForegroundGray})
               )
            });

            let right_view = match tab_selected {
               0 => {
                  let scroll_up = svg!("assets/images/gestures/scroll-up.svg").height(Length::Units(100));
                  let scroll_down = svg!("assets/images/gestures/scroll-down.svg").height(Length::Units(100));
                  Container::new(
                     Column::new().spacing(15)
                     .push(scroll_up)
                     .push(scroll_down)
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               },
               1 => {
                  let zoom_in = svg!("assets/images/gestures/zoom-in.svg").height(Length::Units(100));
                  let zoom_out = svg!("assets/images/gestures/zoom-out.svg").height(Length::Units(100));
                  Container::new(
                     Column::new().spacing(15)
                     .push(zoom_in)
                     .push(zoom_out)
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               },
               2 => {
                  let doubled_tab_2_finger = svg!("assets/images/gestures/tap-2.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(doubled_tab_2_finger).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               3 => {
                  let rotate = svg!("assets/images/gestures/rotate.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(rotate).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               _ => Container::new(Space::with_height(Length::Fill))
            };

            Container::new(
               Row::new().width(Length::Fill).spacing(15)
               .push(
                  Container::new(left_tabs).width(Length::FillPortion(4))
               )
               .push(
                  Container::new(right_view).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite)
               )
            ).width(Length::Fill).height(Length::Fill)
         },
         2 => {
            let MoreGestures {
               more_gestures_tab,
               tab_selected,
            } = more_gestures;

            let left_tabs = more_gestures_tab.iter_mut().enumerate().fold(Column::new().height(Length::Fill).spacing(4), |col, (idx, (is_checked, title, tip, state))| {
               let content = Row::new()
                  .push(
                     Column::new().spacing(3).width(Length::Fill)
                     .push(Checkbox::new(*is_checked, title.as_str(), TouchpadMessage::MoreGesturesTabChanged).spacing(10).style(CustomCheckbox::Default))
                     .push(Row::new().push(Space::with_width(Length::Units(30))).push(Text::new(tip.as_str()).size(12).color(Color::from_rgb8(97, 97, 97))))
                  )
                  .push(Button::new(state, Icon::new('\u{f138}').size(20)).on_press(TouchpadMessage::LeftTabMoreGesturesSelected(idx)).style(CustomButton::Text));
               col.push(
                  Container::new(content).padding(10).style(if *tab_selected == idx {CustomContainer::Hovered} else {CustomContainer::ForegroundGray})
               )
            });

            let right_view = match tab_selected {
               0 => {
                  let swipe_2_fingers = svg!("assets/images/gestures/swipe-2.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(swipe_2_fingers).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               },
               1 => {
                  let tap_3_fingers = svg!("assets/images/gestures/tap-3.svg").height(Length::Units(100));
                  let swipe_fingers = svg!("assets/images/gestures/swipe-2.svg").height(Length::Units(100));
                  Container::new(
                     Column::new().spacing(15)
                     .push(tap_3_fingers)
                     .push(swipe_fingers)
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               },
               2 => {
                  let swipe_left_2_fingers = svg!("assets/images/gestures/swipe-left-2.svg").height(Length::Units(300));
                  Container::new(
                     Row::new().push(Space::with_width(Length::FillPortion(1))).push(swipe_left_2_fingers).push(Space::with_width(Length::FillPortion(1)))
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               3 => {
                  let tap_3_fingers = svg!("assets/images/gestures/tap-3.svg").height(Length::Units(100));
                  let swipe_up = svg!("assets/images/gestures/scroll-up.svg").height(Length::Units(100));
                  Container::new(
                     Column::new().spacing(15)
                     .push(tap_3_fingers)
                     .push(swipe_up)
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               4 => {
                  let tap_4_fingers = svg!("assets/images/gestures/tap-4.svg").height(Length::Units(100));
                  let zoom_out = svg!("assets/images/gestures/zoom-out.svg").height(Length::Units(100));
                  Container::new(
                     Column::new().spacing(15)
                     .push(tap_4_fingers)
                     .push(zoom_out)
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               5 => {
                  let tap_4_fingers = svg!("assets/images/gestures/tap-4.svg").height(Length::Units(100));
                  let zoom_in = svg!("assets/images/gestures/zoom-in.svg").height(Length::Units(100));
                  Container::new(
                     Column::new().spacing(15)
                     .push(tap_4_fingers)
                     .push(zoom_in)
                  ).width(Length::Fill).height(Length::Fill).center_x().center_y()
               }, 
               _ => Container::new(Space::with_height(Length::Fill))
            };

            Container::new(
               Row::new().width(Length::Fill).spacing(15)
               .push(
                  Container::new(left_tabs).width(Length::FillPortion(5))
               )
               .push(
                  Container::new(right_view).width(Length::FillPortion(5)).height(Length::Fill).style(CustomContainer::ForegroundWhite)
               )
            ).width(Length::Fill).height(Length::Fill)
         },
         _ => Container::new(Space::with_height(Length::Fill))
      };

      // ផ្នែកខាងក្រោម
      let bottom_row = Row::new().padding(15).spacing(20).align_items(Align::Center)
         .push(Text::new(if *is_setup_bt_touchpad {"Sorry, It's just UI"} else {""}))
         .push(Button::new(btn_setup_bt_touchpad, Text::new("  Set Up Bluetooth Touchpad...  ")).style(CustomButton::Default).on_press(TouchpadMessage::SetUpBluetoothTouchpad(!self.is_setup_bt_touchpad)));
      let bottom_section = Container::new(bottom_row).width(Length::Fill).align_x(Align::End);

      // មាតិកា   
      let content = Column::new().width(Length::Fill).align_items(Align::Center)
         .push(tabbar_section)
         .push(tabview.height(Length::Fill).padding(15).style(CustomContainer::ForegroundGray))
         .push(bottom_section);

      Container::new(content).width(Length::FillPortion(15)).padding(20).height(Length::Fill).style(CustomContainer::Background).into()
   }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, SmartDefault)]
// pub enum LookUpOptions {
//    #[default]
//    ForceClick1,
//    Tap3,
// }

// impl LookUpOptions {
//    const ALL: [LookUpOptions; 2] = [
//       LookUpOptions::ForceClick1,
//       LookUpOptions::Tap3,
//    ];
// }

// impl std::fmt::Display for LookUpOptions {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//       write!(
//          f,
//          "{}",
//          match self {
//             LookUpOptions::ForceClick1 => "Force Click with one finger",
//             LookUpOptions::Tap3 => "Tab with three fingers", 
//          }
//       )
//    }
// }

// #[derive(Debug, Clone)]
// enum TouchpadConf<T> {
//    Options {
//       state: pick_list::State<T>
//    },
//    Fixed(String)
// }

#[derive(Debug, Clone, Default)]
pub struct PointClick {
   point_click_tab: Vec<(bool, String, String, button::State)>,
   tab_selected: usize,
   click_state: slider::State,
   click_val: u8,
   speed_state: slider::State,
   speed_val: u8,
}

impl PointClick {
   pub fn new() -> Self {
      Self {
         point_click_tab: vec![
            (true, "Look up & data detectors".to_string(), "Force Click with one finger".to_string(), button::State::new()),
            (true, "Secondary click".to_string(), "Click or tap with two fingers".to_string(), button::State::new()),
            (true, "Tap to click".to_string(), "Tap with one finger".to_string(), button::State::new()),
         ],
         tab_selected: 0,
         click_val: 2,
         speed_val: 7,
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct ScrollZoom {
   scroll_zoom_tab: Vec<(bool, String, String, button::State)>,
   tab_selected: usize,
}

impl ScrollZoom {
   pub fn new() -> Self {
      Self {
         scroll_zoom_tab: vec![
            (true, "Scroll direction: Natural".to_string(), "Content tracks finger movement".to_string(), button::State::new()),
            (true, "Zoom in or out".to_string(), "Pinch with two fingers".to_string(), button::State::new()),
            (true, "Smart Zoom".to_string(), "Double-tap with two fingers".to_string(), button::State::new()),
            (true, "Rotate".to_string(), "Rotate with two fingers".to_string(), button::State::new()),
         ],
         ..Default::default()
      }
   }
}

#[derive(Debug, Clone, Default)]
pub struct MoreGestures {
   more_gestures_tab: Vec<(bool, String, String, button::State)>,
   tab_selected: usize,
}

impl MoreGestures {
   pub fn new() -> Self {
      Self {
         more_gestures_tab: vec![
            (true, "Swipe between pages".to_string(), "Scroll left or right with two fingers".to_string(), button::State::new()),
            (false, "Swipe between full-screen apps".to_string(), "Swipe left or right with three fingers".to_string(), button::State::new()),
            (true, "Notification Center".to_string(), "Swipe left from the right edge with two fingers".to_string(), button::State::new()),
            (true, "Workspaces".to_string(), "Swipe up with three fingers".to_string(), button::State::new()),
            (true, "Menu".to_string(), "Pinch with thumb and three fingers".to_string(), button::State::new()),
            (true, "Desktop".to_string(), "Spread with thumb and three fingers".to_string(), button::State::new()),
         ],
         ..Default::default()
      }
   }
}