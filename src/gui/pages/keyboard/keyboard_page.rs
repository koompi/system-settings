use super::keyboard_utils::*;
use super::add_input_source_sec::AddInputSrcMessage;
use super::conf_input_source_sec::ConfigInputSrcMessage;
use crate::helpers::ROOT_PATH;
use crate::gui::styles::{CustomButton, CustomContainer, CustomSlider, CustomCheckbox, CustomSelect};
use crate::gui::addon_widgets::{icon_btn, tabbar};
use iced::{
   button, Element, Align, Space, Length, Svg, Container, Checkbox, Row, Text, Column, Scrollable, PickList, Slider, Button
};
use iced_custom_widget::Icon;
use vedas_core::svg;

#[derive(Debug, Clone)]
pub enum KeyboardMessage {
   TabChanged(usize),
   KeyRepeatChanged(u8),
   DelayRepeatChanged(u8),
   AdjustBrightnessToggled(bool),
   TurnBacklightOffToggled(bool),
   BacklightOffDurationChanged(TurnBacklightOff),
   LeftTabSelected(usize),
   RightPaneSelectedToggled(usize, bool),
   RestoreDefaultClicked,
   KeyNavToggled(bool),
   InputSourceLeftTabSelected(usize),
   AddClicked,
   RemoveClicked,
   UpClicked,
   DownClicked,
   ConfigClicked,
   ShowInputMenuToggled(bool),
   AutoSwitchToggled(bool),
   AddInputSrcMsg(AddInputSrcMessage),
   ConfigInputSrcMsg(ConfigInputSrcMessage),
   ToggleInpSrcChanged(String),
   ShowPressToggleKey(bool),
   TempSwitchInpSrcChanged(String),
   SwitchInpSrcFWChanged(String),
   SwitchInpSrcBWChanged(String),
   Skip1InpSrc(bool),
   ActInpSrcChanged(String),
   DeactInpSrcChanged(String),
   ActiveByDef(bool),
   ShareInpStateChanged(String),
   SwitchShowInpSrcInfo(bool),
   ChangeFocusShowInpSrcInfo(bool),
   ResetClicked,
   DefaultsClicked,
   OKClicked,
}

#[derive(Debug, Clone, Default)]
pub struct KeyboardPage {
   tabbar_state: Vec<(&'static str, button::State)>,
   current_tab_idx: usize,
   keyboard: Keyboard,
   shortcuts: Shortcuts,
   input_sources_tab: InputSources,
   global_opts: GlobalOptions,
   btn_reset_state: button::State,
   btn_defaults_state: button::State,
   btn_ok_state: button::State,
   is_changed: bool,
}

impl KeyboardPage {
   pub fn new() -> Self {
      Self {
         tabbar_state: vec![
            ("Keyboard", button::State::new()),
            ("Shortcuts", button::State::new()),
            ("Input Sources", button::State::new()),
            ("Global Options", button::State::new()),
         ],
         current_tab_idx: 0,
         keyboard: Keyboard::new(),
         shortcuts: Shortcuts::new(),
         input_sources_tab: InputSources::new(),
         global_opts: GlobalOptions::new(),
         ..Self::default()
      }
   }

