use iced::{
   pick_list, slider, button, scrollable, container, Element, Align, Space, Length, Svg, Color,
   Container, Checkbox, Row, Text, Button, Column, PickList, Slider, Scrollable,
};
use iced_custom_widget::{Icon};
use crate::helpers::ROOT_PATH;

use super::super::styles::{CustomButton, CustomContainer, CustomSlider, CustomCheckbox, CustomSelect};
use smart_default::SmartDefault;
use std::{fmt::{Display, Formatter, Result}, vec};

#[derive(Debug, Clone)]
pub enum AccessMessage {
   SidebarChanged(usize),
   KeyShortcutToggled(bool),
   ScrollGestureToggled(bool),
   ZoomStyleChanged(ZoomStyle),
   HoverTextToggled(bool),
   InvertColorToggled(bool),
   ReduceMotionToggled(bool),
   IncreaseContrastToggled(bool),
   ReduceTransToggled(bool),
   DisplayContrastChanged(u8),
   CursorSizeChanged(u8),
   FlashScreenToggled(bool),
   PlayStereoToggled(bool),
   TestFlashClicked(bool),
   SubStyleChanged(usize),
   BtnAddClicked,
   BtnRemoveClicked,
   ClosedCapToggled(bool),
   StickyKeysToggled(bool),
   ShiftStickyKeyToggled(bool),
   BeepKeyToggled(bool),
   DisplayPressedKeyToggled(bool),
   PressedKeyPosChanged(DisplayKeyPos),
   SlowKeysToggled(bool),
   UseKeySoundToggled(bool),
   AcceptDelayChanged(u8),
   AccessKeyboardToggled(bool),
   InsertRemoveSpaceToggled(bool),
   CapitalSentenceToggled(bool),
   ShortcutToggled(usize, bool),
   ShowStatusToggled(bool),
}

#[derive(Debug, Clone, Default)]
pub struct AccessPage {
   sidebar_state: Vec<(&'static str, &'static str, button::State)>,
   current_sidebar_tab_idx: usize,
   show_status: bool,
   sidebar_scroll: scrollable::State,
   zoom_tab: ZoomTab,
   display_tab: DisplayTab,
   audio_tab: AudioTab,
   caption_tab: CaptionTab,
   keyboard_tab: KeyboardTab,
   shortcut_tab: ShortcutTab,
}

impl AccessPage {
   pub fn new() -> Self {
      Self {
         sidebar_state: vec![
            ("overview", "Overview", button::State::new()),
            ("zoom", "Zoom", button::State::new()),
            ("display", "Display", button::State::new()),
            ("audio", "Audio", button::State::new()),
            ("caption", "Caption", button::State::new()),
            ("keyboard", "Keyboard", button::State::new()),
            ("shortcut", "Shortcut", button::State::new()),
         ],
         current_sidebar_tab_idx: 0,
         show_status: false,
         sidebar_scroll: scrollable::State::new(),
         caption_tab: CaptionTab::new(),
         keyboard_tab: KeyboardTab::new(),
         shortcut_tab: ShortcutTab::new(),
         ..Default::default()
      }
   }