   pub fn update(&mut self, msg: KeyboardMessage) {
      use KeyboardMessage::*;
      let KeyboardPage {
         keyboard,
         shortcuts,
         input_sources_tab,
         global_opts: GlobalOptions {
            hotkey_sec,
            behavior_sec,
         },
         ..
      } = self;

      let mut should_apply = Some(true);
      match msg {
         TabChanged(idx) => {self.current_tab_idx = idx; should_apply = None},
         KeyRepeatChanged(val) => keyboard.key_repeat_val = val,
         DelayRepeatChanged(val) => keyboard.delay_repeat_val = val,
         AdjustBrightnessToggled(val) => keyboard.adjust_brightness_low_light = val,
         TurnBacklightOffToggled(val) => keyboard.turn_backlight_off = val,
         BacklightOffDurationChanged(duration) => keyboard.turn_backlight_off_after_val = duration,
         LeftTabSelected(idx) => {shortcuts.left_pane_selected = idx; should_apply = None},
         RightPaneSelectedToggled(idx, is_checked) => {
            shortcuts.right_pane_selected = idx;
            shortcuts.shortcuts_tab_map.get_mut(shortcuts.left_pane_selected).unwrap().get_mut(idx).unwrap().0 = is_checked;
         },
         RestoreDefaultClicked => self.shortcuts = Shortcuts::new(),
         KeyNavToggled(val) => shortcuts.use_keyboard_nav = val,
         InputSourceLeftTabSelected(idx) => {input_sources_tab.input_sources_selected = Some(idx); should_apply = None},
         AddClicked => {input_sources_tab.is_adding = true; should_apply = None},
         RemoveClicked => {
            if let Some(selected_idx) = input_sources_tab.input_sources_selected {
               if input_sources_tab.input_sources.len() > 1 {
                  input_sources_tab.input_sources.remove(selected_idx);
               }
            }
            input_sources_tab.input_sources_selected = None;
         },
         UpClicked => {
            if let Some(selected_idx) = input_sources_tab.input_sources_selected {
               let next_idx = selected_idx - 1;
               input_sources_tab.input_sources.swap(selected_idx, next_idx);
               input_sources_tab.input_sources_selected = Some(next_idx);
            }
         },
         DownClicked => {
            if let Some(selected_idx) = input_sources_tab.input_sources_selected {
               let next_idx = selected_idx + 1;
               input_sources_tab.input_sources.swap(selected_idx, next_idx);
               input_sources_tab.input_sources_selected = Some(next_idx);
            }
         },
         ConfigClicked => input_sources_tab.is_config = true,
         ShowInputMenuToggled(val) => input_sources_tab.show_input_menu = val,
         AutoSwitchToggled(val) => input_sources_tab.auto_switch = val,
         ToggleInpSrcChanged(val) => hotkey_sec.toggle_inp_src_val = Some(val),
         ShowPressToggleKey(is_checked) => hotkey_sec.show_press_toggle_repeat = is_checked,
         TempSwitchInpSrcChanged(val) => hotkey_sec.temp_switch_first_n_cur_inp_src_val = Some(val),
         SwitchInpSrcFWChanged(val) => hotkey_sec.switch_inp_src_fw_val = Some(val),
         SwitchInpSrcBWChanged(val) => hotkey_sec.switch_inp_src_bw_val = Some(val),
         Skip1InpSrc(is_checked) => hotkey_sec.skip_first_inp_src_switch = is_checked,
         ActInpSrcChanged(val) => hotkey_sec.act_inp_src_val = Some(val),
         DeactInpSrcChanged(val) => hotkey_sec.deact_inp_src_val = Some(val),
         ActiveByDef(is_checked) => behavior_sec.act_by_def = is_checked,
         ShareInpStateChanged(val) => behavior_sec.share_inp_state_val = Some(val),
         SwitchShowInpSrcInfo(is_checked) => behavior_sec.switch_show_inp_src_info = is_checked,
         ChangeFocusShowInpSrcInfo(is_checked) => behavior_sec.change_focus_show_inp_src_info = is_checked,
         AddInputSrcMsg(add_inp_msg) => match add_inp_msg {
            AddInputSrcMessage::AddClicked(layout) => {
               input_sources_tab.input_sources.push(('\u{f1ab}', layout, button::State::new())); 
               input_sources_tab.is_adding = false;
            },
            AddInputSrcMessage::CancelClicked => input_sources_tab.is_adding = false,
            _ => input_sources_tab.add_input_source_sec.update(add_inp_msg)
         },
         ConfigInputSrcMsg(conf_inp_src_msg) => match conf_inp_src_msg {
            ConfigInputSrcMessage::AddClicked | ConfigInputSrcMessage::CancelClicked => input_sources_tab.is_config = false,
            _ => input_sources_tab.config_input_source_sec.update(conf_inp_src_msg)
         },
         OKClicked => should_apply = Some(false),
         DefaultsClicked | ResetClicked => {
            let curr_tab = self.current_tab_idx;
            *self = Self::new();
            self.current_tab_idx = curr_tab;
            should_apply = Some(false);
         },
      }
      if let Some(should_apply) = should_apply {
         self.is_changed = should_apply;
      }
   }