   pub fn update(&mut self, msg: AccessMessage) {
      use AccessMessage::*;
      match msg {
         SidebarChanged(idx) => self.current_sidebar_tab_idx = idx,
         KeyShortcutToggled(is_checked) => self.zoom_tab.use_shortcut_key = is_checked,
         ScrollGestureToggled(is_checked) => self.zoom_tab.use_scroll_gesture = is_checked,
         ZoomStyleChanged(val) => self.zoom_tab.zoom_style_val = val,
         HoverTextToggled(is_checked) => self.zoom_tab.enable_hover_text = is_checked,
         InvertColorToggled(is_checked) => self.display_tab.invert_color = is_checked,
         ReduceMotionToggled(is_checked) => self.display_tab.reduce_motion = is_checked,
         IncreaseContrastToggled(is_checked) => self.display_tab.increase_contrast = is_checked,
         ReduceTransToggled(is_checked) => self.display_tab.reduce_trans = is_checked,
         DisplayContrastChanged(val) => self.display_tab.display_contrast_val = val,
         CursorSizeChanged(val) => self.display_tab.cursor_size_val = val,
         FlashScreenToggled(is_checked) => self.audio_tab.alert_flash_screen = is_checked,
         PlayStereoToggled(is_checked) => self.audio_tab.play_stereo = is_checked,
         TestFlashClicked(is_checked) => self.audio_tab.is_test_flash = is_checked,
         SubStyleChanged(idx) => {
            self.caption_tab.selected_sub_style = idx;
            self.caption_tab.preview_cap_style = self.caption_tab.sub_styles[idx].2;
         },
         BtnAddClicked => self.caption_tab.sub_styles.push(("New Style", button::State::new(), CapStyle::default())),
         BtnRemoveClicked => {
            let _ = self.caption_tab.sub_styles.remove(self.caption_tab.selected_sub_style);
         },
         ClosedCapToggled(is_checked) => self.caption_tab.prefer_closed_cap = is_checked,
         StickyKeysToggled(is_checked) => self.keyboard_tab.enable_sticky_keys = is_checked, 
         ShiftStickyKeyToggled(is_checked) => self.keyboard_tab.shift_toggle_sticky_keys = is_checked,
         BeepKeyToggled(is_checked) => self.keyboard_tab.beep_when_key_set = is_checked, 
         DisplayPressedKeyToggled(is_checked) => self.keyboard_tab.display_pressed_key = is_checked, 
         PressedKeyPosChanged(val) => self.keyboard_tab.display_pressed_key_pos_val = val,
         SlowKeysToggled(is_checked) => self.keyboard_tab.enable_slow_keys = is_checked, 
         UseKeySoundToggled(is_checked) => self.keyboard_tab.use_key_sound = is_checked, 
         AcceptDelayChanged(val) => self.keyboard_tab.accept_delay_val = val,
         AccessKeyboardToggled(is_checked) => self.keyboard_tab.enable_access_keyboard = is_checked, 
         InsertRemoveSpaceToggled(is_checked) => self.keyboard_tab.auto_insert_remove_space = is_checked, 
         CapitalSentenceToggled(is_checked) => self.keyboard_tab.auto_capitalize_sentence = is_checked, 
         ShortcutToggled(idx, is_checked) => self.shortcut_tab.shortcuts_ls[idx].1 = is_checked,
         ShowStatusToggled(is_checked) => self.show_status = is_checked,
      }
   }

   pub fn view(&mut self) -> Element<AccessMessage> {
      let AccessPage {
         sidebar_state,
         current_sidebar_tab_idx,
         show_status,
         sidebar_scroll,
         zoom_tab,
         display_tab,
         audio_tab,
         caption_tab,
         keyboard_tab,
         shortcut_tab,
      } = self;

      // របារចំហៀង
      let sidebar_tabs = sidebar_state.iter_mut().enumerate().fold(Scrollable::new(sidebar_scroll).spacing(4).padding(7), |scroll, (idx, (filename, title, state))| {
         let content = Container::new(
            Row::new().spacing(7).padding(4).align_items(Align::Center)
            .push(Svg::from_path(format!("{}/assets/images/access/{}.svg", ROOT_PATH(), filename)).height(Length::Units(35)))
            .push(Text::new(*title))
         );

         scroll.push(
            Button::new(state, content).width(Length::Fill).on_press(AccessMessage::SidebarChanged(idx)).style(if *current_sidebar_tab_idx == idx {CustomButton::Selected} else {CustomButton::Text})
         )
      });
      let sidebar = Container::new(sidebar_tabs).padding(7).width(Length::FillPortion(3)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

      // ទិដ្ឋភាពទូទៅ
      let tabview = match current_sidebar_tab_idx {
         0 => {
            let icon = Svg::from_path(format!("{}/assets/images/access/overview.svg", ROOT_PATH())).height(Length::Units(75));
            let txt_title = Text::new("Accessibility features adapt your computer to your individual needs.").size(15);
            let txt_desc = Text::new("Your computer can be customized to support your vision, hearing and more.");

            Container::new(
               Row::new()
               .push(Space::with_width(Length::FillPortion(2)))
               .push(
                  Column::new().spacing(15).align_items(Align::Center)
                  .push(icon)
                  .push(txt_title)
                  .push(txt_desc)
               )
               .push(Space::with_width(Length::FillPortion(2)))
            ).width(Length::Fill).height(Length::Fill).center_x().center_y()
         },
         1 => {
            let ZoomTab {
               use_shortcut_key,
               use_scroll_gesture,
               zoom_style_state,
               zoom_style_val,
               advanced_state,
               enable_hover_text,
               opt_state,
            } = zoom_tab;

            let chb_key_shortcut = Checkbox::new(*use_shortcut_key, "Use keyboard shortcuts to zoom", AccessMessage::KeyShortcutToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_scroll_gesture = Checkbox::new(*use_scroll_gesture, "Use scroll gesture with Ctrl key to zoom", AccessMessage::ScrollGestureToggled).spacing(10).style(CustomCheckbox::Default);
            let lb_zoom_style = Text::new("Zoom style:");
            let pl_zoom_style = PickList::new(zoom_style_state, &ZoomStyle::ALL[..], Some(*zoom_style_val), AccessMessage::ZoomStyleChanged).style(CustomSelect::Primary);
            let btn_advanced = Button::new(advanced_state, Text::new("  Advanced  ")).style(CustomButton::Default);
            let btn_opt = Button::new(opt_state, Text::new("  Options  ")).style(CustomButton::Default);
            let chb_hover_text = Checkbox::new(*enable_hover_text, "Enable Hover Text", AccessMessage::HoverTextToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_hover_text_hint = Text::new("Press Ctrl to display a large-text view of the item under pointer.").size(12);

            Container::new(
               Column::new().spacing(5)
               .push(chb_key_shortcut)
               .push(
                  Row::new()
                  .push(Space::with_width(Length::Units(30)))
                  .push(
                     Column::new().spacing(5)
                     .push(Text::new("Zoom in: Ctrl+="))
                     .push(Text::new("Zoom out: Ctrl+-"))
                  )
               )
               .push(chb_scroll_gesture)
               .push(Row::new().spacing(10).align_items(Align::Center).push(lb_zoom_style).push(pl_zoom_style))
               .push(Row::new().push(Space::with_width(Length::Fill)).push(btn_advanced))
               .push(Row::new().push(chb_hover_text).push(Space::with_width(Length::Fill)).push(btn_opt))
               .push(Row::new().push(Space::with_width(Length::Units(30))).push(txt_hover_text_hint))
            ).width(Length::Fill).height(Length::Fill).padding(15)
         },
         2 => {
            let DisplayTab {
               invert_color,
               reduce_motion,
               increase_contrast,
               reduce_trans,
               display_contrast_state,
               display_contrast_val,
               cursor_size_state,
               cursor_size_val,
            } = display_tab;

            let chb_invert_color = Checkbox::new(*invert_color, "Invert colors", AccessMessage::InvertColorToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_reduce_motion = Checkbox::new(*reduce_motion, "Reduce motion", AccessMessage::ReduceMotionToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_inc_contrast = Checkbox::new(*increase_contrast, "Increase contrast", AccessMessage::IncreaseContrastToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_reduce_trans = Checkbox::new(*reduce_trans, "Reduce transparency", AccessMessage::ReduceTransToggled).spacing(10).style(CustomCheckbox::Default);
            let lb_display_contrast = Text::new("Display contrast:");
            let slider_display_contrast = Slider::new(display_contrast_state, 0..=100, *display_contrast_val, AccessMessage::DisplayContrastChanged).width(Length::Fill).style(CustomSlider::Default);
            let display_contrast_row = Row::new().spacing(10)
               .push(lb_display_contrast)
               .push(
                  Column::new().width(Length::Units(250))
                  .push(slider_display_contrast)
                  .push(Row::new().width(Length::Fill).push(Text::new("normal").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("maximum")))
               );
            
            let lb_cursor_size = Text::new("Cursor size:");
            let slider_cursor_size = Slider::new(cursor_size_state, 0..=100, *cursor_size_val, AccessMessage::CursorSizeChanged).width(Length::Fill).style(CustomSlider::Default);
            let cursor_size_row = Row::new().spacing(10)
               .push(lb_cursor_size)
               .push(
                  Column::new().width(Length::Units(250))
                  .push(slider_cursor_size)
                  .push(Row::new().width(Length::Fill).push(Text::new("normal").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("large")))
               );

            Container::new(
               Column::new().spacing(10)
               .push(chb_invert_color)
               .push(chb_reduce_motion)
               .push(chb_inc_contrast)
               .push(chb_reduce_trans)
               .push(display_contrast_row)
               .push(cursor_size_row)
            ).width(Length::Fill).height(Length::Fill).padding(15)
         },
         3 => {
            let AudioTab {
               alert_flash_screen,
               test_flash,
               play_stereo,
               is_test_flash,
            } = audio_tab;

            let chb_alert_flash_screen = Checkbox::new(*alert_flash_screen, "Flash screen when an alert sound occurs", AccessMessage::FlashScreenToggled).spacing(10).style(CustomCheckbox::Default);
            let btn_test_flash = Button::new(test_flash, Text::new("  Test screen flash  ")).on_press(AccessMessage::TestFlashClicked(!(*is_test_flash))).style(CustomButton::Default);
            let chb_play_stereo = Checkbox::new(*play_stereo, "Play stereo audio as mono", AccessMessage::PlayStereoToggled).spacing(10).style(CustomCheckbox::Default);
            
            Container::new(
               Column::new().spacing(10)
               .push(chb_alert_flash_screen)
               .push(Container::new(btn_test_flash).width(Length::Fill).center_x())
               .push(chb_play_stereo)
               .push(Space::with_height(Length::Fill))
               .push(Text::new("System volumn can be adjusted in Sound preferences").size(12))
            ).width(Length::Fill).height(Length::Fill).padding(15)
         },
         4 => {
            let CaptionTab {
               preview_cap_style,
               sub_styles,
               selected_sub_style,
               add_state,
               remove_state,
               prefer_closed_cap,
               scroll,
            } = caption_tab;

            let len = sub_styles.len();
            let preview_caption = Container::new(Text::new("Subtitles will look like this.")).padding(2).style(*preview_cap_style);
            let preview_sec = Container::new(preview_caption).width(Length::Fill).height(Length::Units(70)).center_x().center_y().style(CustomContainer::ForegroundWhite);
            let lb_sub_style = Text::new("Style for subtitles and captions:");
            let cap_styles_view = sub_styles.iter_mut().enumerate().fold(Scrollable::new(scroll).height(Length::Fill).width(Length::Fill).spacing(4).padding(7), |scrollable, (idx, (title, state, _))| {
               let btn = Button::new(state, Text::new(*title)).on_press(AccessMessage::SubStyleChanged(idx)).style(if *selected_sub_style == idx {CustomButton::Selected} else {CustomButton::Text});
               scrollable.push(btn)
            });
            let btn_add = Button::new(add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(AccessMessage::BtnAddClicked).style(CustomButton::Text);
            let mut btn_remove = Button::new(remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
            if len > 1 {
               btn_remove = btn_remove.on_press(AccessMessage::BtnRemoveClicked);
            }
            let cap_styles_sec = Container::new(
               Column::new()
               .push(cap_styles_view)
               .push(
                  Container::new(
                     Row::new().push(btn_add).push(btn_remove).push(Space::with_width(Length::Fill))
                  ).width(Length::Fill).style(CustomContainer::Header)
               )
            );

            let chb_prefer_closed_cap = Checkbox::new(*prefer_closed_cap, "Prefer closed captions", AccessMessage::ClosedCapToggled).spacing(10).style(CustomCheckbox::Default);
            
            Container::new(
               Column::new().spacing(10)
               .push(preview_sec)
               .push(lb_sub_style)
               .push(
                  Container::new(cap_styles_sec).style(CustomContainer::ForegroundWhite)
               )
               .push(chb_prefer_closed_cap)
            ).width(Length::Fill).height(Length::Fill).padding(15)
         },
         5 => {
            let KeyboardTab {
               enable_sticky_keys,
               shift_toggle_sticky_keys,
               beep_when_key_set,
               display_pressed_key,
               display_pressed_key_pos_state,
               display_pressed_key_pos_val,
               enable_slow_keys,
               use_key_sound,
               accept_delay_state,
               accept_delay_val,
               enable_access_keyboard,
               auto_insert_remove_space,
               auto_capitalize_sentence,
            } = keyboard_tab;

            let chb_sticky_keys = Checkbox::new(*enable_sticky_keys, "Enable Sticky Keys", AccessMessage::StickyKeysToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_sticky_keys_hint = Text::new("Sticky Keys allows modifier keys to be set without having to hold the key down.").size(12);   
            let chb_shift_sticky_key = Checkbox::new(*shift_toggle_sticky_keys, "Press the Shift key five times to toggle Sticky Keys", AccessMessage::ShiftStickyKeyToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_beep_when_key_set = Checkbox::new(*beep_when_key_set, "Beep when a modifier key is set", AccessMessage::BeepKeyToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_display_pressed_key = Checkbox::new(*display_pressed_key, "Display pressed keys on screen: ", AccessMessage::DisplayPressedKeyToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_display_pos = PickList::new(display_pressed_key_pos_state, &DisplayKeyPos::ALL[..], Some(*display_pressed_key_pos_val), AccessMessage::PressedKeyPosChanged).style(CustomSelect::Primary);

            let sticky_keys_sec = Container::new(
               Column::new().spacing(10)
               .push(chb_sticky_keys)
               .push(
                  Row::new()
                  .push(Space::with_width(Length::Units(30)))
                  .push(
                     Column::new().spacing(10)
                     .push(txt_sticky_keys_hint)
                     .push(chb_shift_sticky_key)
                     .push(chb_beep_when_key_set)
                     .push(Row::new().spacing(10).align_items(Align::Center).push(chb_display_pressed_key).push(pl_display_pos))
                  )
               )
            );
            
            let chb_slow_keys = Checkbox::new(*enable_slow_keys, "Enable Slow Keys", AccessMessage::SlowKeysToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_slow_keys_hint = Text::new("Slow Keys adjusts the amount of time between when a key is pressed and when it is activated.").size(12);   
            let chb_key_sound = Checkbox::new(*use_key_sound, "Use click key sound", AccessMessage::UseKeySoundToggled).spacing(10).style(CustomCheckbox::Default);
            let slider_accept_delay = Slider::new(accept_delay_state, 0..=100, *accept_delay_val, AccessMessage::AcceptDelayChanged).width(Length::Units(250)).style(CustomSlider::Default);
            let slow_keys_sec = Container::new(
               Column::new().spacing(10)
               .push(chb_slow_keys)
               .push(
                  Row::new()
                  .push(Space::with_width(Length::Units(30)))
                  .push(
                     Column::new().spacing(10)
                     .push(txt_slow_keys_hint)
                     .push(chb_key_sound)
                     .push(
                        Row::new().spacing(10).align_items(Align::Center)
                        .push(Text::new("Acceptance Delay:"))
                        .push(
                           Column::new()
                           .push(slider_accept_delay)
                           .push(Row::new().width(Length::Units(250)).push(Text::new("short").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("long").size(12)))
                        )
                     )
                  )
               )
            );

            let chb_access_keyboard = Checkbox::new(*enable_access_keyboard, "Enable Accessibility Keyboard", AccessMessage::AccessKeyboardToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_access_keyboard_hint = Text::new("The Accessibility Keyboard lets you type and interact with your computer without using a hardware keyboard.").size(12);   
            let chb_auto_insert_remove_space = Checkbox::new(*auto_insert_remove_space, "Automatically insert and remove spaces", AccessMessage::InsertRemoveSpaceToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_auto_capitalize_sentence = Checkbox::new(*auto_capitalize_sentence, "Capitalize sentences automatically", AccessMessage::CapitalSentenceToggled).spacing(10).style(CustomCheckbox::Default);
            let access_keyboard_sec = Container::new(
               Column::new().spacing(10)
               .push(chb_access_keyboard)
               .push(
                  Row::new()
                  .push(Space::with_width(Length::Units(30)))
                  .push(
                     Column::new().spacing(10)
                     .push(txt_access_keyboard_hint)
                     .push(chb_auto_insert_remove_space)
                     .push(chb_auto_capitalize_sentence)
                  )
               )
            );

            Container::new(
               Column::new().spacing(15)
               .push(sticky_keys_sec)
               .push(slow_keys_sec)
               .push(access_keyboard_sec)
            ).width(Length::Fill).height(Length::Fill).padding(15)
         },
         6 => {
            let ShortcutTab {
               shortcuts_ls,
               scroll,
            } = shortcut_tab;

            let lb_shortcuts = Text::new("Accessibility shortcuts:");
            let txt_shortcuts_hint = Text::new("Quickly press Ctrl three times to toggle Accessibility Shortcuts.").size(12);
            let shortcuts_view = shortcuts_ls.iter_mut().enumerate().fold(Scrollable::new(scroll).width(Length::Fill).height(Length::Fill).spacing(7).padding(7), |scrollable, (idx, (title, is_checked))| {
               let checkbox = Checkbox::new(*is_checked, *title, move |is| AccessMessage::ShortcutToggled(idx, is)).spacing(10).style(CustomCheckbox::Default);
               scrollable.push(checkbox)
            });

            Container::new(
               Column::new().spacing(15)
               .push(lb_shortcuts)
               .push(txt_shortcuts_hint)
               .push(Container::new(shortcuts_view).style(CustomContainer::ForegroundWhite))
            ).width(Length::Fill).height(Length::Fill).padding(15)
         },
         _ => Container::new(Space::with_height(Length::Fill))
      };

      // ផ្នែកខាងក្រោម
      let chb_show_status = Checkbox::new(*show_status, "Show Accessibility status in menu bar", AccessMessage::ShowStatusToggled).spacing(10).style(CustomCheckbox::Default);

      // មាតិកា   
      let content = Column::new().spacing(15)
         .push(
            Row::new().spacing(20).width(Length::Fill).height(Length::Fill)
            .push(sidebar)
            .push(Container::new(tabview).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundGray))
         )
         .push(chb_show_status);

      Container::new(content).width(Length::FillPortion(15)).height(Length::Fill).padding(15).style(CustomContainer::Background).into()
   }
}

#[derive(Debug, Clone, Default)]
pub(self) struct ZoomTab {
   use_shortcut_key: bool,
   use_scroll_gesture: bool,
   zoom_style_state: pick_list::State<ZoomStyle>,
   zoom_style_val: ZoomStyle,
   advanced_state: button::State,
   enable_hover_text: bool,
   opt_state: button::State,
}

#[derive(Debug, Clone, Default)]
pub(self) struct DisplayTab {
   invert_color: bool,
   reduce_motion: bool,
   increase_contrast: bool,
   reduce_trans: bool,
   display_contrast_state: slider::State,
   display_contrast_val: u8,
   cursor_size_state: slider::State,
   cursor_size_val: u8,
}

#[derive(Debug, Clone, Default)]
pub(self) struct AudioTab {
   alert_flash_screen: bool,
   test_flash: button::State,
   play_stereo: bool,  
   is_test_flash: bool, 
}

#[derive(Debug, Clone, Default)]
pub(self) struct CaptionTab {
   preview_cap_style: CapStyle,
   sub_styles: Vec<(&'static str, button::State, CapStyle)>,
   selected_sub_style: usize,
   add_state: button::State,
   remove_state: button::State,
   prefer_closed_cap: bool,
   scroll: scrollable::State,
}

#[derive(Debug, Clone, Default)]
pub(self) struct KeyboardTab {
   enable_sticky_keys: bool,
   shift_toggle_sticky_keys: bool,
   beep_when_key_set: bool,
   display_pressed_key: bool,
   display_pressed_key_pos_state: pick_list::State<DisplayKeyPos>,
   display_pressed_key_pos_val: DisplayKeyPos,
   enable_slow_keys: bool,
   use_key_sound: bool,
   accept_delay_state: slider::State,
   accept_delay_val: u8,
   enable_access_keyboard: bool,
   auto_insert_remove_space: bool,
   auto_capitalize_sentence: bool,
}

#[derive(Debug, Clone, Default)]
pub(self) struct ShortcutTab {
   shortcuts_ls: Vec<(&'static str, bool)>,
   scroll: scrollable::State
}

impl CaptionTab {
   pub fn new() -> Self {
      Self {
         sub_styles: vec![
            ("Transparent Background", button::State::new(), CapStyle::Trans),
            ("Classic", button::State::new(), CapStyle::Classic),
            ("Outline Text", button::State::new(), CapStyle::Outline),
         ],
         selected_sub_style: 1,
         ..Default::default()
      }
   }
}

impl KeyboardTab {
   pub fn new() -> Self {
      Self {
         beep_when_key_set: true,
         display_pressed_key: true,
         accept_delay_val: 50,
         ..Default::default()
      }
   }
}

impl ShortcutTab {
   pub fn new() -> Self {
      Self {
         shortcuts_ls: vec![
            ("Zoom", true),
            ("Invert Display Color", true),
            ("Sticky Keys", true),
            ("Slow Keys", true),
            ("Mouse Keys", true),
            ("Accessibility Keyboard", true),
            ("Increase Contrast", false),
            ("Reduce Transparency", false),
         ],
         scroll: scrollable::State::new()
      }
   }
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum ZoomStyle {
   #[default]
   Fullscreen,
   SplitScreen,
}

#[derive(Debug, Clone, Copy, SmartDefault, PartialEq, Eq)]
pub enum DisplayKeyPos {
   #[default]
   TopRight,
   TopLeft,
   BottomRight,
   BottomLeft,
}

impl ZoomStyle {
   const ALL:[ZoomStyle; 2] = [
      ZoomStyle::Fullscreen,
      ZoomStyle::SplitScreen
   ];
}

impl Display for ZoomStyle {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}", match self {
         ZoomStyle::Fullscreen => "Full screen",
         ZoomStyle::SplitScreen => "Split screen"
      })
   }
}

impl DisplayKeyPos {
   const ALL:[DisplayKeyPos; 4] = [
      DisplayKeyPos::TopRight,
      DisplayKeyPos::TopLeft,
      DisplayKeyPos::BottomRight,
      DisplayKeyPos::BottomLeft,
   ];
}

impl Display for DisplayKeyPos {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      use DisplayKeyPos::*;
      write!(f, "{}", match self {
         TopRight => "Top Right",
         TopLeft => "Top Left",
         BottomRight => "Bottom Right",
         BottomLeft => "Bottom Left",
      })
   }
}

#[derive(Debug, Clone, Copy, SmartDefault)]
pub(self) enum CapStyle {
   Trans,
   #[default]
   Classic,
   Outline
}

impl container::StyleSheet for CapStyle {
   fn style(&self) -> container::Style {
      use CapStyle::*;
      container::Style {
         text_color: Some(match self {
            Outline => Color::BLACK,
            _ => Color::WHITE,
         }),
         background: match self {
            Trans => Color {a: 0.3, ..Color::BLACK}.into(),
            Classic => Color::BLACK.into(),
            Outline => Color::TRANSPARENT.into(),
         },
         border_color: Color::TRANSPARENT,
         border_radius: match self {
            Trans => 4.0,
            _ => 0.0
         },
         border_width: 0.0
      }
   }
}