   pub fn view(&mut self) -> Element<KeyboardMessage> {
      let KeyboardPage {
         tabbar_state,
         current_tab_idx,
         keyboard,
         shortcuts,
         input_sources_tab,
         global_opts,
         is_changed,
         btn_reset_state,
         btn_defaults_state,
         btn_ok_state,
      } = self;

      // របារផ្ទាំង
      let tabbar_sec = tabbar(tabbar_state, *current_tab_idx, |idx| KeyboardMessage::TabChanged(idx));

      // ទិដ្ឋភាពទូទៅ
      let tabview = match self.current_tab_idx {
         0 => {
            let Keyboard {
               key_repeat_state,
               key_repeat_val,
               delay_repeat_state,
               delay_repeat_val,
               adjust_brightness_low_light,
               turn_backlight_off,
               turn_backlight_off_after_state,
               turn_backlight_off_after_val,
            } = keyboard;

            let lb_key_repeat = Text::new("Key Repeat").size(14);
            let slider_key_repeat = Slider::new(key_repeat_state, 1..=8, *key_repeat_val, KeyboardMessage::KeyRepeatChanged).width(Length::Units(175)).style(CustomSlider::Default);
            let lb_delay_repeat = Text::new("Delay Until Repeat").size(14);
            let slider_delay_repeat = Slider::new(delay_repeat_state, 1..=6, *delay_repeat_val, KeyboardMessage::DelayRepeatChanged).width(Length::Units(175)).style(CustomSlider::Default);
            let key_repeat_row = Row::new().width(Length::Fill).padding(20).spacing(50).align_items(Align::Center)
               .push(
                  Column::new().spacing(15).align_items(Align::Center)
                  .push(lb_key_repeat)
                  .push(
                     Column::new()
                     .push(slider_key_repeat)
                     .push(Row::new().width(Length::Units(175)).spacing(7).push(Text::new("off").size(12)).push(Text::new("slow").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("fast").size(12)))
                  )
               )
               .push(
                  Column::new().spacing(15).align_items(Align::Center)
                  .push(lb_delay_repeat)
                  .push(
                     Column::new()
                     .push(slider_delay_repeat)
                     .push(Row::new().width(Length::Units(175)).push(Text::new("long").size(12)).push(Space::with_width(Length::Fill)).push(Text::new("short").size(12)))
                  )
               );
            let key_repeat_con = Container::new(key_repeat_row).center_x();

            let chk_adjust_brightness = Checkbox::new(*adjust_brightness_low_light, "Adjust keyboard brightness in low light", KeyboardMessage::AdjustBrightnessToggled).spacing(10).style(CustomCheckbox::Default);
            let chk_turn_backlight_off = Checkbox::new(*turn_backlight_off, "Turn keyboard backlight off after", KeyboardMessage::TurnBacklightOffToggled).spacing(10).style(CustomCheckbox::Default);
            let pl_backlight_off_duration = PickList::new(turn_backlight_off_after_state, &TurnBacklightOff::ALL[..], Some(*turn_backlight_off_after_val), KeyboardMessage::BacklightOffDurationChanged).style(CustomSelect::Primary);
            let lb_inactivity = Text::new("of inactivity");
            let keyboard_backligh_off_row = Row::new().spacing(15).align_items(Align::Center)
               .push(chk_turn_backlight_off)
               .push(pl_backlight_off_duration)
               .push(lb_inactivity);

            Container::new(
               Column::new().width(Length::Fill).spacing(20).align_items(Align::Start)
               .push(key_repeat_con)
               .push(
                  Column::new().spacing(15)
                  .push(chk_adjust_brightness)
                  .push(keyboard_backligh_off_row)
               )
            ).width(Length::Fill).height(Length::Fill)
         },
         1 => {
            let Shortcuts {
               shortcuts_tab,
               shortcuts_tab_map,
               left_pane_selected,
               right_pane_selected,
               use_keyboard_nav,
               left_pane_scroll,
               right_pane_scroll,
            } = shortcuts;

            let lb_shortcuts = Text::new("To change a shortcut, select it, click key combination, and then type new keys.").size(15);

            // ផ្ទាំងខាងឆ្វេង
            let left_tab_col = shortcuts_tab.iter_mut().enumerate().fold(Scrollable::new(left_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (icon, title, state))| {
               col.push(icon_btn(state, *icon, title, Some(23)).width(Length::Fill).on_press(KeyboardMessage::LeftTabSelected(idx)).style(if *left_pane_selected == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar}))
            });
            let left_pane = Container::new(left_tab_col).width(Length::FillPortion(4)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្ទាំងខាងស្ដាំ
            let right_pane_col = shortcuts_tab_map.get_mut(*left_pane_selected).unwrap().iter_mut().enumerate().fold(Scrollable::new(right_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (is_checked, title, shortcut))| {
               let row = Row::new().align_items(Align::Center).padding(4)
                  .push(Checkbox::new(*is_checked, *title, move |is| KeyboardMessage::RightPaneSelectedToggled(idx, is)).spacing(10).style(CustomCheckbox::Default))
                  .push(Space::with_width(Length::Fill)).push(Text::new(*shortcut))
                  .push(Space::with_width(Length::Units(15)));

               col.push(Container::new(row).width(Length::Fill).style(if *right_pane_selected == idx {CustomContainer::Hovered} else {CustomContainer::ForegroundWhite}))
            });
            let right_pane = Container::new(right_pane_col).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite);

            // ផ្នែកខាងក្រោម
            let chb_keyboard_nav = Checkbox::new(*use_keyboard_nav, "Use keyboard navigations to move focus between controls", KeyboardMessage::KeyNavToggled).spacing(10).style(CustomCheckbox::Default);
            let txt_hint = Text::new("Press the Tab key to move focus forward and Shift tab to move focus backward.");
            let bottom_col = Column::new().spacing(10).width(Length::Fill)
               .push(Space::with_height(Length::Units(50)))
               .push(chb_keyboard_nav)
               .push(Row::new().push(Space::with_width(Length::Units(30))).push(txt_hint));
            
            Container::new(
               Column::new().spacing(10)
               .push(lb_shortcuts)
               .push(
                  Container::new(
                     Row::new().spacing(15)
                     .push(left_pane)
                     .push(right_pane)
                  ).height(Length::FillPortion(11))
               )
               .push(Container::new(bottom_col).height(Length::FillPortion(5)))
            ).width(Length::Fill).height(Length::Fill)
         },
         2 => {
            let InputSources {
               btn_add_state, 
               btn_remove_state, 
               btn_up_state, 
               btn_down_state, 
               btn_config_state, 
               input_sources,
               input_sources_selected,
               auto_switch,
               show_input_menu,
               is_adding,
               add_input_source_sec,
               is_config,
               config_input_source_sec,
               left_pane_scroll,
               right_pane_scroll,
            } = input_sources_tab;

            let lb_inp_src = Text::new("To change default input source, select and move it to top of the list.").size(15);

            // ផ្ទាំងខាងឆ្វេង
            let inp_src_len = input_sources.len();
            let left_tab_col = input_sources.iter_mut().enumerate().fold(Scrollable::new(left_pane_scroll).height(Length::Fill).padding(7).spacing(4), |col, (idx, (icon, title, state))| {
               let btn = icon_btn(state, *icon, title, Some(23)).width(Length::Fill).on_press(KeyboardMessage::InputSourceLeftTabSelected(idx));
               col.push(
                  if let Some(selected_idx) = input_sources_selected {
                     btn.style(if *selected_idx == idx {CustomButton::SelectedSidebar} else {CustomButton::Sidebar})
                  } else {
                     btn.style(CustomButton::Sidebar)
                  }
               )
            });
            let btn_add = Button::new(btn_add_state, Icon::new('\u{f067}').size(23)).padding(2).on_press(KeyboardMessage::AddClicked).style(CustomButton::Text);
            let mut btn_remove = Button::new(btn_remove_state, Icon::new('\u{f068}').size(23)).padding(2).style(CustomButton::Text);
            let mut btn_up = Button::new(btn_up_state, Icon::new('\u{f062}').size(23)).padding(2).style(CustomButton::Hovered);
            let mut btn_down = Button::new(btn_down_state, Icon::new('\u{f063}').size(23)).padding(2).style(CustomButton::Hovered);
            let mut btn_config = Button::new(btn_config_state, Icon::new('\u{f013}').size(23)).padding(2).style(CustomButton::Hovered);
            if let Some(selected_idx) = input_sources_selected {
               if *selected_idx != 0 {
                  btn_up = btn_up.on_press(KeyboardMessage::UpClicked);
               }
               if *selected_idx != (inp_src_len - 1) {
                  btn_down = btn_down.on_press(KeyboardMessage::DownClicked);
               }
               if !(*is_adding || *is_config) {
                  btn_config = btn_config.on_press(KeyboardMessage::ConfigClicked);
               }
            }
            if input_sources_selected.is_some() && inp_src_len > 1 {
               btn_remove = btn_remove.on_press(KeyboardMessage::RemoveClicked);
            }
            let btn_group = Container::new(Row::new().push(btn_add).push(btn_remove)).width(Length::Fill).style(CustomContainer::Header);
            let btn_shift_group = Container::new(Column::new().spacing(10).push(btn_up).push(btn_down).push(btn_config)).height(Length::Fill).center_y();

            let left_pane = Container::new(
               Row::new().spacing(10).align_items(Align::Center)
               .push(
                  Container::new(
                     Column::new()
                     .push(left_tab_col)
                     .push(btn_group)
                  ).width(Length::Fill).height(Length::Fill).style(CustomContainer::ForegroundWhite)
               )
               .push(btn_shift_group)
            ).width(Length::FillPortion(4)).height(Length::Fill);

            // ផ្ទាំងខាងស្ដាំ
            let right_pane: Element<_> = if *is_config {
               config_input_source_sec.view().map(move |msg| KeyboardMessage::ConfigInputSrcMsg(msg))
            } else if !(*is_adding) {
               let keyboard_image_con = match input_sources_selected {
                  Some(idx) => match idx {
                     0 => {
                        let en_keyboard = svg!(format!("{}/assets/images/keyboard.svg", ROOT_PATH())).height(Length::Units(250));
                        Container::new(
                           Row::new().push(Space::with_width(Length::FillPortion(1))).push(en_keyboard).push(Space::with_width(Length::FillPortion(1)))
                        ).width(Length::Fill).center_x().center_y()
                     },
                     1 => {
                        let kh_keyboard = svg!(format!("{}/assets/images/keyboard.svg", ROOT_PATH())).height(Length::Units(250));
                        Container::new(
                           Row::new().push(Space::with_width(Length::FillPortion(1))).push(kh_keyboard).push(Space::with_width(Length::FillPortion(1)))
                        ).width(Length::Fill).center_x().center_y()
                     },
                     _ => Container::new(Space::with_width(Length::Fill))
                  }
                  None => Container::new(Space::with_width(Length::Fill))
               };
   
               Container::new(
                  Scrollable::new(right_pane_scroll).push(keyboard_image_con)
               ).width(Length::FillPortion(6)).height(Length::Fill).style(CustomContainer::ForegroundWhite).into()
            } else {
               add_input_source_sec.view().map(move |msg| KeyboardMessage::AddInputSrcMsg(msg))
            };

            // ផ្នែកខាងក្រោម
            let chb_show_input_menu = Checkbox::new(*show_input_menu, "Show Input menu in menu bar", KeyboardMessage::ShowInputMenuToggled).spacing(10).style(CustomCheckbox::Default);
            let chb_auto_switch = Checkbox::new(*auto_switch, "Automatically switch to a document's input source", KeyboardMessage::AutoSwitchToggled).spacing(10).style(CustomCheckbox::Default);
            let bottom_right_col = Column::new().spacing(10)
               .push(chb_show_input_menu)
               .push(chb_auto_switch);

            let bottom_row = Row::new().spacing(15).width(Length::Fill)
               .push(Space::with_width(Length::FillPortion(4)))
               .push(Container::new(bottom_right_col).width(Length::FillPortion(6)));
            
            Container::new(
               Column::new().spacing(10)
               .push(lb_inp_src)
               .push(
                  Container::new(
                     Row::new().spacing(15)
                     .push(left_pane)
                     .push(right_pane)
                  ).height(Length::FillPortion(11))
               )
               .push(bottom_row)
            ).width(Length::Fill).height(Length::Fill)
         }, 
         3 => {
            let GlobalOptions {
               hotkey_sec: HotKey {
                  toggle_inp_src_state,
                  toggle_inp_src_val,
                  show_press_toggle_repeat,
                  temp_switch_first_n_cur_inp_src_state,
                  temp_switch_first_n_cur_inp_src_val,
                  switch_inp_src_fw_state,
                  switch_inp_src_fw_val,
                  switch_inp_src_bw_state,
                  switch_inp_src_bw_val,
                  skip_first_inp_src_switch,
                  act_inp_src_state,
                  act_inp_src_val,
                  deact_inp_src_state,
                  deact_inp_src_val,
               },
               behavior_sec: Behavior {
                  act_by_def,
                  share_inp_state,
                  share_inp_state_val,
                  switch_show_inp_src_info,
                  change_focus_show_inp_src_info,
               }
            } = global_opts;

            let sec_lb_hotkey = Text::new("Hotkey").size(15);
            let lb_toggle_inp_src = Text::new("Toggle Input Source:");
            let lb_show_press_toggle_key_repeat = Text::new("Display label when press toggle key repeatedly:");
            let lb_temp_switch_inp_src = Text::new("Temporally switch between first and current input source:");
            let lb_inp_src_fw = Text::new("Switch Input Source Forward:");
            let lb_inp_src_bw = Text::new("Switch Input Source Backward:");
            let lb_skip_fst_inp_while_switch = Text::new("Skip First Input Source While Switching:");
            let lb_act_inp_src = Text::new("Activate Input Source:");
            let lb_deact_inp_src = Text::new("Deactivate Input Source:");
            let ls_hotkeys: Vec<String> = GlobalOptions::hotkey_opts.iter().map(ToString::to_string).collect();
            let select_toggle_inp_src = PickList::new(toggle_inp_src_state, ls_hotkeys.clone(), toggle_inp_src_val.clone(), KeyboardMessage::ToggleInpSrcChanged).style(CustomSelect::Primary);
            let chb_show_press_toggle_key_repeat = Checkbox::new(*show_press_toggle_repeat, "", KeyboardMessage::ShowPressToggleKey).spacing(10).style(CustomCheckbox::Default);
            let select_temp_switch_inp_src = PickList::new(temp_switch_first_n_cur_inp_src_state, ls_hotkeys.clone(), temp_switch_first_n_cur_inp_src_val.clone(), KeyboardMessage::TempSwitchInpSrcChanged).style(CustomSelect::Primary);
            let select_swich_inp_src_fw = PickList::new(switch_inp_src_fw_state, ls_hotkeys.clone(), switch_inp_src_fw_val.clone(), KeyboardMessage::SwitchInpSrcFWChanged).style(CustomSelect::Primary);
            let select_switch_inp_src_bw = PickList::new(switch_inp_src_bw_state, ls_hotkeys.clone(), switch_inp_src_bw_val.clone(), KeyboardMessage::SwitchInpSrcBWChanged).style(CustomSelect::Primary);
            let chb_skip_fst_inp_while_switch = Checkbox::new(*skip_first_inp_src_switch, "", KeyboardMessage::Skip1InpSrc).spacing(10).style(CustomCheckbox::Default);
            let select_act_inp_src = PickList::new(act_inp_src_state, ls_hotkeys.clone(), act_inp_src_val.clone(), KeyboardMessage::ActInpSrcChanged).style(CustomSelect::Primary);
            let select_deact_inp_src = PickList::new(deact_inp_src_state, ls_hotkeys.clone(), deact_inp_src_val.clone(), KeyboardMessage::DeactInpSrcChanged).style(CustomSelect::Primary);
            let hotkey_sec = Row::new().spacing(10)
               .push(
                  Column::new().spacing(15).align_items(Align::End)
                  .push(sec_lb_hotkey)
                  .push(lb_toggle_inp_src)
                  .push(lb_show_press_toggle_key_repeat)
                  .push(lb_temp_switch_inp_src)
                  .push(lb_inp_src_fw)
                  .push(lb_inp_src_bw)
                  .push(lb_skip_fst_inp_while_switch)
                  .push(lb_act_inp_src)
                  .push(lb_deact_inp_src)
               )
               .push(
                  Column::new().spacing(5)
                  .push(Space::with_height(Length::Units(27)))
                  .push(select_toggle_inp_src)
                  .push(chb_show_press_toggle_key_repeat)
                  .push(select_temp_switch_inp_src)
                  .push(select_swich_inp_src_fw)
                  .push(select_switch_inp_src_bw)
                  .push(chb_skip_fst_inp_while_switch)
                  .push(select_act_inp_src)
                  .push(select_deact_inp_src)
               );

            let sec_lb_behavior = Text::new("Behavior").size(15);
            let lb_act_by_def = Text::new("Active By Default:");
            let lb_share_inp_state = Text::new("Share Input State:");
            let lb_switch_show_inp_src_info = Text::new("Show Input Source info when switch:");
            let lb_change_focus_show_inp_src_info = Text::new("Show Input Source info when changing focus:");
            let ls_share_inp_state: Vec<String> = GlobalOptions::share_inp_state_opt.iter().map(ToString::to_string).collect();
            let chb_act_by_def = Checkbox::new(*act_by_def, "", KeyboardMessage::ActiveByDef).spacing(10).style(CustomCheckbox::Default);
            let select_share_inp_state = PickList::new(share_inp_state, ls_share_inp_state.clone(), share_inp_state_val.clone(), KeyboardMessage::ShareInpStateChanged).style(CustomSelect::Primary);
            let chb_switch_show_inp_src_info = Checkbox::new(*switch_show_inp_src_info, "", KeyboardMessage::SwitchShowInpSrcInfo).spacing(10).style(CustomCheckbox::Default);
            let chb_change_focus_show_inp_src_info = Checkbox::new(*change_focus_show_inp_src_info, "", KeyboardMessage::ChangeFocusShowInpSrcInfo).spacing(10).style(CustomCheckbox::Default);
            let behavior_sec = Row::new().spacing(10)
               .push(
                  Column::new().spacing(15).align_items(Align::End)
                  .push(sec_lb_behavior)
                  .push(lb_act_by_def)
                  .push(lb_share_inp_state)
                  .push(lb_switch_show_inp_src_info)
                  .push(lb_change_focus_show_inp_src_info)
               )
               .push(
                  Column::new().spacing(5)
                  .push(Space::with_height(Length::Units(27)))
                  .push(chb_act_by_def)
                  .push(select_share_inp_state)
                  .push(chb_switch_show_inp_src_info)
                  .push(chb_change_focus_show_inp_src_info)
               );

            Container::new(
               Column::new().spacing(15).align_items(Align::Center)
               .push(hotkey_sec)
               .push(Row::new().push(Space::with_width(Length::Units(20))).push(behavior_sec))
            ).width(Length::Fill).center_x()
         },
         _ => Container::new(Space::with_height(Length::Fill))
      };

      // ផ្នែកខាងក្រោម
      let btn_defaults = icon_btn(btn_defaults_state, '\u{f2ea}', "Defaults", None).on_press(KeyboardMessage::DefaultsClicked).style(CustomButton::Default);
      let mut btn_reset = icon_btn(btn_reset_state, '\u{f00d}', "Reset", None).style(CustomButton::Hovered);
      let mut btn_ok = icon_btn(btn_ok_state, '\u{f00c}', "OK", None).style(CustomButton::Primary);
      if *is_changed {
         btn_ok = btn_ok.on_press(KeyboardMessage::OKClicked);
         btn_reset = btn_reset.on_press(KeyboardMessage::ResetClicked);
      }

      let bottom_sec = Container::new(
         Row::new().padding(15).spacing(10).align_items(Align::Center)
         .push(btn_defaults)
         .push(btn_reset)
         .push(Space::with_width(Length::Fill))
         .push(btn_ok),
      )
      .width(Length::Fill)
      .align_x(Align::End);

      // មាតិកា   
      let content = Column::new().width(Length::Fill).align_items(Align::Center)
         .push(tabbar_sec)
         .push(tabview.height(Length::Fill).padding(15).style(CustomContainer::ForegroundGray))
         .push(bottom_sec);

      Container::new(content).width(Length::FillPortion(15)).padding(20).height(Length::Fill).style(CustomContainer::Background).into()
   }
}
